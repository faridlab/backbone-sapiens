//! Authentication application service
//!
//! Orchestrates all authentication workflows: login, register, verify email,
//! password reset, session management, and profile operations.
//!
//! Follows Clean Architecture: handlers delegate here, this service uses
//! repositories for data access and infrastructure services for JWT/email/crypto.

use std::sync::Arc;

use anyhow::Result;
use chrono::{Duration, Utc};
use sqlx::PgPool;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::domain::entity::UserStatus;
use crate::infrastructure::auth::crypto;
use crate::infrastructure::auth::email::AuthEmailService;
use crate::infrastructure::auth::jwt::JwtService;
use crate::infrastructure::persistence::{
    EmailVerificationTokenRepository, PasswordResetTokenRepository, SessionRepository,
    UserRepository,
};

// ── Result Types ────────────────────────────────────────────────────────────

/// Result of a successful login or token refresh.
pub struct LoginResult {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

/// Result of a successful registration.
pub struct RegisterResult {
    pub user_id: Uuid,
    pub email: String,
}

/// Result of a successful email verification.
pub struct VerifyEmailResult {
    pub user_id: Uuid,
}

/// User profile data returned by get_profile / update_profile.
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub roles: Vec<String>,
    pub is_active: bool,
    pub email_verified: bool,
}

/// Input for the register operation.
pub struct RegisterInput {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub first_name: Option<String>,  // Optional, NOT stored in metadata
    pub last_name: Option<String>,   // Optional, NOT stored in metadata
    pub accept_terms: bool,
    pub username: Option<String>,
}

// ── Auth Error ──────────────────────────────────────────────────────────────

/// Auth-specific error type — mapped to HTTP status codes by the presentation layer.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid email or password")]
    InvalidCredentials,

    #[error("Account is temporarily locked. Please try again later.")]
    AccountLocked,

    #[error("Please verify your email before logging in")]
    EmailNotVerified,

    #[error("Account is {0}")]
    AccountInactive(String),

    #[error("{0}")]
    Validation(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Conflict(String),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

// ── Validation Helpers ─────────────────────────────────────────────────────

/// Validate email format using basic RFC 5322 compliant checks.
/// Ensures email has valid structure: local@domain.tld
fn validate_email(email: &str) -> Result<(), AuthError> {
    let email = email.trim();

    // Basic structure checks
    if email.is_empty() {
        return Err(AuthError::Validation("Email is required".into()));
    }

    // Must contain exactly one @ symbol
    let at_count = email.matches('@').count();
    if at_count != 1 {
        return Err(AuthError::Validation("Invalid email format".into()));
    }

    let parts: Vec<&str> = email.split('@').collect();
    let local_part = parts[0];
    let domain_part = parts[1];

    // Local part must not be empty
    if local_part.is_empty() {
        return Err(AuthError::Validation("Invalid email format".into()));
    }

    // Domain must have at least one dot and valid structure
    if !domain_part.contains('.') || domain_part.starts_with('.') || domain_part.ends_with('.') {
        return Err(AuthError::Validation("Invalid email format".into()));
    }

    // Domain parts must not be empty
    if domain_part.split('.').any(|part| part.is_empty()) {
        return Err(AuthError::Validation("Invalid email format".into()));
    }

    // Length check (RFC 5321 limits)
    if email.len() > 254 {
        return Err(AuthError::Validation("Email is too long".into()));
    }

    Ok(())
}

/// Validate new password (both new_password and confirm_password match).
/// Enforces minimum 8 characters with at least one letter and one number.
fn validate_new_password(password: &str, confirm: &str) -> Result<(), AuthError> {
    // Length requirement
    if password.len() < 8 {
        return Err(AuthError::Validation(
            "Password must be at least 8 characters".into(),
        ));
    }

    // Must contain at least one letter
    if !password.chars().any(|c| c.is_alphabetic()) {
        return Err(AuthError::Validation(
            "Password must contain at least one letter".into(),
        ));
    }

    // Must contain at least one digit
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(AuthError::Validation(
            "Password must contain at least one number".into(),
        ));
    }

    // Passwords must match
    if password != confirm {
        return Err(AuthError::Validation("Passwords do not match".into()));
    }

    Ok(())
}

/// Validate registration input
fn validate_registration_input(input: &RegisterInput) -> Result<(), AuthError> {
    validate_email(&input.email)?;
    validate_new_password(&input.password, &input.confirm_password)?;
    if !input.accept_terms {
        return Err(AuthError::Validation(
            "You must accept the terms of service".into(),
        ));
    }
    Ok(())
}

/// Helper to create metadata with current timestamp
fn now_metadata() -> serde_json::Value {
    let now = Utc::now().to_rfc3339();
    serde_json::json!({
        "created_at": now,
        "updated_at": now,
        "deleted_at": null,
        "created_by": null,
        "updated_by": null,
        "deleted_by": null
    })
}

/// Convert user entity + roles + metadata to UserProfile
fn user_to_profile(
    user: &crate::domain::entity::User,
    roles: Vec<String>,
    raw_metadata: &serde_json::Value,
) -> UserProfile {
    let full_name = {
        let first = raw_metadata
            .get("first_name")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let last = raw_metadata
            .get("last_name")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let name = format!("{} {}", first, last).trim().to_string();
        if name.is_empty() { None } else { Some(name) }
    };

    UserProfile {
        id: user.id,
        username: user.username.clone(),
        email: user.email.clone(),
        full_name,
        avatar_url: raw_metadata
            .get("avatar_url")
            .and_then(|v| v.as_str())
            .map(String::from),
        roles,
        is_active: user.status == UserStatus::Active,
        email_verified: user.email_verified,
    }
}

// ── Auth Service ────────────────────────────────────────────────────────────

/// Authentication application service.
pub struct AuthService {
    user_repo: Arc<UserRepository>,
    session_repo: Arc<SessionRepository>,
    email_token_repo: Arc<EmailVerificationTokenRepository>,
    password_token_repo: Arc<PasswordResetTokenRepository>,
    jwt: Arc<JwtService>,
    email: Arc<AuthEmailService>,
    db_pool: PgPool,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<UserRepository>,
        session_repo: Arc<SessionRepository>,
        email_token_repo: Arc<EmailVerificationTokenRepository>,
        password_token_repo: Arc<PasswordResetTokenRepository>,
        jwt: Arc<JwtService>,
        email: Arc<AuthEmailService>,
        db_pool: PgPool,
    ) -> Self {
        Self {
            user_repo,
            session_repo,
            email_token_repo,
            password_token_repo,
            jwt,
            email,
            db_pool,
        }
    }

    // ── Login ───────────────────────────────────────────────────────────────

    pub async fn login(&self, email: &str, password: &str) -> Result<LoginResult, AuthError> {
        let user = self
            .user_repo
            .find_by_email_for_auth(email)
            .await
            .map_err(|e| AuthError::Internal(e))?
            .ok_or(AuthError::InvalidCredentials)?;

        // Check account lock
        if let Some(locked_until) = user.locked_until {
            if locked_until > Utc::now() {
                return Err(AuthError::AccountLocked);
            }
        }

        // Verify password
        let valid = crypto::verify_password(password, &user.password_hash)
            .map_err(|e| AuthError::Internal(e))?;
        if !valid {
            let new_attempts = user.failed_login_attempts + 1;
            let lock_until = if new_attempts >= 5 {
                Some(Utc::now() + Duration::minutes(5))
            } else {
                None
            };
            if let Err(e) = self
                .user_repo
                .increment_failed_attempts(user.id, lock_until)
                .await
            {
                warn!("Failed to increment login attempts for user {}: {}", user.id, e);
            }
            return Err(AuthError::InvalidCredentials);
        }

        // Check email verified
        if !user.email_verified || user.status == UserStatus::PendingVerification {
            return Err(AuthError::EmailNotVerified);
        }

        // Check account active
        if user.status != UserStatus::Active {
            return Err(AuthError::AccountInactive(user.status.to_string()));
        }

        // Record successful login
        if let Err(e) = self.user_repo.record_successful_login(user.id).await {
            warn!("Failed to record successful login for user {}: {}", user.id, e);
        }

        // Create session
        let refresh_token = crypto::generate_refresh_token();
        let refresh_hash = crypto::hash_token(&refresh_token);
        let session_expires = Utc::now() + Duration::days(30);
        let metadata = now_metadata();

        self.session_repo
            .create_auth_session(user.id, &refresh_hash, session_expires, &metadata)
            .await
            .map_err(|e| AuthError::Internal(e))?;

        // Generate JWT
        let roles = self.user_repo.lookup_user_roles(user.id).await;
        let (access_token, expires_in) = self
            .jwt
            .generate_access_token(&user.id.to_string(), &user.email, &roles)
            .map_err(|e| AuthError::Internal(e))?;

        info!("User {} logged in", user.email);

        Ok(LoginResult {
            access_token,
            refresh_token,
            expires_in,
        })
    }

    // ── Register ────────────────────────────────────────────────────────────

    pub async fn register(&self, input: RegisterInput) -> Result<RegisterResult, AuthError> {
        // Validate input using extracted helper
        validate_registration_input(&input)?;

        // Check existing email
        if self
            .user_repo
            .email_exists(&input.email)
            .await
            .unwrap_or(false)
        {
            return Err(AuthError::Conflict("Email already registered".into()));
        }

        // Hash password
        let password_hash =
            crypto::hash_password(&input.password).map_err(|e| AuthError::Internal(e))?;

        // Use full email as username (not just the local part)
        let username = input
            .username
            .unwrap_or_else(|| input.email.clone());

        // Build metadata WITHOUT first_name and last_name
        let metadata = now_metadata();

        // Transaction: user + verification token
        let mut tx = self
            .db_pool
            .begin()
            .await
            .map_err(|e| AuthError::Internal(e.into()))?;

        // Create user using repository method (via transaction)
        let user_id = create_user_in_transaction(
            &mut *tx,
            &username,
            &input.email,
            &password_hash,
            &metadata,
        )
        .await
        .map_err(|e| {
            if e.to_string().contains("Username already taken") {
                AuthError::Conflict("Username already taken".into())
            } else if e.to_string().contains("Email already registered") {
                AuthError::Conflict("Email already registered".into())
            } else {
                error!("Registration DB error: {}", e);
                AuthError::Internal(e)
            }
        })?;

        // Generate OTP
        let otp = crypto::generate_otp();
        let otp_hash = crypto::hash_token(&otp);
        let token_expires = Utc::now() + Duration::minutes(30);
        let token_metadata = now_metadata();

        sqlx::query(
            "INSERT INTO email_verification_tokens \
             (user_id, token, email, token_type, expires_at, status, metadata) \
             VALUES ($1, $2, $3, 'account_creation', $4, 'pending', $5)",
        )
        .bind(user_id)
        .bind(&otp_hash)
        .bind(&input.email)
        .bind(token_expires)
        .bind(&token_metadata)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            error!("Failed to store verification token: {}", e);
            AuthError::Internal(e.into())
        })?;

        tx.commit()
            .await
            .map_err(|e| AuthError::Internal(e.into()))?;

        // Send email (outside transaction)
        self.email
            .send_verification_email(&input.email, &otp)
            .await;

        Ok(RegisterResult {
            user_id,
            email: input.email,
        })
    }

    // ── Verify Email ────────────────────────────────────────────────────────

    pub async fn verify_email(
        &self,
        email: &str,
        otp: &str,
    ) -> Result<VerifyEmailResult, AuthError> {
        let token_row = self
            .email_token_repo
            .find_pending_by_email(email)
            .await
            .map_err(|e| AuthError::Internal(e))?
            .ok_or_else(|| {
                AuthError::NotFound("No pending verification found for this email".into())
            })?;

        // Check expiry
        if token_row.expires_at < Utc::now() {
            if let Err(e) = self
                .email_token_repo
                .update_token_status(token_row.id, "expired")
                .await
            {
                warn!("Failed to mark token {} as expired: {}", token_row.id, e);
            }
            return Err(AuthError::Validation(
                "Verification code has expired. Please request a new one.".into(),
            ));
        }

        // Increment attempts
        let new_attempts = self
            .email_token_repo
            .increment_attempts(token_row.id)
            .await
            .unwrap_or(token_row.attempts + 1);

        if new_attempts > token_row.max_attempts {
            if let Err(e) = self
                .email_token_repo
                .update_token_status(token_row.id, "revoked")
                .await
            {
                warn!("Failed to revoke token {} after max attempts: {}", token_row.id, e);
            }
            return Err(AuthError::Validation(
                "Too many attempts. Please request a new verification code.".into(),
            ));
        }

        // Constant-time compare
        let input_hash = crypto::hash_token(otp.trim());
        if !crypto::constant_time_eq(&input_hash, &token_row.token) {
            return Err(AuthError::Validation("Invalid verification code".into()));
        }

        // Transaction: mark token verified + activate user
        let mut tx = self
            .db_pool
            .begin()
            .await
            .map_err(|e| AuthError::Internal(e.into()))?;

        sqlx::query(
            "UPDATE email_verification_tokens \
             SET status = 'verified', verified_at = NOW() WHERE id = $1",
        )
        .bind(token_row.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AuthError::Internal(e.into()))?;

        sqlx::query("UPDATE users SET status = 'active', email_verified = true WHERE id = $1")
            .bind(token_row.user_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AuthError::Internal(e.into()))?;

        tx.commit()
            .await
            .map_err(|e| AuthError::Internal(e.into()))?;

        info!("Email verified for user {}", token_row.user_id);

        Ok(VerifyEmailResult {
            user_id: token_row.user_id,
        })
    }

    // ── Resend Verification ─────────────────────────────────────────────────

    /// Resend verification email. Always returns Ok to prevent email enumeration.
    /// Uses constant-time processing to prevent timing attacks on email enumeration.
    pub async fn resend_verification(&self, email: &str) -> Result<(), AuthError> {
        // Find user — if not found, still perform constant-time work below
        let user_opt = self.user_repo.find_by_email_for_auth(email).await.ok().flatten();

        // Always perform the same cryptographic operations to prevent timing attacks
        let otp = crypto::generate_otp();
        let otp_hash = crypto::hash_token(&otp);

        // Only proceed if user exists and isn't verified
        let Some(user) = user_opt else {
            // User doesn't exist — silently return (don't reveal)
            return Ok(());
        };

        if user.email_verified {
            return Ok(()); // Already verified — don't reveal
        }

        // Cooldown check: most recent pending token must be older than 2 min
        if let Ok(Some(expires_at)) = self
            .email_token_repo
            .latest_pending_expires_at(user.id)
            .await
        {
            let created_at = expires_at - Duration::minutes(30);
            if Utc::now() - created_at < Duration::minutes(2) {
                return Ok(()); // Cooldown active — don't reveal
            }
        }

        // Revoke old pending tokens
        if let Err(e) = self.email_token_repo.revoke_pending_for_user(user.id).await {
            warn!("Failed to revoke pending tokens for user {}: {}", user.id, e);
        }

        // Store new OTP
        let token_expires = Utc::now() + Duration::minutes(30);
        let metadata = now_metadata();

        if let Err(e) = self
            .email_token_repo
            .create_verification_token(user.id, &otp_hash, email, token_expires, &metadata)
            .await
        {
            error!(
                "Failed to create verification token for user {}: {}",
                user.id, e
            );
            return Err(AuthError::Internal(e));
        }

        self.email.send_verification_email(email, &otp).await;
        Ok(())
    }

    // ── Refresh Token ───────────────────────────────────────────────────────

    pub async fn refresh_token(&self, token: &str) -> Result<LoginResult, AuthError> {
        let token_hash = crypto::hash_token(token);

        let session = self
            .session_repo
            .find_active_by_token_hash(&token_hash)
            .await
            .map_err(|e| AuthError::Internal(e))?
            .ok_or_else(|| AuthError::Validation("Invalid or expired refresh token".into()))?;

        if session.expires_at < Utc::now() {
            if let Err(e) = self.session_repo.revoke_session(session.id).await {
                warn!("Failed to revoke expired session {}: {}", session.id, e);
            }
            return Err(AuthError::Validation("Refresh token expired".into()));
        }

        // Get user
        let user = self
            .user_repo
            .find_by_id(&session.user_id.to_string())
            .await
            .map_err(|e| AuthError::Internal(e))?
            .ok_or_else(|| AuthError::NotFound("User not found".into()))?;

        // Atomic session rotation
        let new_refresh = crypto::generate_refresh_token();
        let new_hash = crypto::hash_token(&new_refresh);
        let new_expires = Utc::now() + Duration::days(30);
        let metadata = now_metadata();

        let mut tx = self
            .db_pool
            .begin()
            .await
            .map_err(|e| AuthError::Internal(e.into()))?;

        sqlx::query("UPDATE sessions SET is_active = false, revoked_at = NOW() WHERE id = $1")
            .bind(session.id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AuthError::Internal(e.into()))?;

        sqlx::query(
            "INSERT INTO sessions (user_id, token_hash, expires_at, device_type, is_active, metadata) \
             VALUES ($1, $2, $3, 'mobile', true, $4)",
        )
        .bind(user.id)
        .bind(&new_hash)
        .bind(new_expires)
        .bind(&metadata)
        .execute(&mut *tx)
        .await
        .map_err(|e| AuthError::Internal(e.into()))?;

        tx.commit()
            .await
            .map_err(|e| AuthError::Internal(e.into()))?;

        // Generate JWT
        let roles = self.user_repo.lookup_user_roles(user.id).await;
        let (access_token, expires_in) = self
            .jwt
            .generate_access_token(&user.id.to_string(), &user.email, &roles)
            .map_err(|e| AuthError::Internal(e))?;

        Ok(LoginResult {
            access_token,
            refresh_token: new_refresh,
            expires_in,
        })
    }

    // ── Logout ──────────────────────────────────────────────────────────────

    pub async fn logout(&self, user_id: Uuid) -> Result<u64, AuthError> {
        let count = self
            .session_repo
            .revoke_all_user_sessions(user_id)
            .await
            .map_err(|e| AuthError::Internal(e))?;

        info!("User {} logged out, {} sessions revoked", user_id, count);
        Ok(count)
    }

    // ── Forgot Password ─────────────────────────────────────────────────────

    /// Send a password reset OTP. Always returns Ok to prevent email enumeration.
    /// Uses constant-time processing to prevent timing attacks on email enumeration.
    pub async fn forgot_password(&self, email: &str) -> Result<(), AuthError> {
        // Always perform the same cryptographic operations to prevent timing attacks
        let otp = crypto::generate_otp();
        let otp_hash = crypto::hash_token(&otp);
        let expires = Utc::now() + Duration::minutes(15);
        let metadata = now_metadata();

        // Find user — if not found, still performed constant-time work above
        let user_opt = self.user_repo.find_by_email_for_auth(email).await.ok().flatten();

        let Some(user) = user_opt else {
            // User doesn't exist — silently return (don't reveal)
            return Ok(());
        };

        // User exists — create the reset token
        if let Err(e) = self
            .password_token_repo
            .create_reset_token(user.id, &otp_hash, expires, &metadata)
            .await
        {
            error!(
                "Failed to create password reset token for user {}: {}",
                user.id, e
            );
        }

        // Email service logs its own success/failure, no need for additional logging here
        self.email.send_password_reset_email(email, &otp).await;
        Ok(())
    }

    // ── Reset Password ──────────────────────────────────────────────────────

    pub async fn reset_password(
        &self,
        token: &str,
        new_password: &str,
        confirm_password: &str,
    ) -> Result<(), AuthError> {
        validate_new_password(new_password, confirm_password)?;

        let token_hash = crypto::hash_token(token.trim());

        let token_row = self
            .password_token_repo
            .find_valid_by_hash(&token_hash)
            .await
            .map_err(|e| AuthError::Internal(e))?
            .ok_or_else(|| AuthError::Validation("Invalid or expired reset code".into()))?;

        if token_row.expires_at < Utc::now() {
            return Err(AuthError::Validation(
                "Reset code has expired. Please request a new one.".into(),
            ));
        }

        let password_hash =
            crypto::hash_password(new_password).map_err(|e| {
                error!("Password hash failed during reset: {}", e);
                AuthError::Internal(e)
            })?;

        // Transaction: update password + mark token used + invalidate sessions
        let mut tx = self
            .db_pool
            .begin()
            .await
            .map_err(|e| AuthError::Internal(e.into()))?;

        sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2")
            .bind(&password_hash)
            .bind(token_row.user_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AuthError::Internal(e.into()))?;

        sqlx::query(
            "UPDATE password_reset_tokens SET is_used = true, used_at = NOW() WHERE id = $1",
        )
        .bind(token_row.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AuthError::Internal(e.into()))?;

        sqlx::query(
            "UPDATE sessions SET is_active = false, revoked_at = NOW() WHERE user_id = $1",
        )
        .bind(token_row.user_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AuthError::Internal(e.into()))?;

        tx.commit()
            .await
            .map_err(|e| AuthError::Internal(e.into()))?;

        info!("Password reset for user {}", token_row.user_id);

        // Send confirmation email
        if let Ok(Some(user)) = self.user_repo.find_by_id(&token_row.user_id.to_string()).await {
            self.email.send_password_changed_email(&user.email).await;
        }

        Ok(())
    }

    // ── Change Password (protected) ─────────────────────────────────────────

    pub async fn change_password(
        &self,
        user_id: Uuid,
        user_email: &str,
        current_password: &str,
        new_password: &str,
        confirm_password: &str,
    ) -> Result<(), AuthError> {
        validate_new_password(new_password, confirm_password)?;

        let user = self
            .user_repo
            .find_by_id(&user_id.to_string())
            .await
            .map_err(|e| AuthError::Internal(e))?
            .ok_or_else(|| AuthError::NotFound("User not found".into()))?;

        let valid = crypto::verify_password(current_password, &user.password_hash)
            .map_err(|e| AuthError::Internal(e))?;
        if !valid {
            return Err(AuthError::Validation(
                "Current password is incorrect".into(),
            ));
        }

        let new_hash = crypto::hash_password(new_password).map_err(|e| AuthError::Internal(e))?;

        // Transaction: update password + invalidate sessions
        let mut tx = self
            .db_pool
            .begin()
            .await
            .map_err(|e| AuthError::Internal(e.into()))?;

        sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2")
            .bind(&new_hash)
            .bind(user_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AuthError::Internal(e.into()))?;

        sqlx::query(
            "UPDATE sessions SET is_active = false, revoked_at = NOW() \
             WHERE user_id = $1 AND is_active = true",
        )
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AuthError::Internal(e.into()))?;

        tx.commit()
            .await
            .map_err(|e| AuthError::Internal(e.into()))?;

        self.email.send_password_changed_email(user_email).await;

        Ok(())
    }

    // ── Get Profile ─────────────────────────────────────────────────────────

    pub async fn get_profile(&self, user_id: Uuid) -> Result<UserProfile, AuthError> {
        let user = self
            .user_repo
            .find_by_id(&user_id.to_string())
            .await
            .map_err(|e| AuthError::Internal(e))?
            .ok_or_else(|| AuthError::NotFound("User not found".into()))?;

        let roles = self.user_repo.lookup_user_roles(user.id).await;
        let raw_meta = self
            .user_repo
            .get_user_metadata_raw(user.id)
            .await
            .map_err(|e| AuthError::Internal(e))?;
        Ok(user_to_profile(&user, roles, &raw_meta))
    }

    // ── Update Profile ──────────────────────────────────────────────────────

    pub async fn update_profile(
        &self,
        user_id: Uuid,
        username: Option<String>,
        full_name: Option<String>,
    ) -> Result<UserProfile, AuthError> {
        // Update username
        if let Some(ref uname) = username {
            self.user_repo
                .update_username(user_id, uname)
                .await
                .map_err(|e| {
                    let msg = e.to_string();
                    if msg.contains("unique") || msg.contains("duplicate") || msg.contains("constraint") {
                        AuthError::Conflict("Username already taken".into())
                    } else {
                        AuthError::Internal(e)
                    }
                })?;
        }

        // Update full_name in metadata
        if let Some(ref name) = full_name {
            let parts: Vec<&str> = name.splitn(2, ' ').collect();
            let first = parts.first().unwrap_or(&"");
            let last = parts.get(1).unwrap_or(&"");
            let patch = serde_json::json!({
                "first_name": first,
                "last_name": last,
                "updated_at": Utc::now().to_rfc3339()
            });
            self.user_repo
                .merge_metadata(user_id, &patch)
                .await
                .map_err(|e| AuthError::Internal(e))?;
        }

        // Re-fetch with raw metadata
        let user = self
            .user_repo
            .find_by_id(&user_id.to_string())
            .await
            .map_err(|e| AuthError::Internal(e))?
            .ok_or_else(|| AuthError::NotFound("User not found".into()))?;
        let roles = self.user_repo.lookup_user_roles(user.id).await;
        let raw_meta = self
            .user_repo
            .get_user_metadata_raw(user.id)
            .await
            .map_err(|e| AuthError::Internal(e))?;
        Ok(user_to_profile(&user, roles, &raw_meta))
    }
}

// ── Private Helpers ───────────────────────────────────────────────────────────

/// Create user within a transaction for atomicity with verification token.
/// This private helper encapsulates the user creation SQL while allowing
/// the caller to manage the transaction lifecycle.
async fn create_user_in_transaction(
    tx: &mut sqlx::PgConnection,
    username: &str,
    email: &str,
    password_hash: &str,
    metadata: &serde_json::Value,
) -> Result<Uuid> {
    let query = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO users (username, email, password_hash, status, email_verified, metadata) \
         VALUES ($1, $2, $3, 'pending_verification', false, $4) \
         RETURNING id",
    )
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .bind(metadata);

    query.fetch_one(&mut *tx).await.map_err(|e| {
        if let Some(db_err) = e.as_database_error() {
            if db_err.constraint().map_or(false, |c| c.contains("username")) {
                return anyhow::anyhow!("Username already taken");
            }
            if db_err.constraint().map_or(false, |c| c.contains("email")) {
                return anyhow::anyhow!("Email already registered");
            }
        }
        anyhow::anyhow!("Failed to create user: {}", e)
    })
}

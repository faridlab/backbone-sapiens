//! User Domain Events
//!
//! Custom domain events for the User aggregate that represent
//! meaningful business occurrences in the authentication and
//! user lifecycle domain.
//!
//! These events are published to the domain EventBus and consumed by:
//! - SapiensLoggingHandler (structured logging)
//! - EmailNotificationHandler (transactional emails)
//! - AuditLoggingHandler (security audit trail)
//! - SapiensIntegrationEventPublisher (cross-module integration)

use backbone_messaging::DomainEvent;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Domain events for the User aggregate.
///
/// Each variant represents a significant business event in the user's lifecycle.
/// Events are published via `EventBus<UserDomainEvent>` and consumed by registered handlers.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum UserDomainEvent {
    // ── Account Lifecycle ──────────────────────────────────────────

    /// A new user account was created (registration or admin-created)
    Created {
        user_id: String,
        occurred_at: DateTime<Utc>,
    },

    /// User account was activated
    Activated {
        user_id: String,
        occurred_at: DateTime<Utc>,
    },

    /// User account was deactivated
    Deactivated {
        user_id: String,
        reason: String,
        occurred_at: DateTime<Utc>,
    },

    /// User account was suspended (policy violation, admin action)
    Suspended {
        user_id: String,
        reason: String,
        occurred_at: DateTime<Utc>,
    },

    /// User account was soft-deleted
    Deleted {
        user_id: String,
        occurred_at: DateTime<Utc>,
    },

    /// User account was restored from deletion
    Restored {
        user_id: String,
        occurred_at: DateTime<Utc>,
    },

    // ── Authentication ─────────────────────────────────────────────

    /// User successfully logged in
    LoggedIn {
        user_id: String,
        session_id: String,
        occurred_at: DateTime<Utc>,
    },

    /// Login attempt failed (bad credentials, locked account, etc.)
    LoginFailed {
        user_id: String,
        reason: String,
        occurred_at: DateTime<Utc>,
    },

    /// User logged out
    LoggedOut {
        user_id: String,
        session_id: String,
        occurred_at: DateTime<Utc>,
    },

    // ── Email & Verification ───────────────────────────────────────

    /// User's email address was verified
    EmailVerified {
        user_id: String,
        occurred_at: DateTime<Utc>,
    },

    /// User's email address was changed
    EmailChanged {
        user_id: String,
        occurred_at: DateTime<Utc>,
    },

    // ── Profile & Password ─────────────────────────────────────────

    /// User's profile was updated
    ProfileUpdated {
        user_id: String,
        occurred_at: DateTime<Utc>,
    },

    /// User's password was changed
    PasswordChanged {
        user_id: String,
        occurred_at: DateTime<Utc>,
    },

    // ── MFA ────────────────────────────────────────────────────────

    /// MFA was enabled for the user's account
    MfaEnabled {
        user_id: String,
        occurred_at: DateTime<Utc>,
    },

    /// MFA was disabled for the user's account
    MfaDisabled {
        user_id: String,
        occurred_at: DateTime<Utc>,
    },

    // ── Account Security ───────────────────────────────────────────

    /// Account was locked (too many failed attempts)
    AccountLocked {
        user_id: String,
        reason: String,
        locked_until: DateTime<Utc>,
        occurred_at: DateTime<Utc>,
    },

    /// Account was unlocked
    AccountUnlocked {
        user_id: String,
        occurred_at: DateTime<Utc>,
    },
}

impl DomainEvent for UserDomainEvent {
    fn event_type(&self) -> &'static str {
        match self {
            Self::Created { .. } => "UserCreated",
            Self::Activated { .. } => "UserActivated",
            Self::Deactivated { .. } => "UserDeactivated",
            Self::Suspended { .. } => "UserSuspended",
            Self::Deleted { .. } => "UserDeleted",
            Self::Restored { .. } => "UserRestored",
            Self::LoggedIn { .. } => "UserLoggedIn",
            Self::LoginFailed { .. } => "UserLoginFailed",
            Self::LoggedOut { .. } => "UserLoggedOut",
            Self::EmailVerified { .. } => "UserEmailVerified",
            Self::EmailChanged { .. } => "UserEmailChanged",
            Self::ProfileUpdated { .. } => "UserProfileUpdated",
            Self::PasswordChanged { .. } => "UserPasswordChanged",
            Self::MfaEnabled { .. } => "UserMfaEnabled",
            Self::MfaDisabled { .. } => "UserMfaDisabled",
            Self::AccountLocked { .. } => "UserAccountLocked",
            Self::AccountUnlocked { .. } => "UserAccountUnlocked",
        }
    }

    fn aggregate_id(&self) -> &str {
        match self {
            Self::Created { user_id, .. }
            | Self::Activated { user_id, .. }
            | Self::Deactivated { user_id, .. }
            | Self::Suspended { user_id, .. }
            | Self::Deleted { user_id, .. }
            | Self::Restored { user_id, .. }
            | Self::LoggedIn { user_id, .. }
            | Self::LoginFailed { user_id, .. }
            | Self::LoggedOut { user_id, .. }
            | Self::EmailVerified { user_id, .. }
            | Self::EmailChanged { user_id, .. }
            | Self::ProfileUpdated { user_id, .. }
            | Self::PasswordChanged { user_id, .. }
            | Self::MfaEnabled { user_id, .. }
            | Self::MfaDisabled { user_id, .. }
            | Self::AccountLocked { user_id, .. }
            | Self::AccountUnlocked { user_id, .. } => user_id,
        }
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        match self {
            Self::Created { occurred_at, .. }
            | Self::Activated { occurred_at, .. }
            | Self::Deactivated { occurred_at, .. }
            | Self::Suspended { occurred_at, .. }
            | Self::Deleted { occurred_at, .. }
            | Self::Restored { occurred_at, .. }
            | Self::LoggedIn { occurred_at, .. }
            | Self::LoginFailed { occurred_at, .. }
            | Self::LoggedOut { occurred_at, .. }
            | Self::EmailVerified { occurred_at, .. }
            | Self::EmailChanged { occurred_at, .. }
            | Self::ProfileUpdated { occurred_at, .. }
            | Self::PasswordChanged { occurred_at, .. }
            | Self::MfaEnabled { occurred_at, .. }
            | Self::MfaDisabled { occurred_at, .. }
            | Self::AccountLocked { occurred_at, .. }
            | Self::AccountUnlocked { occurred_at, .. } => *occurred_at,
        }
    }
}

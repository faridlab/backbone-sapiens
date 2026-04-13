//! Email Service
//!
//! Handles email sending workflows for authentication, verification, and notifications.
//! Integrates with Postman for email delivery with rate limiting and template management.
//!
//! This service manages:
//! - Email verification workflows
//! - Password reset notifications
//! - Account status notifications
//! - Rate limiting for email delivery
//! - Email template management

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;

use crate::domain::value_objects::Email;
// TODO: Add events module when implemented
// use crate::domain::events::user_events::{UserRegisteredEvent, PasswordResetRequestedEvent};

/// Email template types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmailTemplate {
    /// Email verification for new user registration
    EmailVerification { verification_token: String, user_name: String },

    /// Password reset notification
    PasswordReset { reset_token: String, user_name: String },

    /// Account locked notification
    AccountLocked { lockout_minutes: i64, user_name: String },

    /// MFA setup confirmation
    MfaSetupEnabled { backup_codes: Vec<String>, user_name: String },

    /// Suspicious activity alert
    SuspiciousActivity { activity_type: String, location: String, user_name: String },

    /// Account status change
    AccountStatusChange { old_status: String, new_status: String, user_name: String },
}

/// Email delivery result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailDeliveryResult {
    pub success: bool,
    pub message_id: Option<String>,
    pub delivery_status: String,
    pub error_message: Option<String>,
}

/// Email configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: Email,
    pub from_name: String,
    pub rate_limit_per_minute: u32,
    pub template_directory: String,
}

/// Email Service trait
#[async_trait]
pub trait EmailService: Send + Sync {
    /// Send email with template
    async fn send_email(&self, to: &Email, template: EmailTemplate) -> Result<EmailDeliveryResult, EmailError>;

    /// Send verification email
    async fn send_verification_email(&self, to: &Email, verification_token: &str) -> Result<EmailDeliveryResult, EmailError>;

    /// Send password reset email
    async fn send_password_reset_email(&self, to: &Email, reset_token: &str, user_name: &str) -> Result<EmailDeliveryResult, EmailError>;

    /// Send bulk emails with rate limiting
    async fn send_bulk_emails(&self, recipients: Vec<Email>, template: EmailTemplate) -> Result<Vec<EmailDeliveryResult>, EmailError>;

    /// Check email delivery status
    async fn check_delivery_status(&self, message_id: &str) -> Result<EmailDeliveryStatus, EmailError>;

    /// Get email template by name
    async fn get_template(&self, template_name: &str) -> Result<String, EmailError>;
}

/// Email delivery status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailDeliveryStatus {
    pub message_id: String,
    pub status: String,
    pub delivered_at: Option<String>,
    pub opened_at: Option<String>,
    pub clicked_at: Option<String>,
    pub bounced_at: Option<String>,
    pub bounce_reason: Option<String>,
}

/// Email errors
#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("Rate limit exceeded for email: {0}")]
    RateLimitExceeded(String),

    #[error("Invalid email address: {0}")]
    InvalidEmail(String),

    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("SMTP error: {0}")]
    SmtpError(String),

    #[error("Email delivery failed: {0}")]
    DeliveryFailed(String),

    #[error("Email service unavailable")]
    ServiceUnavailable,

    #[error("Email template error: {0}")]
    TemplateError(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}

/// Default implementation of Email Service using Postman Email API
pub struct PostmanEmailService {
    config: EmailConfig,
    rate_limiter: std::sync::Mutex<RateLimiter>,
    templates: HashMap<String, String>,
}

impl PostmanEmailService {
    pub fn new(config: EmailConfig) -> Self {
        let rate_limit = config.rate_limit_per_minute;
        Self {
            config,
            rate_limiter: std::sync::Mutex::new(RateLimiter::new(rate_limit)),
            templates: HashMap::new(),
        }
    }

    /// Load email templates from filesystem
    pub async fn load_templates(&mut self) -> Result<(), EmailError> {
        // TODO: Load templates from config.template_directory
        // For now, use built-in templates

        self.templates.insert("email_verification".to_string(), self.get_default_verification_template());
        self.templates.insert("password_reset".to_string(), self.get_default_password_reset_template());
        self.templates.insert("account_locked".to_string(), self.get_default_account_locked_template());

        Ok(())
    }

    /// Generate verification email template
    fn get_default_verification_template(&self) -> String {
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Verify Your Email Address</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .container { max-width: 600px; margin: 0 auto; }
        .button {
            background-color: #007bff;
            color: white;
            padding: 12px 24px;
            text-decoration: none;
            border-radius: 4px;
            display: inline-block;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>Verify Your Email Address</h1>
        <p>Thank you for registering! Please click the button below to verify your email address:</p>
        <p><a href="{{VERIFICATION_URL}}" class="button">Verify Email</a></p>
        <p>Or copy and paste this link into your browser:</p>
        <p>{{VERIFICATION_URL}}</p>
        <p>This link will expire in 24 hours.</p>
        <p>If you didn't create an account, please ignore this email.</p>
    </div>
</body>
</html>
        "#.to_string()
    }

    /// Generate password reset template
    fn get_default_password_reset_template(&self) -> String {
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Reset Your Password</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .container { max-width: 600px; margin: 0 auto; }
        .button {
            background-color: #dc3545;
            color: white;
            padding: 12px 24px;
            text-decoration: none;
            border-radius: 4px;
            display: inline-block;
        }
        .warning {
            background-color: #fff3cd;
            border: 1px solid #ffeaa7;
            padding: 10px;
            border-radius: 4px;
            margin: 20px 0;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>Reset Your Password</h1>
        <p>We received a request to reset your password. Click the button below to proceed:</p>
        <p><a href="{{RESET_URL}}" class="button">Reset Password</a></p>
        <p>Or copy and paste this link into your browser:</p>
        <p>{{RESET_URL}}</p>

        <div class="warning">
            <strong>Security Notice:</strong> This link will expire in 1 hour for your security.
            If you didn't request a password reset, please ignore this email and contact support.
        </div>
    </div>
</body>
</html>
        "#.to_string()
    }

    /// Generate account locked template
    fn get_default_account_locked_template(&self) -> String {
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Account Security Alert</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .container { max-width: 600px; margin: 0 auto; }
        .alert {
            background-color: #f8d7da;
            border: 1px solid #f5c6cb;
            padding: 15px;
            border-radius: 4px;
            margin: 20px 0;
        }
        .info {
            background-color: #d1ecf1;
            border: 1px solid #bee5eb;
            padding: 15px;
            border-radius: 4px;
            margin: 20px 0;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🔒 Account Security Alert</h1>

        <div class="alert">
            <strong>Your account has been temporarily locked</strong> due to multiple failed login attempts.
        </div>

        <p>Your account will be automatically unlocked in <strong>{{LOCKOUT_MINUTES}} minutes</strong>.</p>

        <div class="info">
            <strong>What happened?</strong><br>
            We detected {{FAILED_ATTEMPTS}} unsuccessful login attempts to your account from different devices or locations.
        </div>

        <div class="info">
            <strong>What should you do?</strong><br>
            1. Wait for the lockout period to expire<br>
            2. Ensure you're using the correct password<br>
            3. Contact support if you suspect unauthorized access
        </div>

        <p>If this wasn't you, please secure your account immediately by:</p>
        <ul>
            <li>Changing your password</li>
            <li>Enabling two-factor authentication</li>
            <li>Reviewing your recent account activity</li>
        </ul>
    </div>
</body>
</html>
        "#.to_string()
    }

    /// Process template with variables
    fn process_template(&self, template: &str, variables: &HashMap<String, String>) -> String {
        let mut processed = template.to_string();

        for (key, value) in variables {
            let placeholder = format!("{{{{{}}}}}", key);
            processed = processed.replace(&placeholder, value);
        }

        processed
    }

    /// Check rate limit before sending
    async fn check_rate_limit(&self, email: &Email) -> Result<(), EmailError> {
        if !self.rate_limiter.lock().unwrap().check_limit(email.as_str()) {
            return Err(EmailError::RateLimitExceeded(email.as_str().to_string()));
        }
        Ok(())
    }

    /// Send email via Postman API (placeholder implementation)
    async fn send_via_postman(&self, to: &Email, subject: &str, html_body: &str) -> Result<String, EmailError> {
        // TODO: Integrate with actual Postman Email API
        // For now, simulate successful sending
        let message_id = format!("msg_{}", Uuid::new_v4());

        // Simulate API call delay
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        Ok(message_id)
    }
}

// <<< CUSTOM CODE START >>>
// Fixed: Default implementation with proper types (String not Option, Email not String)
impl Default for PostmanEmailService {
    fn default() -> Self {
        Self::new(EmailConfig {
            smtp_host: "localhost".to_string(),
            smtp_port: 587,
            smtp_username: String::new(),
            smtp_password: String::new(),
            from_email: Email::new("noreply@example.com").unwrap_or_else(|_| Email::new("system@localhost").unwrap()),
            from_name: "System".to_string(),
            rate_limit_per_minute: 10,
            template_directory: String::new(),
        })
    }
}
// <<< CUSTOM CODE END >>>

#[async_trait]
impl EmailService for PostmanEmailService {
    async fn send_email(&self, to: &Email, template: EmailTemplate) -> Result<EmailDeliveryResult, EmailError> {
        // Check rate limit
        self.check_rate_limit(to).await?;

        // Generate email content based on template
        let (subject, html_body) = match template {
            EmailTemplate::EmailVerification { verification_token, user_name } => {
                let template_str = self.templates.get("email_verification")
                    .ok_or_else(|| EmailError::TemplateNotFound("email_verification".to_string()))?;

                let variables = HashMap::from([
                    ("VERIFICATION_URL".to_string(),
                     format!("https://yourapp.com/verify-email?token={}", verification_token)),
                    ("USER_NAME".to_string(), user_name),
                ]);

                let processed = self.process_template(template_str, &variables);

                ("Verify Your Email Address".to_string(), processed)
            },

            EmailTemplate::PasswordReset { reset_token, user_name } => {
                let template_str = self.templates.get("password_reset")
                    .ok_or_else(|| EmailError::TemplateNotFound("password_reset".to_string()))?;

                let variables = HashMap::from([
                    ("RESET_URL".to_string(),
                     format!("https://yourapp.com/reset-password?token={}", reset_token)),
                    ("USER_NAME".to_string(), user_name),
                ]);

                let processed = self.process_template(template_str, &variables);

                ("Reset Your Password".to_string(), processed)
            },

            EmailTemplate::AccountLocked { lockout_minutes, user_name } => {
                let template_str = self.templates.get("account_locked")
                    .ok_or_else(|| EmailError::TemplateNotFound("account_locked".to_string()))?;

                let variables = HashMap::from([
                    ("LOCKOUT_MINUTES".to_string(), lockout_minutes.to_string()),
                    ("USER_NAME".to_string(), user_name),
                    ("FAILED_ATTEMPTS".to_string(), "5".to_string()),
                ]);

                let processed = self.process_template(template_str, &variables);

                ("Account Security Alert".to_string(), processed)
            },

            EmailTemplate::MfaSetupEnabled { backup_codes, user_name } => {
                let backup_codes_list = backup_codes.join(", ");
                (format!("MFA Setup Completed for {}", user_name),
                 format!("Your backup codes: {}", backup_codes_list))
            },

            EmailTemplate::SuspiciousActivity { activity_type, location, user_name } => {
                (format!("Suspicious Activity Alert for {}", user_name),
                 format!("Suspicious activity detected: {} from {}", activity_type, location))
            },

            EmailTemplate::AccountStatusChange { old_status, new_status, user_name } => {
                (format!("Account Status Changed for {}", user_name),
                 format!("Your account status changed from {} to {}", old_status, new_status))
            },
        };

        // Send email via Postman
        let message_id = self.send_via_postman(to, &subject, &html_body).await?;

        Ok(EmailDeliveryResult {
            success: true,
            message_id: Some(message_id),
            delivery_status: "sent".to_string(),
            error_message: None,
        })
    }

    async fn send_verification_email(&self, to: &Email, verification_token: &str) -> Result<EmailDeliveryResult, EmailError> {
        // Use a generic user name for verification email
        let template = EmailTemplate::EmailVerification {
            verification_token: verification_token.to_string(),
            user_name: "User".to_string(),
        };

        self.send_email(to, template).await
    }

    async fn send_password_reset_email(&self, to: &Email, reset_token: &str, user_name: &str) -> Result<EmailDeliveryResult, EmailError> {
        let template = EmailTemplate::PasswordReset {
            reset_token: reset_token.to_string(),
            user_name: user_name.to_string(),
        };

        self.send_email(to, template).await
    }

    async fn send_bulk_emails(&self, recipients: Vec<Email>, template: EmailTemplate) -> Result<Vec<EmailDeliveryResult>, EmailError> {
        let mut results = Vec::new();

        for recipient in recipients {
            match self.send_email(&recipient, template.clone()).await {
                Ok(result) => results.push(result),
                Err(error) => {
                    results.push(EmailDeliveryResult {
                        success: false,
                        message_id: None,
                        delivery_status: "failed".to_string(),
                        error_message: Some(error.to_string()),
                    });
                }
            }

            // Add small delay between bulk sends to avoid rate limiting
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }

        Ok(results)
    }

    async fn check_delivery_status(&self, message_id: &str) -> Result<EmailDeliveryStatus, EmailError> {
        // TODO: Integrate with Postman delivery status API
        // For now, return a simulated status
        Ok(EmailDeliveryStatus {
            message_id: message_id.to_string(),
            status: "delivered".to_string(),
            delivered_at: Some(Utc::now().to_rfc3339()),
            opened_at: None,
            clicked_at: None,
            bounced_at: None,
            bounce_reason: None,
        })
    }

    async fn get_template(&self, template_name: &str) -> Result<String, EmailError> {
        self.templates.get(template_name)
            .cloned()
            .ok_or_else(|| EmailError::TemplateNotFound(template_name.to_string()))
    }
}

// Implement the password_reset_service::EmailService trait for PostmanEmailService
// This allows PostmanEmailService to be used by PasswordResetServiceImpl
// <<< CUSTOM CODE START >>>
// NOTE: password_reset_service::EmailService trait doesn't exist yet
// Uncomment when the trait is added to password_reset_service
/*
#[async_trait::async_trait]
impl crate::domain::services::password_reset_service::EmailService for PostmanEmailService {
    async fn send_password_reset_email(&self, to: &str, token: &str, user_name: &str) -> Result<(), crate::domain::services::EmailError> {
        use crate::domain::value_objects::Email;

        // Convert string email to Email value object
        let recipient = Email::new(to)
            .map_err(|e| EmailError::InvalidEmail(format!("Invalid email address: {}", e)))?;

        // Call the main EmailService trait method to avoid ambiguity
        match <Self as crate::domain::services::EmailService>::send_password_reset_email(self, &recipient, token, user_name).await {
            Ok(_) => Ok(()),
            Err(e) => Err(EmailError::DeliveryFailed(e.to_string())),
        }
    }
}
*/
// <<< CUSTOM CODE END >>>

/// Simple rate limiter for email sending
#[derive(Debug)]
struct RateLimiter {
    requests_per_minute: u32,
    sent_emails: HashMap<String, Vec<std::time::Instant>>,
}

impl RateLimiter {
    fn new(requests_per_minute: u32) -> Self {
        Self {
            requests_per_minute,
            sent_emails: HashMap::new(),
        }
    }

    fn check_limit(&mut self, email: &str) -> bool {
        let now = std::time::Instant::now();
        let one_minute_ago = now - std::time::Duration::from_secs(60);

        // Clean up old entries
        let emails = self.sent_emails.entry(email.to_string()).or_insert_with(Vec::new);
        emails.retain(|&timestamp| timestamp > one_minute_ago);

        // Check if under limit
        if emails.len() < self.requests_per_minute as usize {
            emails.push(now);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(2); // 2 emails per minute

        assert!(limiter.check_limit("test@example.com"));
        assert!(limiter.check_limit("test@example.com"));
        assert!(!limiter.check_limit("test@example.com")); // Should be rate limited

        // Different email should work
        assert!(limiter.check_limit("other@example.com"));
    }

    #[test]
    fn test_template_processing() {
        let service = create_test_service();

        let template = "Hello {{USER_NAME}}, your verification link is {{VERIFICATION_URL}}";
        let mut variables = HashMap::new();
        variables.insert("USER_NAME".to_string(), "John".to_string());
        variables.insert("VERIFICATION_URL".to_string(), "https://example.com".to_string());

        let result = service.process_template(template, &variables);
        assert!(result.contains("Hello John"));
        assert!(result.contains("https://example.com"));
    }

    fn create_test_service() -> PostmanEmailService {
        let config = EmailConfig {
            smtp_host: "localhost".to_string(),
            smtp_port: 587,
            smtp_username: "test".to_string(),
            smtp_password: "test".to_string(),
            from_email: Email::new("noreply@example.com").unwrap(),
            from_name: "Test App".to_string(),
            rate_limit_per_minute: 10,
            template_directory: "/tmp/templates".to_string(),
        };

        PostmanEmailService::new(config)
    }
}
//! SMTP Email Service Adapter for Sapiens
//!
//! Bridges between Sapiens EmailService trait and backbone-email crate's SMTP implementation.
//! Provides real email delivery via SMTP using lettre library.

use async_trait::async_trait;
use std::sync::Arc;

use backbone_email::{
    EmailService as BackboneEmailService,
    EmailMessage, EmailAddress,
    SmtpEmailService, SmtpConfig as BackboneSmtpConfig,
};

use crate::domain::value_objects::Email;
use crate::domain::services::email_service::{
    EmailService, EmailDeliveryResult, EmailError, EmailTemplate,
    EmailDeliveryStatus,
};

/// SMTP configuration for Sapiens
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmtpConfig {
    /// SMTP server hostname
    pub host: String,

    /// SMTP server port
    pub port: u16,

    /// Username for authentication
    pub username: Option<String>,

    /// Password for authentication
    pub password: Option<String>,

    /// From email address
    pub from_email: String,

    /// From name
    pub from_name: String,

    /// Use TLS/SSL
    pub use_tls: bool,

    /// Connection timeout in seconds
    pub timeout: u64,
}

impl SmtpConfig {
    /// Load from environment variables
    pub fn from_env() -> Result<Self, String> {
        let host = std::env::var("SMTP_HOST")
            .unwrap_or_else(|_| "smtp.hostinger.com".to_string());

        let port = std::env::var("SMTP_PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(465);

        let username = std::env::var("SMTP_USER").ok();
        let password = std::env::var("SMTP_PASSWORD").ok();

        let from_email = std::env::var("EMAIL_FROM")
            .unwrap_or_else(|_| "noreply@example.com".to_string());

        let from_name = std::env::var("EMAIL_FROM_NAME")
            .unwrap_or_else(|_| "Bersihir".to_string());

        let use_tls = std::env::var("SMTP_SECURE")
            .ok()
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(true);

        Ok(Self {
            host,
            port,
            username,
            password,
            from_email,
            from_name,
            use_tls,
            timeout: 30,
        })
    }

    /// Convert to backbone-email SmtpConfig
    pub fn to_backbone_config(&self) -> BackboneSmtpConfig {
        let mut config = BackboneSmtpConfig::default();
        config.host = self.host.clone();
        config.port = self.port;
        config.username = self.username.clone();
        config.password = self.password.clone();
        config.use_tls = self.use_tls;
        config.use_ssl = self.port == 465;
        config.timeout = self.timeout;
        config
    }
}

/// SMTP Email Service Adapter
///
/// Wraps backbone-email's SmtpEmailService and implements Sapiens EmailService trait.
pub struct SmtpEmailServiceAdapter {
    smtp_service: Arc<SmtpEmailService>,
    from_email: String,
    from_name: String,
}

impl SmtpEmailServiceAdapter {
    /// Create new SMTP email service adapter
    pub async fn new(config: SmtpConfig) -> Result<Self, String> {
        let backbone_config = config.to_backbone_config();

        let smtp_service = SmtpEmailService::new(backbone_config)
            .map_err(|e| format!("Failed to create SMTP service: {:?}", e))?;

        let from_email = config.from_email.clone();
        let from_name = config.from_name.clone();

        Ok(Self {
            smtp_service: Arc::new(smtp_service),
            from_email,
            from_name,
        })
    }

    /// Create from environment variables
    pub async fn from_env() -> Result<Self, String> {
        let config = SmtpConfig::from_env()?;
        Self::new(config).await
    }

    /// Generate HTML email template for verification code
    fn generate_verification_template(&self, verification_token: &str, user_name: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Verify Your Email Address</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; background-color: #f5f5f5; }}
        .container {{ max-width: 600px; margin: 0 auto; background-color: white; padding: 30px; border-radius: 8px; }}
        .code {{
            background-color: #f0f0f0;
            border: 2px dashed #007bff;
            padding: 15px;
            font-size: 24px;
            font-weight: bold;
            text-align: center;
            letter-spacing: 3px;
            margin: 20px 0;
            border-radius: 4px;
        }}
        .footer {{ font-size: 12px; color: #666; margin-top: 30px; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Verify Your Email Address</h1>
        <p>Thank you for registering! Your verification code is:</p>

        <div class="code">{verification_token}</div>

        <p><strong>This code will expire in 24 hours.</strong></p>

        <p>If you didn't create an account, please ignore this email.</p>

        <div class="footer">
            <p>This is an automated message from {from_name}. Please do not reply.</p>
        </div>
    </div>
</body>
</html>"#,
            verification_token = verification_token,
            from_name = self.from_name
        )
    }

    /// Generate HTML email template for password reset
    fn generate_password_reset_template(&self, reset_token: &str, user_name: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Reset Your Password</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; background-color: #f5f5f5; }}
        .container {{ max-width: 600px; margin: 0 auto; background-color: white; padding: 30px; border-radius: 8px; }}
        .code {{
            background-color: #fff3cd;
            border: 2px dashed #dc3545;
            padding: 15px;
            font-size: 24px;
            font-weight: bold;
            text-align: center;
            letter-spacing: 3px;
            margin: 20px 0;
            border-radius: 4px;
        }}
        .warning {{
            background-color: #fff3cd;
            border: 1px solid #ffeaa7;
            padding: 15px;
            border-radius: 4px;
            margin: 20px 0;
        }}
        .footer {{ font-size: 12px; color: #666; margin-top: 30px; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Reset Your Password</h1>
        <p>We received a request to reset your password. Your reset code is:</p>

        <div class="code">{reset_token}</div>

        <div class="warning">
            <strong>Security Notice:</strong> This code will expire in 1 hour for your security.
            If you didn't request a password reset, please ignore this email.
        </div>

        <div class="footer">
            <p>This is an automated message from {from_name}. Please do not reply.</p>
        </div>
    </div>
</body>
</html>"#,
            reset_token = reset_token,
            from_name = self.from_name
        )
    }

    /// Send email via backbone SMTP service
    async fn send_via_smtp(
        &self,
        to: &Email,
        subject: &str,
        html_body: &str,
    ) -> Result<String, EmailError> {
        let message_id = format!("msg_{}", uuid::Uuid::new_v4());

        let email_message = EmailMessage::builder()
            .from(&self.from_email)
            .to(to.as_str())
            .subject(subject)
            .html(html_body)
            .build();

        self.smtp_service
            .send(email_message)
            .await
            .map(|report| {
                // Use the report's message_id if available, otherwise use the generated one
                if report.message_id.is_empty() {
                    message_id.clone()
                } else {
                    report.message_id.clone()
                }
            })
            .map_err(|e| EmailError::DeliveryFailed(format!("SMTP error: {:?}", e)))
    }
}

#[async_trait]
impl EmailService for SmtpEmailServiceAdapter {
    async fn send_email(&self, to: &Email, template: EmailTemplate) -> Result<EmailDeliveryResult, EmailError> {
        let (subject, html_body) = match template {
            EmailTemplate::EmailVerification { verification_token, user_name } => {
                ("Verify Your Email Address".to_string(),
                 self.generate_verification_template(&verification_token, &user_name))
            }
            EmailTemplate::PasswordReset { reset_token, user_name } => {
                ("Reset Your Password".to_string(),
                 self.generate_password_reset_template(&reset_token, &user_name))
            }
            EmailTemplate::AccountLocked { lockout_minutes, user_name } => {
                let body = format!(
                    r#"<!DOCTYPE html>
<html>
<head><title>Account Security Alert</title></head>
<body>
    <div style="max-width: 600px; margin: 0 auto; font-family: Arial, sans-serif;">
        <h1>🔒 Account Security Alert</h1>
        <p>Your account has been temporarily locked due to multiple failed login attempts.</p>
        <p>Your account will be automatically unlocked in <strong>{lockout_minutes} minutes</strong>.</p>
    </div>
</body>
</html>"#
                );
                ("Account Security Alert".to_string(), body)
            }
            _ => {
                return Err(EmailError::TemplateError("Template not implemented".to_string()));
            }
        };

        let message_id = self.send_via_smtp(to, &subject, &html_body).await?;

        Ok(EmailDeliveryResult {
            success: true,
            message_id: Some(message_id),
            delivery_status: "sent".to_string(),
            error_message: None,
        })
    }

    async fn send_verification_email(&self, to: &Email, verification_token: &str) -> Result<EmailDeliveryResult, EmailError> {
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

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        Ok(results)
    }

    async fn check_delivery_status(&self, message_id: &str) -> Result<EmailDeliveryStatus, EmailError> {
        Ok(EmailDeliveryStatus {
            message_id: message_id.to_string(),
            status: "delivered".to_string(),
            delivered_at: Some(chrono::Utc::now().to_rfc3339()),
            opened_at: None,
            clicked_at: None,
            bounced_at: None,
            bounce_reason: None,
        })
    }

    async fn get_template(&self, template_name: &str) -> Result<String, EmailError> {
        match template_name {
            "email_verification" => Ok(self.generate_verification_template("123456", "User")),
            "password_reset" => Ok(self.generate_password_reset_template("123456", "User")),
            _ => Err(EmailError::TemplateNotFound(template_name.to_string())),
        }
    }
}

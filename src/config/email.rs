//! Email Configuration Loader
//!
//! Loads SMTP configuration from environment variables and makes it available
//! to the external services layer.
//!
//! ## Environment Variables
//!
//! | Variable | Description | Default |
//! |----------|-------------|---------|
//! | `EMAIL_ENABLED` | Enable email sending | `true` |
//! | `SMTP_HOST` | SMTP server hostname | `smtp.gmail.com` |
//! | `SMTP_PORT` | SMTP server port | `587` |
//! | `SMTP_USERNAME` | SMTP authentication username | - |
//! | `SMTP_PASSWORD` | SMTP authentication password | - |
//! | `SMTP_FROM_EMAIL` | From email address | `noreply@example.com` |
//! | `SMTP_FROM_NAME` | From display name | `Sapiens` |

use crate::infrastructure::external::{EmailService, EmailServiceConfig};

/// Email configuration loader
pub struct EmailConfig {
    pub enabled: bool,
    pub smtp: Option<SmtpConfig>,
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            enabled: std::env::var("EMAIL_ENABLED").map_or(true, |v| v.parse().unwrap_or(false)),
            smtp: None,
        }
    }
}

impl EmailConfig {
    /// Load configuration from environment
    pub fn from_env() -> Self {
        Self {
            enabled: std::env::var("EMAIL_ENABLED").map_or(true, |v| v.parse().unwrap_or(false)),
            smtp: if std::env::var("SMTP_HOST").is_ok() {
                Some(SmtpConfig {
                    host: std::env::var("SMTP_HOST").unwrap(),
                    port: std::env::var("SMTP_PORT")
                        .unwrap_or_else(|_| "587".to_string())
                        .parse()
                        .unwrap_or(587),
                    username: std::env::var("SMTP_USERNAME").ok(),
                    password: std::env::var("SMTP_PASSWORD").ok(),
                })
            } else {
                None
            },
        }
    }

    /// Create email service config from environment config
    pub fn to_email_service_config(&self) -> EmailServiceConfig {
        EmailServiceConfig {
            default_sender: std::env::var("SMTP_FROM_EMAIL")
                .unwrap_or_else(|_| "noreply@example.com".to_string()),
            default_reply_to: None,
            enable_tracking: false,
            max_retries: 3,
            retry_delay: 5,
        }
    }
}

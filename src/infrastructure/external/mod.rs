//! External Services Module for Sapiens Bounded Context
//!
//! Provides integrations with external services such as:
//! - Email verification services (via SMTP)
//! - SMS providers for MFA
//! - Identity providers (OAuth, SAML)
//! - Audit logging services
//!
//! ## SMTP Email Integration
//!
//! This module now includes real SMTP email service integration:
//! - `backbone-email` crate for actual email delivery via lettre
//! - Environment-based configuration for SMTP settings
//! - Email templates for verification codes and password resets
//! - Proper error handling and service adapter pattern

use std::sync::Arc;

/// SMTP email service adapter
pub mod smtp_email_service;

pub use smtp_email_service::{SmtpEmailServiceAdapter, SmtpConfig};

// Re-export EmailService trait for convenience
pub use crate::domain::services::email_service::EmailService;

/// Create SMTP email service from environment
pub async fn create_smtp_email_service() -> Result<Arc<dyn EmailService>, String> {
    let adapter = SmtpEmailServiceAdapter::from_env().await?;
    Ok(Arc::new(adapter))
}

/// Create SMTP email service with custom config
pub async fn create_smtp_email_service_with_config(
    config: SmtpConfig,
) -> Result<Arc<dyn EmailService>, String> {
    let adapter = SmtpEmailServiceAdapter::new(config).await?;
    Ok(Arc::new(adapter))
}

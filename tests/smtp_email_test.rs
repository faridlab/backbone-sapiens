//! Test SMTP email service
//!
//! Run with: SMTP_HOST=... SMTP_USER=... SMTP_PASSWORD=... EMAIL_FROM=... \
//!   cargo test --package backbone-sapiens --test smtp_email_test

use std::env;

#[tokio::test]
async fn test_smtp_config_from_env() {
    use backbone_sapiens::infrastructure::external::SmtpConfig;

    // Set environment variables for testing
    env::set_var("SMTP_HOST", "smtp.hostinger.com");
    env::set_var("SMTP_PORT", "465");
    env::set_var("SMTP_USER", "test@example.com");
    env::set_var("SMTP_PASSWORD", "test_password");
    env::set_var("EMAIL_FROM", "test@example.com");
    env::set_var("EMAIL_FROM_NAME", "Test App");
    env::set_var("SMTP_SECURE", "true");

    // Load config from environment
    let config = SmtpConfig::from_env().expect("Failed to load config from env");

    // Assert configuration values match environment variables
    assert_eq!(config.host, "smtp.hostinger.com");
    assert_eq!(config.port, 465);
    assert_eq!(config.username, Some("test@example.com".to_string()));
    assert_eq!(config.password, Some("test_password".to_string()));
    assert_eq!(config.from_email, "test@example.com");
    assert_eq!(config.from_name, "Test App");
    assert_eq!(config.use_tls, true);
}

#[tokio::test]
async fn test_smtp_service_adapter() {
    use backbone_sapiens::infrastructure::external::{SmtpEmailServiceAdapter, SmtpConfig};

    // Test configuration
    let config = SmtpConfig {
        host: "smtp.hostinger.com".to_string(),
        port: 465,
        username: Some("test@example.com".to_string()),
        password: Some("password".to_string()),
        from_email: "test@example.com".to_string(),
        from_name: "Test App".to_string(),
        use_tls: true,
        timeout: 30,
    };

    // Convert to backbone config
    let backbone_config = config.to_backbone_config();
    assert_eq!(backbone_config.host, "smtp.hostinger.com");
    assert_eq!(backbone_config.port, 465);
}

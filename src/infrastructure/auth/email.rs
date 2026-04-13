//! Auth email adapter
//!
//! Wraps the backbone-email SMTP service with auth-specific email templates.

use backbone_email::{EmailAddress, EmailMessage, EmailService, SmtpEmailService};
use chrono::Utc;
use tracing::{error, info, warn};

// Email HTML template components
const EMAIL_CONTAINER_START: &str = r#"<div style="font-family:sans-serif;max-width:480px;margin:0 auto">"#;

const EMAIL_CONTAINER_END: &str = "</div>";

const EMAIL_FOOTER_IGNORE: &str = r#"<p style="color:#888;font-size:12px">If you didn't create an account, please ignore this email.</p>"#;

const EMAIL_FOOTER_PASSWORD_RESET: &str = r#"<p style="color:#888;font-size:12px">If you didn't request a password reset, please ignore this email. Your account is safe.</p>"#;

const EMAIL_FOOTER_SECURITY: &str = r#"<p>If you did not make this change, please reset your password immediately or contact support.</p>"#;

const OTP_CODE_STYLE: &str = r#"<p style="font-size:32px;font-weight:bold;letter-spacing:8px;text-align:center;background:#f4f4f4;padding:16px;border-radius:8px">{}</p>"#;

/// Auth email service — sends verification, password reset, and confirmation emails.
pub struct AuthEmailService {
    smtp: Option<SmtpEmailService>,
    from_address: String,
    from_name: String,
}

impl AuthEmailService {
    /// Initialize from environment variables.
    pub fn from_env() -> Self {
        let from_address = env_or("EMAIL_FROM", "noreply@bersihir.com");
        let from_name = env_or("EMAIL_FROM_NAME", "Bersihir");

        let smtp = init_smtp();

        Self {
            smtp,
            from_address,
            from_name,
        }
    }

    /// Send an email. Falls back to logging if SMTP is unavailable.
    async fn send(&self, to: &str, subject: &str, html: &str) {
        // Also add plain text version for better deliverability
        let text = html_to_text(html);

        // Create EmailAddress with display name for proper Gmail/SMTP display
        let from_addr = EmailAddress::with_name(&self.from_address, &self.from_name);

        let message = EmailMessage::builder()
            .from(from_addr)
            .to(to)
            .subject(subject)
            .html(html)
            .text(text)  // Add plain text version
            .build();

        match &self.smtp {
            Some(svc) => match svc.send(message).await {
                Ok(report) => {
                    info!("Email sent to {} [id={}]", to, report.message_id);
                }
                Err(e) => {
                    error!("Failed to send email to {}: {}", to, e);
                }
            },
            None => {
                info!(
                    "Email service not configured — logged email to={} subject={}",
                    to, subject
                );
            }
        }
    }

    /// Send OTP verification email.
    pub async fn send_verification_email(&self, to: &str, otp: &str) {
        let otp_code = format!("<p>Your verification code is:</p>{}<p>This code expires in <strong>30 minutes</strong>.</p>{}",
            OTP_CODE_STYLE.replace("{}", otp),
            EMAIL_FOOTER_IGNORE
        );
        let html = format!("{}<h2>Verify Your Email</h2>{}{}", EMAIL_CONTAINER_START, otp_code, EMAIL_CONTAINER_END);
        let subject = format!("Verify your email — {}", self.from_name);
        self.send(to, &subject, &html).await;
    }

    /// Send password reset OTP email.
    pub async fn send_password_reset_email(&self, to: &str, otp: &str) {
        // Security: Don't log OTP in plain text - only log recipient
        info!("Sending password reset email to {}", to);
        let otp_code = format!("<p>Your password reset code is:</p>{}<p>This code expires in <strong>15 minutes</strong>.</p>{}",
            OTP_CODE_STYLE.replace("{}", otp),
            EMAIL_FOOTER_PASSWORD_RESET
        );
        let html = format!("{}<h2>Reset Your Password</h2>{}{}", EMAIL_CONTAINER_START, otp_code, EMAIL_CONTAINER_END);
        let subject = format!("Reset your password — {}", self.from_name);
        self.send(to, &subject, &html).await;
    }

    /// Send password-changed confirmation email.
    pub async fn send_password_changed_email(&self, to: &str) {
        let now = Utc::now().format("%Y-%m-%d %H:%M UTC").to_string();
        let body = format!(
            "<p>Your password was successfully changed on <strong>{}</strong>.</p>{}",
            now,
            EMAIL_FOOTER_SECURITY
        );
        let html = format!("{}<h2>Password Changed</h2>{}{}", EMAIL_CONTAINER_START, body, EMAIL_CONTAINER_END);
        let subject = format!("Your password was changed — {}", self.from_name);
        self.send(to, &subject, &html).await;
    }
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn init_smtp() -> Option<SmtpEmailService> {
    let host = env_or("SMTP_HOST", "localhost");
    let port: u16 = std::env::var("SMTP_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(1025);
    let is_secure = std::env::var("SMTP_SECURE")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);

    // Port 465 uses SSL, port 587 uses STARTTLS
    let use_ssl = port == 465 && is_secure;
    let use_tls = port == 587 && is_secure;

    let mut builder = SmtpEmailService::builder()
        .host(&host)
        .port(port)
        .use_tls(use_tls)
        .use_ssl(use_ssl);

    let username = std::env::var("SMTP_USER").unwrap_or_default();
    let password = std::env::var("SMTP_PASSWORD").unwrap_or_default();
    if !username.is_empty() {
        builder = builder.credentials(&username, &password);
    }

    match builder.build() {
        Ok(svc) => {
            info!("Email service initialized (SMTP {}:{} SSL:{} TLS:{})", host, port, use_ssl, use_tls);
            Some(svc)
        }
        Err(e) => {
            warn!(
                "Email service unavailable: {}. Emails will be logged only.",
                e
            );
            None
        }
    }
}

/// Remove HTML style tags and their content.
fn remove_style_tags(html: &str) -> String {
    let mut result = html.to_string();
    while let Some(start) = result.find("<style") {
        if let Some(end) = result[start..].find('>') {
            let tag_end = start + end + 1;
            if let Some(close_start) = result[tag_end..].find("</style>") {
                let close_end = tag_end + close_start + 8;
                result.replace_range(start..close_end, "");
            } else {
                break;
            }
        } else {
            break;
        }
    }
    result
}

/// Convert HTML entities to their text equivalents.
fn decode_html_entities(html: &str) -> String {
    html.replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
}

/// Strip remaining HTML tags after structured replacements.
fn strip_html_tags(html: &str) -> String {
    html.split('<')
        .map(|s| s.split('>').next().unwrap_or(s))
        .collect::<Vec<&str>>()
        .join("")
}

/// Clean up whitespace: trim lines and remove empty lines.
fn cleanup_whitespace(text: &str) -> String {
    text.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>()
        .join("\n")
}

/// Simple HTML to text conversion for email plain text fallback.
/// Handles common HTML tags for the simple email templates used in this service.
fn html_to_text(html: &str) -> String {
    let mut result = html.to_string();

    // Remove style tags with content
    result = remove_style_tags(&result);

    // Replace common tags with text equivalents
    result = result
        .replace("<h2>", "")
        .replace("</h2>", "\n\n")
        .replace("<h1>", "")
        .replace("</h1>", "\n\n")
        .replace("<p>", "")
        .replace("</p>", "\n\n")
        .replace("<strong>", "")
        .replace("</strong>", "")
        .replace("<b>", "")
        .replace("</b>", "")
        .replace("<br>", "\n")
        .replace("<br/>", "\n")
        .replace("<br />", "\n")
        .replace("<hr>", "\n---\n")
        .replace("<hr/>", "\n---\n");

    // Decode HTML entities
    result = decode_html_entities(&result);

    // Strip any remaining HTML tags
    result = strip_html_tags(&result);

    // Clean up whitespace
    cleanup_whitespace(&result)
}

//! JWT infrastructure service
//!
//! Handles access token generation and validation using HS256.

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::warn;

/// JWT configuration loaded from environment variables.
pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub audience: String,
    pub access_token_ttl_secs: i64,
}

impl JwtConfig {
    /// Load configuration from environment variables.
    ///
    /// Panics in production if `JWT_SECRET` is not set.
    pub fn from_env() -> Self {
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
            let is_prod = std::env::var("APP_ENV").unwrap_or_default() == "production";
            if is_prod {
                panic!("JWT_SECRET must be set in production");
            }
            warn!("JWT_SECRET not set — using development default (NOT safe for production)");
            "your-super-secret-jwt-key-change-in-production".to_string()
        });

        Self {
            secret,
            issuer: std::env::var("JWT_ISSUER")
                .unwrap_or_else(|_| "backbone-framework".to_string()),
            audience: std::env::var("JWT_AUDIENCE")
                .unwrap_or_else(|_| "backbone-users".to_string()),
            access_token_ttl_secs: 86400, // 24 hours
        }
    }
}

/// JWT claims structure — inserted into request extensions by the auth middleware.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (user ID)
    pub exp: usize,         // Expiration time
    pub iat: usize,         // Issued at
    pub iss: String,        // Issuer
    pub aud: String,        // Audience
    pub roles: Vec<String>, // User roles
    pub email: String,      // User email
}

/// JWT service for token generation and validation.
pub struct JwtService {
    config: JwtConfig,
}

impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    /// Generate an access token. Returns `(token_string, expires_in_secs)`.
    pub fn generate_access_token(
        &self,
        user_id: &str,
        email: &str,
        roles: &[String],
    ) -> Result<(String, i64), anyhow::Error> {
        let expires_in = self.config.access_token_ttl_secs;
        let now = Utc::now();

        let claims = Claims {
            sub: user_id.to_string(),
            exp: (now + Duration::seconds(expires_in)).timestamp() as usize,
            iat: now.timestamp() as usize,
            iss: self.config.issuer.clone(),
            aud: self.config.audience.clone(),
            roles: roles.to_vec(),
            email: email.to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.secret.as_bytes()),
        )?;

        Ok((token, expires_in))
    }

    /// Validate an access token and return the claims.
    pub fn validate_token(&self, token: &str) -> Result<Claims, String> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[&self.config.issuer]);
        validation.set_audience(&[&self.config.audience]);
        validation.validate_exp = true;

        let key = DecodingKey::from_secret(self.config.secret.as_bytes());
        decode::<Claims>(token, &key, &validation)
            .map(|data| data.claims)
            .map_err(|e| format!("Invalid token: {}", e))
    }
}

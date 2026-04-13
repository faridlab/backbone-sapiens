//! Mock JWT Service
//!
//! Simple mock JWT implementation for testing.

use backbone_auth::jwt::JwtService;
use backbone_auth::traits::{Claims, RefreshTokenClaims};
use async_trait::async_trait;
use chrono::{Utc, Duration};
use uuid::Uuid;

/// Mock JWT Service
#[derive(Clone)]
pub struct MockJwtService {
    pub secret: String,
}

impl MockJwtService {
    pub fn new() -> Self {
        Self {
            secret: "test_secret_key_for_mock_jwt".to_string(),
        }
    }

    pub fn with_secret(secret: String) -> Self {
        Self { secret }
    }
}

impl Default for MockJwtService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl JwtService for MockJwtService {
    async fn generate_access_token(&self, user_id: Uuid) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok(format!("mock_access_token_{}", user_id))
    }

    async fn generate_refresh_token(&self, user_id: Uuid) -> Result<RefreshTokenClaims, Box<dyn std::error::Error + Send + Sync>> {
        let now = Utc::now();
        Ok(RefreshTokenClaims {
            sub: user_id.to_string(),
            exp: (now + Duration::days(7)).timestamp() as usize,
            iat: now.timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
            token_type: "refresh".to_string(),
        })
    }

    async fn validate_token(&self, token: &str) -> Result<Claims, Box<dyn std::error::Error + Send + Sync>> {
        if token.starts_with("mock_access_token_") {
            let user_id_str = token.strip_prefix("mock_access_token_").unwrap_or("invalid");
            if let Ok(user_id) = Uuid::parse_str(user_id_str) {
                Ok(Claims {
                    sub: user_id.to_string(),
                    exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
                    iat: Utc::now().timestamp() as usize,
                    iss: "mock".to_string(),
                })
            } else {
                Err("Invalid token".into())
            }
        } else {
            Err("Invalid token".into())
        }
    }

    async fn validate_refresh_token(&self, token: &str) -> Result<RefreshTokenClaims, Box<dyn std::error::Error + Send + Sync>> {
        if token.starts_with("mock_refresh_token_") {
            let user_id_str = token.strip_prefix("mock_refresh_token_").unwrap_or("invalid");
            if let Ok(user_id) = Uuid::parse_str(user_id_str) {
                let now = Utc::now();
                Ok(RefreshTokenClaims {
                    sub: user_id.to_string(),
                    exp: (now + Duration::days(7)).timestamp() as usize,
                    iat: now.timestamp() as usize,
                    jti: Uuid::new_v4().to_string(),
                    token_type: "refresh".to_string(),
                })
            } else {
                Err("Invalid refresh token".into())
            }
        } else {
            Err("Invalid refresh token".into())
        }
    }

    async fn refresh_access_token(&self, refresh_token: &str) -> Result<(String, RefreshTokenClaims), Box<dyn std::error::Error + Send + Sync>> {
        let claims = self.validate_refresh_token(refresh_token).await?;
        let user_id = Uuid::parse_str(&claims.sub)?;
        Ok((format!("mock_access_token_{}", user_id), claims))
    }
}

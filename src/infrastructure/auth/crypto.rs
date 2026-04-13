//! Cryptography utilities for authentication
//!
//! Provides password hashing (Argon2), OTP generation, token hashing (SHA-256),
//! and constant-time comparison.

use rand::Rng;
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Hash a password using Argon2 with a random salt.
pub fn hash_password(password: &str) -> Result<String, anyhow::Error> {
    use argon2::password_hash::{rand_core::OsRng, SaltString};
    use argon2::{Argon2, PasswordHasher};

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Password hash failed: {}", e))?;
    Ok(hash.to_string())
}

/// Verify a password against an Argon2 hash.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, anyhow::Error> {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};

    let parsed = PasswordHash::new(hash)
        .map_err(|e| anyhow::anyhow!("Invalid hash format: {}", e))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

/// Generate a 6-digit OTP code.
pub fn generate_otp() -> String {
    let code: u32 = rand::thread_rng().gen_range(100_000..1_000_000);
    format!("{:06}", code)
}

/// SHA-256 hash a token/OTP before storing in DB.
pub fn hash_token(token: &str) -> String {
    let result = Sha256::digest(token.as_bytes());
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Constant-time string comparison to prevent timing side-channel attacks.
pub fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.bytes()
        .zip(b.bytes())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}

/// Generate a new refresh token (UUID v4).
pub fn generate_refresh_token() -> String {
    Uuid::new_v4().to_string()
}

//! Envelope encryption for the credential store.
//!
//! The master key (`CREDENTIAL_MASTER_KEY`) never encrypts directly: each
//! credential is sealed under a KEK derived per `key_id` via HKDF-SHA256, and
//! the ciphertext is AES-256-GCM with the scope triple bound as AAD. A row
//! copied to another scope (different provider, account, or purpose) fails
//! authentication — GCM rejects it — so exfiltrated rows are unusable out of
//! context. Plaintext buffers are zeroized on drop.
//!
//! The master key comes from the environment and is required: when it is unset
//! the whole credential surface fails closed (no issue, no read) rather than
//! storing secrets under a NULL key.

use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use hkdf::Hkdf;
use rand::RngCore;
use sha2::Sha256;
use zeroize::Zeroize;

/// HKDF salt — versioned domain separator for the credential store's KEK
/// derivation. Bump the version (and mint a new `key_id` generation) when the
/// derivation parameters ever change.
pub const KEK_SALT: &[u8] = b"backbone-sapiens/credential-store/v1";

/// Current KEK generation id. Stored per row in `key_id`; a master-key rotation
/// mints the next generation and a later re-encrypt pass moves rows over.
pub const CURRENT_KEY_ID: &str = "k1";

const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("credential master key is not configured (CREDENTIAL_MASTER_KEY) — the credential store fails closed")]
    MissingMasterKey,
    #[error("ciphertext failed authentication — wrong key or scope (provider/account/purpose) mismatch")]
    Authentication,
    #[error("ciphertext is malformed: {0}")]
    Malformed(&'static str),
    #[error("HKDF-SHA256 key derivation failed for key id {0}")]
    Kdf(String),
}

/// The scope triple bound as authenticated associated data. Serialization is
/// canonical (colon-joined, no escaping) — colons cannot appear in the slug
/// validation the service enforces upstream.
#[derive(Debug, Clone)]
pub struct CredentialScope {
    pub provider: String,
    pub account_ref: String,
    pub purpose: String,
}

impl CredentialScope {
    pub fn aad(&self) -> Vec<u8> {
        format!("{}:{}:{}", self.provider, self.account_ref, self.purpose).into_bytes()
    }
}

fn derive_kek(master_key: &[u8], key_id: &str) -> Result<[u8; KEY_LEN], CryptoError> {
    let hk = Hkdf::<Sha256>::new(Some(KEK_SALT), master_key);
    let mut okm = [0u8; KEY_LEN];
    // HKDF-SHA256 with a 32-byte output cannot fail in practice, but a panic
    // on a crypto path is never acceptable — the error propagates instead.
    hk.expand(key_id.as_bytes(), &mut okm)
        .map_err(|_| CryptoError::Kdf(key_id.to_string()))?;
    Ok(okm)
}

fn cipher_for(master_key: &[u8], key_id: &str) -> Result<Aes256Gcm, CryptoError> {
    let kek = derive_kek(master_key, key_id)?;
    Ok(Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&kek)))
}

/// Read the master key from the environment. `Err` means "fail closed".
pub fn master_key_from_env() -> Result<Vec<u8>, CryptoError> {
    let raw = std::env::var("CREDENTIAL_MASTER_KEY").unwrap_or_default();
    if raw.is_empty() {
        return Err(CryptoError::MissingMasterKey);
    }
    // Accept both raw 32-byte strings and their base64 encoding; anything else
    // is a configuration error surfaced as fail-closed.
    let decoded = match B64.decode(raw.as_bytes()) {
        Ok(bytes) if bytes.len() == KEY_LEN => bytes,
        _ if raw.as_bytes().len() == KEY_LEN => raw.into_bytes(),
        _ => return Err(CryptoError::MissingMasterKey),
    };
    if decoded.len() != KEY_LEN {
        return Err(CryptoError::MissingMasterKey);
    }
    Ok(decoded)
}

/// Seal a secret: returns base64(nonce ‖ ct ‖ tag) under the CURRENT key
/// generation, AAD-bound to the scope.
pub fn seal(master_key: &[u8], scope: &CredentialScope, secret: &[u8]) -> Result<String, CryptoError> {
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);

    let payload = Payload {
        msg: secret,
        aad: &scope.aad(),
    };
    let sealed = cipher_for(master_key, CURRENT_KEY_ID)?
        .encrypt(Nonce::from_slice(&nonce_bytes), payload)
        .map_err(|_| CryptoError::Authentication)?;

    let mut envelope = Vec::with_capacity(NONCE_LEN + sealed.len());
    envelope.extend_from_slice(&nonce_bytes);
    envelope.extend_from_slice(&sealed);
    Ok(B64.encode(&envelope))
}

/// Open a sealed secret. `key_id` selects the KEK generation (rows sealed under
/// an older generation stay readable until re-encrypted); the scope must match
/// the sealing scope exactly or GCM authentication fails.
pub fn open(
    master_key: &[u8],
    key_id: &str,
    scope: &CredentialScope,
    ciphertext: &str,
) -> Result<ZeroizingSecret, CryptoError> {
    let envelope = B64
        .decode(ciphertext.as_bytes())
        .map_err(|_| CryptoError::Malformed("invalid base64"))?;
    if envelope.len() <= NONCE_LEN {
        return Err(CryptoError::Malformed("truncated envelope"));
    }
    let (nonce_bytes, sealed) = envelope.split_at(NONCE_LEN);

    let payload = Payload {
        msg: sealed,
        aad: &scope.aad(),
    };
    let opened = cipher_for(master_key, key_id)?
        .decrypt(Nonce::from_slice(nonce_bytes), payload)
        .map_err(|_| CryptoError::Authentication)?;

    Ok(ZeroizingSecret(opened))
}

/// A plaintext secret that zeroizes on drop. Use `.expose()` sparingly — only
/// to hand bytes to a consumer (HMAC key, Authorization header), never to log.
pub struct ZeroizingSecret(Vec<u8>);

impl ZeroizingSecret {
    pub fn expose(&self) -> &[u8] {
        &self.0
    }
    pub fn as_string(&self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }
}

impl Drop for ZeroizingSecret {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scope() -> CredentialScope {
        CredentialScope {
            provider: "doku".into(),
            account_ref: "3f2b1a".into(),
            purpose: "webhook_verify".into(),
        }
    }

    #[test]
    fn seal_open_roundtrip() {
        let key = [7u8; 32];
        let ct = seal(&key, &scope(), b"hunter2").unwrap();
        let pt = open(&key, CURRENT_KEY_ID, &scope(), &ct).unwrap();
        assert_eq!(pt.expose(), b"hunter2");
    }

    #[test]
    fn wrong_scope_fails_authentication() {
        let key = [7u8; 32];
        let ct = seal(&key, &scope(), b"hunter2").unwrap();
        let other = CredentialScope {
            provider: "midtrans".into(),
            ..scope()
        };
        assert!(matches!(
            open(&key, CURRENT_KEY_ID, &other, &ct),
            Err(CryptoError::Authentication)
        ));
    }

    #[test]
    fn wrong_key_fails_authentication() {
        let key = [7u8; 32];
        let ct = seal(&key, &scope(), b"hunter2").unwrap();
        assert!(matches!(
            open(&[8u8; 32], CURRENT_KEY_ID, &scope(), &ct),
            Err(CryptoError::Authentication)
        ));
    }

    #[test]
    fn wrong_key_id_fails_authentication() {
        let key = [7u8; 32];
        let ct = seal(&key, &scope(), b"hunter2").unwrap();
        assert!(matches!(
            open(&key, "k0", &scope(), &ct),
            Err(CryptoError::Authentication)
        ));
    }

    #[test]
    fn tampered_ciphertext_fails() {
        let key = [7u8; 32];
        let mut ct = seal(&key, &scope(), b"hunter2").unwrap();
        // flip a character inside the envelope
        let unsafe_mid = ct.len() / 2;
        ct.replace_range(unsafe_mid..unsafe_mid + 1, if ct.as_bytes()[unsafe_mid] == b'A' { "B" } else { "A" });
        assert!(open(&key, CURRENT_KEY_ID, &scope(), &ct).is_err());
    }

    #[test]
    fn missing_master_key_env_fails_closed() {
        std::env::remove_var("CREDENTIAL_MASTER_KEY");
        assert!(matches!(master_key_from_env(), Err(CryptoError::MissingMasterKey)));
    }
}

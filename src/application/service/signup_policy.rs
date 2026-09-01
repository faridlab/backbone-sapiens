//! Signup policy — the kill-switchable registration gate.
//!
//! Self-service signup is a POLICY surface, never an install-time bootstrap:
//! the durable posture lives in `sapiens.auth_policy` (key `signup`), and a
//! missing row means the most restrictive posture (`Off`). Nothing is seeded at
//! migration time, so a freshly deployed database starts with signup disabled
//! and an operator must deliberately flip the row to open it.
//!
//! Postures:
//! - `Off`             — registration refused outright (the default).
//! - `InvitationOnly`  — registration requires an invitation credential that
//!   verifies AND is not on the revocation list. The shipped default path.
//! - `Open`            — registration open to anyone (deliberate opt-in; the
//!   public form still throttles and never reveals whether an address exists).
//!
//! Invitation credentials are minted elsewhere (the portal module issues them
//! as Tier A capabilities); this service owns the sapiens-side halves: the
//! [`InvitationVerifier`] seam the host wires, and the durable revocation list
//! (`sapiens.auth_signup_revocations`) that archive, delete and login-kill
//! flows punch revoked credentials into.

use sqlx::PgPool;

/// The signup posture read from `sapiens.auth_policy`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignupMode {
    /// Registration refused outright. The default when no policy row exists.
    Off,
    /// Registration requires a verified, unrevoked invitation credential.
    InvitationOnly,
    /// Registration open (still throttled; still identity-opaque).
    Open,
}

impl SignupMode {
    /// Parse the mode stored in the policy JSON value.
    fn from_value(value: &serde_json::Value) -> Option<Self> {
        match value.get("mode")?.as_str()? {
            "off" => Some(Self::Off),
            "invitation_only" => Some(Self::InvitationOnly),
            "open" => Some(Self::Open),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::InvitationOnly => "invitation_only",
            Self::Open => "open",
        }
    }
}

/// A verified invitation credential — what an [`InvitationVerifier`] returns
/// for a token it accepts. `credential_id` is the stable identifier the
/// revocation list keys on.
#[derive(Clone, Debug)]
pub struct VerifiedInvitation {
    pub credential_id: String,
    pub email: Option<String>,
}

/// Seam for verifying invitation tokens. The portal module (or the host) wires
/// the real verifier; with no verifier wired, `InvitationOnly` mode refuses
/// every registration — fail-closed by construction.
#[async_trait::async_trait]
pub trait InvitationVerifier: Send + Sync {
    async fn verify(&self, token: &str) -> Option<VerifiedInvitation>;
}

/// Durable signup policy + invitation revocation list.
pub struct SignupPolicyService {
    pool: PgPool,
}

impl SignupPolicyService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Read the current signup posture. Missing or malformed row ⇒ `Off`
    /// (fail-closed; the absence of a decision is a refusal, not an opening).
    pub async fn signup_mode(&self) -> SignupMode {
        let row: Option<(serde_json::Value,)> = sqlx::query_as(
            "SELECT value FROM sapiens.auth_policy WHERE key = 'signup'",
        )
        .fetch_optional(&self.pool)
        .await
        .ok()
        .flatten();

        row.and_then(|(v,)| SignupMode::from_value(&v))
            .unwrap_or(SignupMode::Off)
    }

    /// Overwrite the signup posture (the kill-switch lever). Flipping to `Off`
    /// immediately refuses every in-flight registration attempt on its next
    /// policy read — no process restart, no redeploy.
    pub async fn set_signup_mode(&self, mode: SignupMode) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO sapiens.auth_policy (key, value, updated_at) \
             VALUES ('signup', $1::jsonb, NOW()) \
             ON CONFLICT (key) DO UPDATE SET value = $1::jsonb, updated_at = NOW()",
        )
        .bind(serde_json::json!({ "mode": mode.as_str() }).to_string())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Is this invitation credential on the revocation list?
    ///
    /// An unreadable revocation list counts as revoked (fail closed): an
    /// invitation must never verify because the list could not be checked.
    pub async fn invitation_is_revoked(&self, credential_id: &str) -> bool {
        match sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM sapiens.auth_signup_revocations \
             WHERE credential_id = $1",
        )
        .bind(credential_id)
        .fetch_one(&self.pool)
        .await
        {
            Ok(n) => n > 0,
            Err(_) => true,
        }
    }

    /// Add a credential to the revocation list (idempotent). Archive/delete of
    /// a user, or an explicit invite kill, punches the credential here so the
    /// invitation can never be replayed even while the token itself looks
    /// structurally valid.
    pub async fn revoke_invitation(
        &self,
        credential_id: &str,
        reason: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO sapiens.auth_signup_revocations (credential_id, reason) \
             VALUES ($1, $2) \
             ON CONFLICT (credential_id) DO NOTHING",
        )
        .bind(credential_id)
        .bind(reason)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signup_mode_parses_every_stored_form() {
        let off = serde_json::json!({"mode": "off"});
        let inv = serde_json::json!({"mode": "invitation_only"});
        let open = serde_json::json!({"mode": "open"});
        let junk = serde_json::json!({"mode": "something-else"});
        assert_eq!(SignupMode::from_value(&off), Some(SignupMode::Off));
        assert_eq!(SignupMode::from_value(&inv), Some(SignupMode::InvitationOnly));
        assert_eq!(SignupMode::from_value(&open), Some(SignupMode::Open));
        assert_eq!(SignupMode::from_value(&junk), None);
        assert_eq!(SignupMode::from_value(&serde_json::json!({})), None);
    }
}

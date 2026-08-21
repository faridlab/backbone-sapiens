use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{AuditMetadata, CredentialPurpose, CredentialStatus};

/// A stored integration credential (envelope-encrypted). The secret exists only
/// inside `ciphertext`; this struct is used internally by the credential service
/// and must never be serialized into an HTTP response — responses carry
/// [`crate::application::service::integration_credential_service::CredentialDescriptor`]
/// (metadata only).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IntegrationCredential {
    pub id: Uuid,
    pub company_id: Uuid,
    pub provider: String,
    pub account_ref: String,
    pub purpose: CredentialPurpose,
    pub key_id: String,
    pub ciphertext: String,
    pub status: CredentialStatus,
    pub expires_at: Option<DateTime<Utc>>,
    pub rotated_from: Option<Uuid>,
    pub last_used_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl IntegrationCredential {
    /// A credential past its honest expiry is treated as expired regardless of
    /// the stored status — the read path CASes the status over when it observes
    /// this, so the drift self-heals on first observation.
    pub fn effective_status(&self, now: DateTime<Utc>) -> CredentialStatus {
        if self.status == CredentialStatus::Active
            && self.expires_at.map(|e| e <= now).unwrap_or(false)
        {
            CredentialStatus::Expired
        } else {
            self.status
        }
    }
}

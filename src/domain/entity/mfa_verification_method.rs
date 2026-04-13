use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "mfa_verification_method", rename_all = "snake_case")]
pub enum MFAVerificationMethod {
    Totp,
    Sms,
    Email,
    HardwareKey,
    PushNotification,
    BackupCode,
    Biometric,
}

impl std::fmt::Display for MFAVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Totp => write!(f, "totp"),
            Self::Sms => write!(f, "sms"),
            Self::Email => write!(f, "email"),
            Self::HardwareKey => write!(f, "hardware_key"),
            Self::PushNotification => write!(f, "push_notification"),
            Self::BackupCode => write!(f, "backup_code"),
            Self::Biometric => write!(f, "biometric"),
        }
    }
}

impl FromStr for MFAVerificationMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "totp" => Ok(Self::Totp),
            "sms" => Ok(Self::Sms),
            "email" => Ok(Self::Email),
            "hardware_key" => Ok(Self::HardwareKey),
            "push_notification" => Ok(Self::PushNotification),
            "backup_code" => Ok(Self::BackupCode),
            "biometric" => Ok(Self::Biometric),
            _ => Err(format!("Unknown MFAVerificationMethod variant: {}", s)),
        }
    }
}

impl Default for MFAVerificationMethod {
    fn default() -> Self {
        Self::Totp
    }
}

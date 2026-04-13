use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "mfa_device_type", rename_all = "snake_case")]
pub enum MFADeviceType {
    Totp,
    Sms,
    Email,
    HardwareKey,
    Biometric,
    PushNotification,
}

impl std::fmt::Display for MFADeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Totp => write!(f, "totp"),
            Self::Sms => write!(f, "sms"),
            Self::Email => write!(f, "email"),
            Self::HardwareKey => write!(f, "hardware_key"),
            Self::Biometric => write!(f, "biometric"),
            Self::PushNotification => write!(f, "push_notification"),
        }
    }
}

impl FromStr for MFADeviceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "totp" => Ok(Self::Totp),
            "sms" => Ok(Self::Sms),
            "email" => Ok(Self::Email),
            "hardware_key" => Ok(Self::HardwareKey),
            "biometric" => Ok(Self::Biometric),
            "push_notification" => Ok(Self::PushNotification),
            _ => Err(format!("Unknown MFADeviceType variant: {}", s)),
        }
    }
}

impl Default for MFADeviceType {
    fn default() -> Self {
        Self::Totp
    }
}

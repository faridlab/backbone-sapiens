mod data_export_state_machine;
mod email_verification_token_state_machine;
mod impersonation_session_state_machine;
mod ldap_directory_state_machine;
mod mfa_device_state_machine;
mod notification_preference_state_machine;
mod o_auth_provider_state_machine;
mod password_reset_token_state_machine;
mod resource_permission_state_machine;
mod saml_provider_state_machine;
mod security_event_state_machine;
mod session_state_machine;
mod session_limit_state_machine;
mod temporary_permission_state_machine;
mod user_state_machine;
mod user_o_auth_link_state_machine;
mod user_permission_state_machine;
mod user_saml_link_state_machine;

/// Shared error type for all state machines in this module
#[derive(Debug, Clone, thiserror::Error)]
pub enum StateMachineError {
    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Invalid transition: {0}")]
    InvalidTransition(String),

    #[error("Transition '{transition}' not allowed from state '{from}'")]
    TransitionNotAllowed {
        transition: String,
        from: String,
    },

    #[error("Role '{role}' not authorized for transition '{transition}'")]
    RoleNotAuthorized {
        role: String,
        transition: String,
    },

    #[error("Guard condition failed for transition '{0}'")]
    GuardFailed(String),

    #[error("Cannot transition from final state '{0}'")]
    FinalStateReached(String),
}

pub use data_export_state_machine::{DataExportState, DataExportTransition, DataExportStateMachine};
pub use email_verification_token_state_machine::{EmailVerificationTokenState, EmailVerificationTokenTransition, EmailVerificationTokenStateMachine};
pub use impersonation_session_state_machine::{ImpersonationSessionState, ImpersonationSessionTransition, ImpersonationSessionStateMachine};
pub use ldap_directory_state_machine::{LDAPDirectoryState, LDAPDirectoryTransition, LDAPDirectoryStateMachine};
pub use mfa_device_state_machine::{MFADeviceState, MFADeviceTransition, MFADeviceStateMachine};
pub use notification_preference_state_machine::{NotificationPreferenceState, NotificationPreferenceTransition, NotificationPreferenceStateMachine};
pub use o_auth_provider_state_machine::{OAuthProviderState, OAuthProviderTransition, OAuthProviderStateMachine};
pub use password_reset_token_state_machine::{PasswordResetTokenState, PasswordResetTokenTransition, PasswordResetTokenStateMachine};
pub use resource_permission_state_machine::{ResourcePermissionState, ResourcePermissionTransition, ResourcePermissionStateMachine};
pub use saml_provider_state_machine::{SAMLProviderState, SAMLProviderTransition, SAMLProviderStateMachine};
pub use security_event_state_machine::{SecurityEventState, SecurityEventTransition, SecurityEventStateMachine};
pub use session_state_machine::{SessionState, SessionTransition, SessionStateMachine};
pub use session_limit_state_machine::{SessionLimitState, SessionLimitTransition, SessionLimitStateMachine};
pub use temporary_permission_state_machine::{TemporaryPermissionState, TemporaryPermissionTransition, TemporaryPermissionStateMachine};
pub use user_state_machine::{UserState, UserTransition, UserStateMachine};
pub use user_o_auth_link_state_machine::{UserOAuthLinkState, UserOAuthLinkTransition, UserOAuthLinkStateMachine};
pub use user_permission_state_machine::{UserPermissionState, UserPermissionTransition, UserPermissionStateMachine};
pub use user_saml_link_state_machine::{UserSAMLLinkState, UserSAMLLinkTransition, UserSAMLLinkStateMachine};

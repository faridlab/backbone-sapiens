//! Messaging Module for Sapiens Bounded Context
//!
//! This module provides both domain events (internal) and integration events (cross-module).
//!
//! # Architecture
//!
//! ```text
//! Domain Events (Internal)          Integration Events (Cross-Module)
//! ┌─────────────────────────┐       ┌─────────────────────────────┐
//! │ UserDomainEvent         │──────▶│ UserCreatedIntegrationEvent │
//! │ (UserCreated, etc.)     │       │ (sapiens.user.created)      │
//! └─────────────────────────┘       └─────────────────────────────┘
//!           │                                    │
//!           ▼                                    ▼
//! ┌─────────────────────────┐       ┌─────────────────────────────┐
//! │ EventBus<UserDomainEvent>│       │ IntegrationEventBus         │
//! │ (typed, internal)       │       │ (type-erased, cross-module) │
//! └─────────────────────────┘       └─────────────────────────────┘
//! ```
//!
//! # Usage
//!
//! ```rust,ignore
//! // Internal domain events
//! let domain_bus = create_sapiens_event_bus();
//! domain_bus.publish(UserDomainEvent::Created { ... }).await;
//!
//! // Cross-module integration events
//! let integration_bus = IntegrationEventBus::new();
//! let publisher = SapiensIntegrationEventPublisher::new(integration_bus.clone());
//! domain_bus.register_handler(Arc::new(publisher)).await;
//! ```

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::domain::entity::UserDomainEvent;

// Sub-modules
pub mod integration_events;
pub mod event_translator;

// Re-export integration event types
pub use integration_events::*;
pub use event_translator::SapiensIntegrationEventPublisher;

// Re-export backbone-messaging types for UserDomainEvent
pub use backbone_messaging::{
    DomainEvent,
    EventBus as GenericEventBus,
    EventBusConfig,
    EventEnvelope as GenericEventEnvelope,
    EventHandler as GenericEventHandler,
    EventError,
    // Integration event types
    IntegrationEventBus,
    IntegrationBusConfig,
    IntegrationEventHandler,
    IntegrationEvent,
    IntegrationEventEnvelope,
};

/// Type alias for EventBus specialized for UserDomainEvent
pub type EventBus = GenericEventBus<UserDomainEvent>;

/// Type alias for EventEnvelope specialized for UserDomainEvent
pub type EventEnvelope = GenericEventEnvelope<UserDomainEvent>;

/// Event handler trait for UserDomainEvent
/// Re-exported from backbone-messaging
pub type EventHandler = dyn GenericEventHandler<UserDomainEvent>;

// ============================================================
// Sapiens-specific Event Handlers
// ============================================================

/// Logging event handler for Sapiens domain events
pub struct SapiensLoggingHandler {
    event_types: Vec<&'static str>,
}

impl SapiensLoggingHandler {
    /// Create a handler that logs specific event types
    pub fn new(event_types: Vec<&'static str>) -> Self {
        Self { event_types }
    }

    /// Create a handler that logs all event types
    pub fn all() -> Self {
        Self {
            event_types: vec![
                "UserCreated",
                "UserProfileUpdated",
                "UserEmailChanged",
                "UserEmailVerified",
                "UserPasswordChanged",
                "UserMfaEnabled",
                "UserMfaDisabled",
                "UserAccountLocked",
                "UserAccountUnlocked",
                "UserActivated",
                "UserDeactivated",
                "UserSuspended",
                "UserDeleted",
                "UserRestored",
                "UserLoggedIn",
                "UserLoginFailed",
                "UserLoggedOut",
            ],
        }
    }
}

#[async_trait::async_trait]
impl GenericEventHandler<UserDomainEvent> for SapiensLoggingHandler {
    async fn handle(&self, envelope: GenericEventEnvelope<UserDomainEvent>) -> Result<(), EventError> {
        tracing::info!(
            event_type = %envelope.event_type,
            aggregate_id = %envelope.aggregate_id,
            event_id = %envelope.id,
            aggregate_type = "User",
            "Sapiens domain event received"
        );
        Ok(())
    }

    fn event_types(&self) -> Vec<&'static str> {
        self.event_types.clone()
    }
}

/// Email notification handler for user events
pub struct EmailNotificationHandler;

#[async_trait::async_trait]
impl GenericEventHandler<UserDomainEvent> for EmailNotificationHandler {
    async fn handle(&self, envelope: GenericEventEnvelope<UserDomainEvent>) -> Result<(), EventError> {
        let Some(event) = envelope.event() else {
            return Ok(());
        };

        match event {
            UserDomainEvent::Created { user_id, .. } => {
                tracing::info!(user_id = %user_id, "Would send welcome email");
            }
            UserDomainEvent::EmailVerified { user_id, .. } => {
                tracing::info!(user_id = %user_id, "Would send email verification confirmation");
            }
            UserDomainEvent::PasswordChanged { user_id, .. } => {
                tracing::info!(user_id = %user_id, "Would send password change notification");
            }
            UserDomainEvent::AccountLocked { user_id, reason, .. } => {
                tracing::warn!(user_id = %user_id, reason = %reason, "Would send account locked notification");
            }
            _ => {}
        }
        Ok(())
    }

    fn event_types(&self) -> Vec<&'static str> {
        vec![
            "UserCreated",
            "UserEmailVerified",
            "UserPasswordChanged",
            "UserAccountLocked",
        ]
    }
}

/// Audit logging handler for security-sensitive events
pub struct AuditLoggingHandler;

#[async_trait::async_trait]
impl GenericEventHandler<UserDomainEvent> for AuditLoggingHandler {
    async fn handle(&self, envelope: GenericEventEnvelope<UserDomainEvent>) -> Result<(), EventError> {
        let Some(event) = envelope.event() else {
            return Ok(());
        };

        let (action, severity) = match event {
            UserDomainEvent::Created { .. } => ("user_created", "info"),
            UserDomainEvent::PasswordChanged { .. } => ("password_changed", "info"),
            UserDomainEvent::EmailChanged { .. } => ("email_changed", "warning"),
            UserDomainEvent::MfaEnabled { .. } => ("mfa_enabled", "info"),
            UserDomainEvent::MfaDisabled { .. } => ("mfa_disabled", "warning"),
            UserDomainEvent::AccountLocked { .. } => ("account_locked", "warning"),
            UserDomainEvent::Suspended { .. } => ("account_suspended", "warning"),
            UserDomainEvent::Deleted { .. } => ("account_deleted", "info"),
            UserDomainEvent::LoggedIn { .. } => ("user_logged_in", "info"),
            UserDomainEvent::LoginFailed { .. } => ("login_failed", "warning"),
            UserDomainEvent::LoggedOut { .. } => ("user_logged_out", "info"),
            _ => return Ok(()),
        };

        tracing::info!(
            action = %action,
            severity = %severity,
            user_id = %envelope.aggregate_id,
            event_id = %envelope.id,
            "Audit log entry"
        );

        Ok(())
    }

    fn event_types(&self) -> Vec<&'static str> {
        vec![
            "UserCreated",
            "UserPasswordChanged",
            "UserEmailChanged",
            "UserMfaEnabled",
            "UserMfaDisabled",
            "UserAccountLocked",
            "UserSuspended",
            "UserDeleted",
            "UserLoggedIn",
            "UserLoginFailed",
            "UserLoggedOut",
        ]
    }
}

// ============================================================
// Helper Functions
// ============================================================

/// Create a configured EventBus for Sapiens module
pub fn create_sapiens_event_bus() -> EventBus {
    EventBus::with_config(EventBusConfig {
        buffer_size: 1000,
        persist_events: true,
        retention_seconds: 86400, // 24 hours
        max_history_size: 10000,
    })
}

/// Create an EventBus with all standard handlers registered
pub async fn create_sapiens_event_bus_with_handlers() -> EventBus {
    let bus = create_sapiens_event_bus();

    // Register handlers
    bus.register_handler(std::sync::Arc::new(SapiensLoggingHandler::all())).await;
    bus.register_handler(std::sync::Arc::new(EmailNotificationHandler)).await;
    bus.register_handler(std::sync::Arc::new(AuditLoggingHandler)).await;

    bus
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_event_bus_publish() {
        let bus = EventBus::with_config(EventBusConfig {
            persist_events: true,
            ..Default::default()
        });

        let event = UserDomainEvent::Created {
            user_id: "user-123".to_string(),
            occurred_at: Utc::now(),
        };

        bus.publish(event).await.unwrap();

        let history = bus.history().await;
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].event_type, "UserCreated");
    }

    #[tokio::test]
    async fn test_event_bus_subscribe() {
        let bus = EventBus::new();
        let mut rx = bus.subscribe();

        let event = UserDomainEvent::Created {
            user_id: "user-123".to_string(),
            occurred_at: Utc::now(),
        };

        bus.publish(event).await.unwrap();

        let envelope = rx.recv().await.unwrap();
        assert_eq!(envelope.event_type, "UserCreated");
    }

    #[tokio::test]
    async fn test_events_for_aggregate() {
        let bus = EventBus::with_config(EventBusConfig {
            persist_events: true,
            ..Default::default()
        });

        let user_id = "user-123";

        bus.publish(UserDomainEvent::Created {
            user_id: user_id.to_string(),
            occurred_at: Utc::now(),
        }).await.unwrap();

        bus.publish(UserDomainEvent::ProfileUpdated {
            user_id: user_id.to_string(),
            occurred_at: Utc::now(),
        }).await.unwrap();

        bus.publish(UserDomainEvent::Created {
            user_id: "other-user".to_string(),
            occurred_at: Utc::now(),
        }).await.unwrap();

        let events = bus.events_for_aggregate(user_id).await;
        assert_eq!(events.len(), 2);
    }
}

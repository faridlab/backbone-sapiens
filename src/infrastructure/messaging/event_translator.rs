//! Domain-to-Integration Event Translator
//!
//! This module translates Sapiens domain events into integration events
//! that can be consumed by other bounded contexts.
//!
//! # Architecture
//!
//! ```text
//! UserAggregate.take_events()
//!       │
//!       ▼
//! Domain Event Bus (typed)
//!       │
//!       ▼
//! SapiensIntegrationEventPublisher (this)
//!       │
//!       ▼
//! Integration Event Bus (type-erased)
//!       │
//!       ▼
//! Other Modules (Postman, Bucket, etc.)
//! ```
//!
//! # Usage
//!
//! ```rust,ignore
//! // During module initialization:
//! let integration_bus = Arc::new(IntegrationEventBus::new());
//! let publisher = SapiensIntegrationEventPublisher::new(
//!     integration_bus.clone(),
//!     user_repository.clone(),
//! );
//!
//! // Register with domain event bus
//! domain_event_bus.register_handler(Arc::new(publisher)).await;
//! ```

use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{debug, warn};

use backbone_messaging::{
    EventEnvelope, EventError, EventHandler,
    IntegrationEvent,
    IntegrationEventBus,
};

use crate::domain::entity::UserDomainEvent;
use super::integration_events::*;

/// Translates Sapiens domain events to integration events
///
/// This handler listens to domain events from the Sapiens module's
/// internal event bus and publishes corresponding integration events
/// to the cross-module integration bus.
pub struct SapiensIntegrationEventPublisher {
    /// The integration event bus for cross-module communication
    integration_bus: Arc<IntegrationEventBus>,
}

impl SapiensIntegrationEventPublisher {
    /// Create a new integration event publisher
    ///
    /// # Arguments
    /// * `integration_bus` - The shared integration event bus
    pub fn new(integration_bus: Arc<IntegrationEventBus>) -> Self {
        Self { integration_bus }
    }

    /// Translate and publish a UserCreated event
    async fn publish_user_created(
        &self,
        user_id: &str,
        correlation_id: Option<String>,
        occurred_at: chrono::DateTime<Utc>,
    ) -> Result<(), EventError> {
        // Note: In a real implementation, we would fetch user details from repository
        // For now, we publish with minimal info and let consumers fetch more if needed
        let event = UserCreatedIntegrationEvent {
            user_id: user_id.to_string(),
            email: String::new(), // Would be fetched from repository
            username: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            display_name: None,
            occurred_at,
            correlation_id,
        };

        debug!(
            user_id = %user_id,
            event_type = %event.event_type(),
            "Publishing UserCreated integration event"
        );

        self.integration_bus.publish(event).await
    }

    /// Translate and publish a UserEmailVerified event
    async fn publish_email_verified(
        &self,
        user_id: &str,
        correlation_id: Option<String>,
        occurred_at: chrono::DateTime<Utc>,
    ) -> Result<(), EventError> {
        let event = UserEmailVerifiedIntegrationEvent {
            user_id: user_id.to_string(),
            email: String::new(), // Would be fetched from repository
            occurred_at,
            correlation_id,
        };

        debug!(
            user_id = %user_id,
            event_type = %event.event_type(),
            "Publishing UserEmailVerified integration event"
        );

        self.integration_bus.publish(event).await
    }

    /// Translate and publish a UserPasswordChanged event
    async fn publish_password_changed(
        &self,
        user_id: &str,
        correlation_id: Option<String>,
        occurred_at: chrono::DateTime<Utc>,
    ) -> Result<(), EventError> {
        let event = UserPasswordChangedIntegrationEvent {
            user_id: user_id.to_string(),
            occurred_at,
            require_reauth: true, // Always require re-authentication after password change
            correlation_id,
        };

        debug!(
            user_id = %user_id,
            event_type = %event.event_type(),
            "Publishing UserPasswordChanged integration event"
        );

        self.integration_bus.publish(event).await
    }

    /// Translate and publish a UserDeactivated event
    async fn publish_user_deactivated(
        &self,
        user_id: &str,
        reason: &str,
        correlation_id: Option<String>,
        occurred_at: chrono::DateTime<Utc>,
    ) -> Result<(), EventError> {
        let event = UserDeactivatedIntegrationEvent {
            user_id: user_id.to_string(),
            reason: reason.to_string(),
            occurred_at,
            correlation_id,
        };

        debug!(
            user_id = %user_id,
            event_type = %event.event_type(),
            "Publishing UserDeactivated integration event"
        );

        self.integration_bus.publish(event).await
    }

    /// Translate and publish a UserActivated event
    async fn publish_user_activated(
        &self,
        user_id: &str,
        correlation_id: Option<String>,
        occurred_at: chrono::DateTime<Utc>,
    ) -> Result<(), EventError> {
        let event = UserActivatedIntegrationEvent {
            user_id: user_id.to_string(),
            occurred_at,
            correlation_id,
        };

        debug!(
            user_id = %user_id,
            event_type = %event.event_type(),
            "Publishing UserActivated integration event"
        );

        self.integration_bus.publish(event).await
    }

    /// Translate and publish a UserSuspended event
    async fn publish_user_suspended(
        &self,
        user_id: &str,
        reason: &str,
        correlation_id: Option<String>,
        occurred_at: chrono::DateTime<Utc>,
    ) -> Result<(), EventError> {
        let event = UserSuspendedIntegrationEvent {
            user_id: user_id.to_string(),
            reason: reason.to_string(),
            occurred_at,
            correlation_id,
        };

        debug!(
            user_id = %user_id,
            event_type = %event.event_type(),
            "Publishing UserSuspended integration event"
        );

        self.integration_bus.publish(event).await
    }

    /// Translate and publish a UserDeleted event
    async fn publish_user_deleted(
        &self,
        user_id: &str,
        correlation_id: Option<String>,
        occurred_at: chrono::DateTime<Utc>,
    ) -> Result<(), EventError> {
        let event = UserDeletedIntegrationEvent {
            user_id: user_id.to_string(),
            occurred_at,
            correlation_id,
        };

        debug!(
            user_id = %user_id,
            event_type = %event.event_type(),
            "Publishing UserDeleted integration event"
        );

        self.integration_bus.publish(event).await
    }

    /// Translate and publish a UserAccountLocked event
    async fn publish_account_locked(
        &self,
        user_id: &str,
        reason: &str,
        locked_until: chrono::DateTime<Utc>,
        correlation_id: Option<String>,
        occurred_at: chrono::DateTime<Utc>,
    ) -> Result<(), EventError> {
        let event = UserAccountLockedIntegrationEvent {
            user_id: user_id.to_string(),
            reason: reason.to_string(),
            locked_until,
            occurred_at,
            correlation_id,
        };

        debug!(
            user_id = %user_id,
            event_type = %event.event_type(),
            "Publishing UserAccountLocked integration event"
        );

        self.integration_bus.publish(event).await
    }
}

#[async_trait]
impl EventHandler<UserDomainEvent> for SapiensIntegrationEventPublisher {
    async fn handle(&self, envelope: EventEnvelope<UserDomainEvent>) -> Result<(), EventError> {
        let Some(event) = envelope.event() else {
            warn!("Received envelope without event payload");
            return Ok(());
        };

        let correlation_id = envelope.correlation_id.clone();

        // Translate domain events to integration events
        match event {
            UserDomainEvent::Created { user_id, occurred_at } => {
                self.publish_user_created(user_id, correlation_id, *occurred_at).await?;
            }
            UserDomainEvent::EmailVerified { user_id, occurred_at } => {
                self.publish_email_verified(user_id, correlation_id, *occurred_at).await?;
            }
            UserDomainEvent::PasswordChanged { user_id, occurred_at } => {
                self.publish_password_changed(user_id, correlation_id, *occurred_at).await?;
            }
            UserDomainEvent::Deactivated { user_id, reason, occurred_at } => {
                self.publish_user_deactivated(user_id, reason, correlation_id, *occurred_at).await?;
            }
            UserDomainEvent::Activated { user_id, occurred_at } => {
                self.publish_user_activated(user_id, correlation_id, *occurred_at).await?;
            }
            UserDomainEvent::Suspended { user_id, reason, occurred_at } => {
                self.publish_user_suspended(user_id, reason, correlation_id, *occurred_at).await?;
            }
            UserDomainEvent::Deleted { user_id, occurred_at } => {
                self.publish_user_deleted(user_id, correlation_id, *occurred_at).await?;
            }
            UserDomainEvent::AccountLocked { user_id, reason, locked_until, occurred_at } => {
                self.publish_account_locked(user_id, reason, *locked_until, correlation_id, *occurred_at).await?;
            }
            // Events that don't need to be published to other contexts
            UserDomainEvent::ProfileUpdated { .. }
            | UserDomainEvent::EmailChanged { .. }
            | UserDomainEvent::MfaEnabled { .. }
            | UserDomainEvent::MfaDisabled { .. }
            | UserDomainEvent::AccountUnlocked { .. }
            | UserDomainEvent::Restored { .. }
            | UserDomainEvent::LoggedIn { .. }
            | UserDomainEvent::LoginFailed { .. }
            | UserDomainEvent::LoggedOut { .. } => {
                debug!(
                    event_type = %envelope.event_type,
                    "Domain event does not require integration event"
                );
            }
        }

        Ok(())
    }

    fn event_types(&self) -> Vec<&'static str> {
        vec![
            "UserCreated",
            "UserEmailVerified",
            "UserPasswordChanged",
            "UserDeactivated",
            "UserActivated",
            "UserSuspended",
            "UserDeleted",
            "UserAccountLocked",
        ]
    }

    fn name(&self) -> &'static str {
        "SapiensIntegrationEventPublisher"
    }

    fn should_retry(&self) -> bool {
        true // Integration events are important, retry on failure
    }

    fn max_retries(&self) -> u32 {
        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backbone_messaging::{EventBus, EventBusConfig};

    #[tokio::test]
    async fn test_publisher_creation() {
        let integration_bus = Arc::new(IntegrationEventBus::new());
        let publisher = SapiensIntegrationEventPublisher::new(integration_bus);

        assert_eq!(publisher.name(), "SapiensIntegrationEventPublisher");
        assert!(publisher.should_retry());
        assert_eq!(publisher.max_retries(), 3);
    }

    #[tokio::test]
    async fn test_event_types() {
        let integration_bus = Arc::new(IntegrationEventBus::new());
        let publisher = SapiensIntegrationEventPublisher::new(integration_bus);

        let types = publisher.event_types();
        assert!(types.contains(&"UserCreated"));
        assert!(types.contains(&"UserPasswordChanged"));
        assert!(types.contains(&"UserDeactivated"));
    }

    #[tokio::test]
    async fn test_user_created_translation() {
        let integration_bus = Arc::new(IntegrationEventBus::new());
        let publisher = Arc::new(SapiensIntegrationEventPublisher::new(integration_bus.clone()));

        // Create domain event bus and register publisher
        let domain_bus = EventBus::<UserDomainEvent>::with_config(EventBusConfig::with_persistence());
        domain_bus.register_handler(publisher).await;

        // Subscribe to integration events
        let mut rx = integration_bus.subscribe();

        // Publish domain event
        let domain_event = UserDomainEvent::Created {
            user_id: "user-123".to_string(),
            occurred_at: Utc::now(),
        };
        domain_bus.publish(domain_event).await.unwrap();

        // Should receive integration event
        let envelope = rx.recv().await.unwrap();
        assert_eq!(envelope.event_type, "sapiens.user.created");
        assert_eq!(envelope.source_context, "sapiens");
        assert_eq!(envelope.aggregate_id, "user-123");
    }

    #[tokio::test]
    async fn test_password_changed_translation() {
        let integration_bus = Arc::new(IntegrationEventBus::new());
        let publisher = Arc::new(SapiensIntegrationEventPublisher::new(integration_bus.clone()));

        let domain_bus = EventBus::<UserDomainEvent>::with_config(EventBusConfig::with_persistence());
        domain_bus.register_handler(publisher).await;

        let mut rx = integration_bus.subscribe();

        let domain_event = UserDomainEvent::PasswordChanged {
            user_id: "user-456".to_string(),
            occurred_at: Utc::now(),
        };
        domain_bus.publish(domain_event).await.unwrap();

        let envelope = rx.recv().await.unwrap();
        assert_eq!(envelope.event_type, "sapiens.user.password_changed");

        // Verify payload
        let event: UserPasswordChangedIntegrationEvent = envelope.deserialize().unwrap();
        assert_eq!(event.user_id, "user-456");
        assert!(event.require_reauth);
    }

    #[tokio::test]
    async fn test_non_published_events() {
        let integration_bus = Arc::new(IntegrationEventBus::new());
        let publisher = Arc::new(SapiensIntegrationEventPublisher::new(integration_bus.clone()));

        let domain_bus = EventBus::<UserDomainEvent>::with_config(EventBusConfig::with_persistence());
        domain_bus.register_handler(publisher).await;

        // ProfileUpdated should not generate integration event
        let domain_event = UserDomainEvent::ProfileUpdated {
            user_id: "user-789".to_string(),
            occurred_at: Utc::now(),
        };
        domain_bus.publish(domain_event).await.unwrap();

        // Integration bus history should be empty (no event published)
        let history = integration_bus.history().await;
        assert!(history.is_empty());
    }
}

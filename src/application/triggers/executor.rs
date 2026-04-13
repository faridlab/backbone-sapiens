//! Trigger Executor
//!
//! Executes side effects from state transitions and entity lifecycle events.

use std::sync::Arc;
use std::collections::HashMap;
use tracing::{info, warn, error, instrument};

use crate::domain::state_machine::{SideEffect, LogSeverity, TransitionResult};

/// Trait for email service integration
#[async_trait]
pub trait EmailService: Send + Sync {
    /// Send an email using a template
    async fn send_template(&self, to: &str, template: &str, data: HashMap<String, String>) -> anyhow::Result<()>;
}

/// Trait for notification service integration
#[async_trait]
pub trait NotificationService: Send + Sync {
    /// Send a notification to a channel
    async fn notify(&self, channel: &str, message: &str) -> anyhow::Result<()>;
}

/// Trait for audit logging
#[async_trait]
pub trait AuditLogger: Send + Sync {
    /// Log an audit event
    async fn log(&self, event: &str, severity: LogSeverity, data: HashMap<String, String>) -> anyhow::Result<()>;
}

/// Trait for session management
#[async_trait]
pub trait SessionManager: Send + Sync {
    /// Invalidate all sessions for a user
    async fn invalidate_user_sessions(&self, user_id: &str) -> anyhow::Result<u64>;
}

/// Trait for entity updates
#[async_trait]
pub trait EntityUpdater: Send + Sync {
    /// Update a field on an entity
    async fn update_field(&self, entity_type: &str, entity_id: &str, field: &str, value: serde_json::Value) -> anyhow::Result<()>;
}

/// Trait for domain event publishing
#[async_trait]
pub trait EventPublisher: Send + Sync {
    /// Publish a domain event
    async fn publish(&self, event_type: &str, payload: serde_json::Value) -> anyhow::Result<()>;
}

/// Result of executing side effects
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub successful: usize,
    pub failed: usize,
    pub errors: Vec<(String, String)>,
}

impl ExecutionResult {
    pub fn new() -> Self {
        Self {
            successful: 0,
            failed: 0,
            errors: vec![],
        }
    }

    pub fn success(&mut self) {
        self.successful += 1;
    }

    pub fn failure(&mut self, effect: String, error: String) {
        self.failed += 1;
        self.errors.push((effect, error));
    }

    pub fn is_success(&self) -> bool {
        self.failed == 0
    }
}

impl Default for ExecutionResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Side effect executor
pub struct SideEffectExecutor {
    email_service: Option<Arc<dyn EmailService>>,
    notification_service: Option<Arc<dyn NotificationService>>,
    audit_logger: Option<Arc<dyn AuditLogger>>,
    session_manager: Option<Arc<dyn SessionManager>>,
    entity_updater: Option<Arc<dyn EntityUpdater>>,
    event_publisher: Option<Arc<dyn EventPublisher>>,
}

impl SideEffectExecutor {
    pub fn new() -> Self {
        Self {
            email_service: None,
            notification_service: None,
            audit_logger: None,
            session_manager: None,
            entity_updater: None,
            event_publisher: None,
        }
    }

    pub fn with_email_service(mut self, service: Arc<dyn EmailService>) -> Self {
        self.email_service = Some(service);
        self
    }

    pub fn with_notification_service(mut self, service: Arc<dyn NotificationService>) -> Self {
        self.notification_service = Some(service);
        self
    }

    pub fn with_audit_logger(mut self, logger: Arc<dyn AuditLogger>) -> Self {
        self.audit_logger = Some(logger);
        self
    }

    pub fn with_session_manager(mut self, manager: Arc<dyn SessionManager>) -> Self {
        self.session_manager = Some(manager);
        self
    }

    pub fn with_entity_updater(mut self, updater: Arc<dyn EntityUpdater>) -> Self {
        self.entity_updater = Some(updater);
        self
    }

    pub fn with_event_publisher(mut self, publisher: Arc<dyn EventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    /// Execute a list of side effects
    #[instrument(skip(self, effects), fields(effect_count = effects.len()))]
    pub async fn execute(&self, effects: Vec<SideEffect>) -> ExecutionResult {
        let mut result = ExecutionResult::new();

        for effect in effects {
            match self.execute_single(&effect).await {
                Ok(()) => {
                    info!(?effect, "Side effect executed successfully");
                    result.success();
                }
                Err(e) => {
                    error!(?effect, error = %e, "Failed to execute side effect");
                    result.failure(format!("{:?}", effect), e.to_string());
                }
            }
        }

        result
    }

    /// Execute a single side effect
    async fn execute_single(&self, effect: &SideEffect) -> anyhow::Result<()> {
        match effect {
            SideEffect::SendEmail { template, to } => {
                if let Some(service) = &self.email_service {
                    service.send_template(to, template, HashMap::new()).await?;
                } else {
                    warn!("Email service not configured, skipping email to {}", to);
                }
            }

            SideEffect::Log { event, severity, data } => {
                if let Some(logger) = &self.audit_logger {
                    logger.log(event, *severity, data.clone()).await?;
                } else {
                    // Fallback to tracing
                    match severity {
                        LogSeverity::Info => info!(event = %event, ?data, "Audit log"),
                        LogSeverity::Warning => warn!(event = %event, ?data, "Audit log"),
                        LogSeverity::Error => error!(event = %event, ?data, "Audit log"),
                    }
                }
            }

            SideEffect::EmitEvent { event_type, payload } => {
                if let Some(publisher) = &self.event_publisher {
                    publisher.publish(event_type, payload.clone()).await?;
                } else {
                    info!(event_type = %event_type, "Event emitted (no publisher configured)");
                }
            }

            SideEffect::InvalidateSessions { user_id } => {
                if let Some(manager) = &self.session_manager {
                    let count = manager.invalidate_user_sessions(user_id).await?;
                    info!(user_id = %user_id, count = count, "Sessions invalidated");
                } else {
                    warn!("Session manager not configured, skipping session invalidation for {}", user_id);
                }
            }

            SideEffect::Notify { channel, message } => {
                if let Some(service) = &self.notification_service {
                    service.notify(channel, message).await?;
                } else {
                    info!(channel = %channel, message = %message, "Notification (no service configured)");
                }
            }

            SideEffect::UpdateField { field, value } => {
                if let Some(updater) = &self.entity_updater {
                    // Note: This requires context about which entity to update
                    // In practice, this would be called with entity context
                    warn!("UpdateField effect requires entity context - skipping");
                } else {
                    warn!("Entity updater not configured");
                }
            }
        }

        Ok(())
    }

    /// Execute side effects from a transition result
    pub async fn execute_transition<S>(&self, result: &TransitionResult<S>) -> ExecutionResult
    where
        S: std::fmt::Debug + Clone + std::fmt::Display,
    {
        info!(
            from = %result.from,
            to = %result.to,
            transition = %result.transition,
            effect_count = result.side_effects.len(),
            "Executing transition side effects"
        );

        self.execute(result.side_effects.clone()).await
    }
}

impl Default for SideEffectExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// In-memory audit logger for development/testing
#[derive(Debug, Default)]
pub struct InMemoryAuditLogger {
    logs: std::sync::RwLock<Vec<AuditLogEntry>>,
}

#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    pub event: String,
    pub severity: LogSeverity,
    pub data: HashMap<String, String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl InMemoryAuditLogger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn logs(&self) -> Vec<AuditLogEntry> {
        self.logs.read().unwrap().clone()
    }

    pub fn clear(&self) {
        self.logs.write().unwrap().clear();
    }
}

#[async_trait]
impl AuditLogger for InMemoryAuditLogger {
    async fn log(&self, event: &str, severity: LogSeverity, data: HashMap<String, String>) -> anyhow::Result<()> {
        let entry = AuditLogEntry {
            event: event.to_string(),
            severity,
            data,
            timestamp: chrono::Utc::now(),
        };
        self.logs.write().unwrap().push(entry);
        Ok(())
    }
}

/// Console-based notification service for development
#[derive(Debug, Default)]
pub struct ConsoleNotificationService;

#[async_trait]
impl NotificationService for ConsoleNotificationService {
    async fn notify(&self, channel: &str, message: &str) -> anyhow::Result<()> {
        println!("[NOTIFICATION:{}] {}", channel, message);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::state_machine::UserState;

    #[tokio::test]
    async fn test_execute_log_effect() {
        let logger = Arc::new(InMemoryAuditLogger::new());
        let executor = SideEffectExecutor::new()
            .with_audit_logger(logger.clone());

        let effects = vec![
            SideEffect::Log {
                event: "test.event".to_string(),
                severity: LogSeverity::Info,
                data: HashMap::from([("key".to_string(), "value".to_string())]),
            },
        ];

        let result = executor.execute(effects).await;
        assert!(result.is_success());
        assert_eq!(result.successful, 1);

        let logs = logger.logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].event, "test.event");
    }

    #[tokio::test]
    async fn test_execute_transition_effects() {
        let logger = Arc::new(InMemoryAuditLogger::new());
        let notifier = Arc::new(ConsoleNotificationService);
        let executor = SideEffectExecutor::new()
            .with_audit_logger(logger.clone())
            .with_notification_service(notifier);

        let transition = TransitionResult {
            from: UserState::PendingVerification,
            to: UserState::Active,
            transition: "verify".to_string(),
            side_effects: vec![
                SideEffect::Log {
                    event: "user.activated".to_string(),
                    severity: LogSeverity::Info,
                    data: HashMap::new(),
                },
                SideEffect::Notify {
                    channel: "admin".to_string(),
                    message: "User activated".to_string(),
                },
            ],
        };

        let result = executor.execute_transition(&transition).await;
        assert!(result.is_success());
        assert_eq!(result.successful, 2);
    }
}

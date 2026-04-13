//! AuditLog trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::AuditLog;

pub type AuditLogTriggerEvent      = TriggerEvent;
pub type AuditLogTriggerContext    = TriggerContext<AuditLog>;
pub type AuditLogTriggerContextMut = TriggerContextMut<AuditLog>;
pub type AuditLogActionExecutor    = ActionExecutor;
pub type AuditLogTriggerRegistry   = TriggerRegistry<AuditLog>;
pub type AuditLogTriggerHandlerObj = dyn TriggerHandler<TriggerContext<AuditLog>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct AuditLogAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::AuditLogEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<AuditLogActionExecutor>>,
}

impl AuditLogAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::AuditLogEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<AuditLogActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<AuditLogTriggerContext, AuditLogTriggerEvent> for AuditLogAfterCreateHandler1 {
    fn events(&self) -> Vec<AuditLogTriggerEvent> {
        vec![AuditLogTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &AuditLogTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Send notification
        if let Some(executor) = &self.action_executor {
            executor.notify(ctx, "default").await?;
        }
        // Unknown action type 'trigger': check_suspicious_activity
        Ok(())
    }
}

/// AfterCreate handler
pub struct AuditLogAfterCreateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::AuditLogEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<AuditLogActionExecutor>>,
}

impl AuditLogAfterCreateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::AuditLogEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<AuditLogActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<AuditLogTriggerContext, AuditLogTriggerEvent> for AuditLogAfterCreateHandler2 {
    fn events(&self) -> Vec<AuditLogTriggerEvent> {
        vec![AuditLogTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &AuditLogTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'trigger': archive_old_logs
        Ok(())
    }
}

/// AfterCreate handler
pub struct AuditLogAfterCreateHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::AuditLogEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<AuditLogActionExecutor>>,
}

impl AuditLogAfterCreateHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::AuditLogEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<AuditLogActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<AuditLogTriggerContext, AuditLogTriggerEvent> for AuditLogAfterCreateHandler3 {
    fn events(&self) -> Vec<AuditLogTriggerEvent> {
        vec![AuditLogTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &AuditLogTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'trigger': cleanup_archived_logs
        Ok(())
    }
}

/// Action executor for AuditLog triggers

pub fn audit_log_trigger_registry() -> AuditLogTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(AuditLogAfterCreateHandler1::new()));
        r.register(Arc::new(AuditLogAfterCreateHandler2::new()));
        r.register(Arc::new(AuditLogAfterCreateHandler3::new()));
    })
}

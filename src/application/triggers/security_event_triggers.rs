//! SecurityEvent trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::SecurityEvent;

pub type SecurityEventTriggerEvent      = TriggerEvent;
pub type SecurityEventTriggerContext    = TriggerContext<SecurityEvent>;
pub type SecurityEventTriggerContextMut = TriggerContextMut<SecurityEvent>;
pub type SecurityEventActionExecutor    = ActionExecutor;
pub type SecurityEventTriggerRegistry   = TriggerRegistry<SecurityEvent>;
pub type SecurityEventTriggerHandlerObj = dyn TriggerHandler<TriggerContext<SecurityEvent>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct SecurityEventAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::SecurityEventEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<SecurityEventActionExecutor>>,
}

impl SecurityEventAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::SecurityEventEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<SecurityEventActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<SecurityEventTriggerContext, SecurityEventTriggerEvent> for SecurityEventAfterCreateHandler1 {
    fn events(&self) -> Vec<SecurityEventTriggerEvent> {
        vec![SecurityEventTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &SecurityEventTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit securityeventcreatedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_created(ctx.entity.clone(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct SecurityEventAfterCreateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::SecurityEventEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<SecurityEventActionExecutor>>,
}

impl SecurityEventAfterCreateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::SecurityEventEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<SecurityEventActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<SecurityEventTriggerContext, SecurityEventTriggerEvent> for SecurityEventAfterCreateHandler2 {
    fn events(&self) -> Vec<SecurityEventTriggerEvent> {
        vec![SecurityEventTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &SecurityEventTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit securityeventresolvedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: securityeventresolvedevent
            // <<< CUSTOM EMIT: securityeventresolvedevent >>>
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct SecurityEventAfterCreateHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::SecurityEventEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<SecurityEventActionExecutor>>,
}

impl SecurityEventAfterCreateHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::SecurityEventEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<SecurityEventActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<SecurityEventTriggerContext, SecurityEventTriggerEvent> for SecurityEventAfterCreateHandler3 {
    fn events(&self) -> Vec<SecurityEventTriggerEvent> {
        vec![SecurityEventTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &SecurityEventTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'trigger': archive_resolved_events
        // Unknown action type 'trigger': delete_old_events
        Ok(())
    }
}

/// AfterCreate handler
pub struct SecurityEventAfterCreateHandler4 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::SecurityEventEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<SecurityEventActionExecutor>>,
}

impl SecurityEventAfterCreateHandler4 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::SecurityEventEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<SecurityEventActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<SecurityEventTriggerContext, SecurityEventTriggerEvent> for SecurityEventAfterCreateHandler4 {
    fn events(&self) -> Vec<SecurityEventTriggerEvent> {
        vec![SecurityEventTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &SecurityEventTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'trigger': detect_attack_patterns
        Ok(())
    }
}

// State machine trigger handlers

/// Handler for entering false state
pub struct SecurityEventOnEnterFalseHandler {
    pub event_publisher: Option<Arc<crate::domain::event::SecurityEventEventPublisher>>,
}

impl SecurityEventOnEnterFalseHandler {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<SecurityEventTriggerContext, SecurityEventTriggerEvent> for SecurityEventOnEnterFalseHandler {
    fn events(&self) -> Vec<SecurityEventTriggerEvent> {
        vec![SecurityEventTriggerEvent::OnEnterState("false".to_string())]
    }

    async fn handle(&self, ctx: &SecurityEventTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit securityeventcreatedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_created(ctx.entity.clone(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// Handler for entering true state
pub struct SecurityEventOnEnterTrueHandler {
    pub event_publisher: Option<Arc<crate::domain::event::SecurityEventEventPublisher>>,
}

impl SecurityEventOnEnterTrueHandler {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<SecurityEventTriggerContext, SecurityEventTriggerEvent> for SecurityEventOnEnterTrueHandler {
    fn events(&self) -> Vec<SecurityEventTriggerEvent> {
        vec![SecurityEventTriggerEvent::OnEnterState("true".to_string())]
    }

    async fn handle(&self, ctx: &SecurityEventTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Set resolved_at to current timestamp
        // <<< CUSTOM SET: resolved_at = now >>>
        // ctx.entity.resolved_at = Some(chrono::Utc::now();
        // Note: Add closing parenthesis if using Some()
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit securityeventresolvedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: securityeventresolvedevent
            // <<< CUSTOM EMIT: securityeventresolvedevent >>>
        }
        Ok(())
    }
}

/// Action executor for SecurityEvent triggers

pub fn security_event_trigger_registry() -> SecurityEventTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(SecurityEventAfterCreateHandler1::new()));
        r.register(Arc::new(SecurityEventAfterCreateHandler2::new()));
        r.register(Arc::new(SecurityEventAfterCreateHandler3::new()));
        r.register(Arc::new(SecurityEventAfterCreateHandler4::new()));
        r.register(Arc::new(SecurityEventOnEnterFalseHandler::new()));
        r.register(Arc::new(SecurityEventOnEnterTrueHandler::new()));
    })
}

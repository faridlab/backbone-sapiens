//! Session trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::Session;

pub type SessionTriggerEvent      = TriggerEvent;
pub type SessionTriggerContext    = TriggerContext<Session>;
pub type SessionTriggerContextMut = TriggerContextMut<Session>;
pub type SessionActionExecutor    = ActionExecutor;
pub type SessionTriggerRegistry   = TriggerRegistry<Session>;
pub type SessionTriggerHandlerObj = dyn TriggerHandler<TriggerContext<Session>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct SessionAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::SessionEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<SessionActionExecutor>>,
}

impl SessionAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::SessionEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<SessionActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<SessionTriggerContext, SessionTriggerEvent> for SessionAfterCreateHandler1 {
    fn events(&self) -> Vec<SessionTriggerEvent> {
        vec![SessionTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &SessionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit sessioncreatedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_created(ctx.entity.clone(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct SessionAfterCreateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::SessionEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<SessionActionExecutor>>,
}

impl SessionAfterCreateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::SessionEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<SessionActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<SessionTriggerContext, SessionTriggerEvent> for SessionAfterCreateHandler2 {
    fn events(&self) -> Vec<SessionTriggerEvent> {
        vec![SessionTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &SessionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        Ok(())
    }
}

/// AfterCreate handler
pub struct SessionAfterCreateHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::SessionEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<SessionActionExecutor>>,
}

impl SessionAfterCreateHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::SessionEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<SessionActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<SessionTriggerContext, SessionTriggerEvent> for SessionAfterCreateHandler3 {
    fn events(&self) -> Vec<SessionTriggerEvent> {
        vec![SessionTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &SessionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit sessionrevokedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: sessionrevokedevent
            // <<< CUSTOM EMIT: sessionrevokedevent >>>
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct SessionAfterCreateHandler4 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::SessionEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<SessionActionExecutor>>,
}

impl SessionAfterCreateHandler4 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::SessionEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<SessionActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<SessionTriggerContext, SessionTriggerEvent> for SessionAfterCreateHandler4 {
    fn events(&self) -> Vec<SessionTriggerEvent> {
        vec![SessionTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &SessionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'trigger': expire_sessions_batch
        Ok(())
    }
}

// State machine trigger handlers

/// Handler for entering active state
pub struct SessionOnEnterActiveHandler {}

impl SessionOnEnterActiveHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<SessionTriggerContext, SessionTriggerEvent> for SessionOnEnterActiveHandler {
    fn events(&self) -> Vec<SessionTriggerEvent> {
        vec![SessionTriggerEvent::OnEnterState("active".to_string())]
    }

    async fn handle(&self, ctx: &SessionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Handler for entering revoked state
pub struct SessionOnEnterRevokedHandler {}

impl SessionOnEnterRevokedHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<SessionTriggerContext, SessionTriggerEvent> for SessionOnEnterRevokedHandler {
    fn events(&self) -> Vec<SessionTriggerEvent> {
        vec![SessionTriggerEvent::OnEnterState("revoked".to_string())]
    }

    async fn handle(&self, ctx: &SessionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Set revoked_at to current timestamp
        // <<< CUSTOM SET: revoked_at = now >>>
        // ctx.entity.revoked_at = Some(chrono::Utc::now();
        // Note: Add closing parenthesis if using Some()
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Action executor for Session triggers

pub fn session_trigger_registry() -> SessionTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(SessionAfterCreateHandler1::new()));
        r.register(Arc::new(SessionAfterCreateHandler2::new()));
        r.register(Arc::new(SessionAfterCreateHandler3::new()));
        r.register(Arc::new(SessionAfterCreateHandler4::new()));
        r.register(Arc::new(SessionOnEnterActiveHandler::new()));
        r.register(Arc::new(SessionOnEnterRevokedHandler::new()));
    })
}

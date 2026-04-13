//! SessionLimit trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::SessionLimit;

pub type SessionLimitTriggerEvent      = TriggerEvent;
pub type SessionLimitTriggerContext    = TriggerContext<SessionLimit>;
pub type SessionLimitTriggerContextMut = TriggerContextMut<SessionLimit>;
pub type SessionLimitActionExecutor    = ActionExecutor;
pub type SessionLimitTriggerRegistry   = TriggerRegistry<SessionLimit>;
pub type SessionLimitTriggerHandlerObj = dyn TriggerHandler<TriggerContext<SessionLimit>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct SessionLimitAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::SessionLimitEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<SessionLimitActionExecutor>>,
}

impl SessionLimitAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::SessionLimitEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<SessionLimitActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<SessionLimitTriggerContext, SessionLimitTriggerEvent> for SessionLimitAfterCreateHandler1 {
    fn events(&self) -> Vec<SessionLimitTriggerEvent> {
        vec![SessionLimitTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &SessionLimitTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// AfterCreate handler
pub struct SessionLimitAfterCreateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::SessionLimitEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<SessionLimitActionExecutor>>,
}

impl SessionLimitAfterCreateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::SessionLimitEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<SessionLimitActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<SessionLimitTriggerContext, SessionLimitTriggerEvent> for SessionLimitAfterCreateHandler2 {
    fn events(&self) -> Vec<SessionLimitTriggerEvent> {
        vec![SessionLimitTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &SessionLimitTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Unknown action type 'trigger': revoke_excess_sessions
        Ok(())
    }
}

/// AfterCreate handler
pub struct SessionLimitAfterCreateHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::SessionLimitEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<SessionLimitActionExecutor>>,
}

impl SessionLimitAfterCreateHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::SessionLimitEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<SessionLimitActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<SessionLimitTriggerContext, SessionLimitTriggerEvent> for SessionLimitAfterCreateHandler3 {
    fn events(&self) -> Vec<SessionLimitTriggerEvent> {
        vec![SessionLimitTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &SessionLimitTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Unknown action type 'trigger': revoke_excess_sessions
        Ok(())
    }
}

/// AfterDelete handler
pub struct SessionLimitAfterDeleteHandler4 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::SessionLimitEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<SessionLimitActionExecutor>>,
}

impl SessionLimitAfterDeleteHandler4 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::SessionLimitEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<SessionLimitActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<SessionLimitTriggerContext, SessionLimitTriggerEvent> for SessionLimitAfterDeleteHandler4 {
    fn events(&self) -> Vec<SessionLimitTriggerEvent> {
        vec![SessionLimitTriggerEvent::AfterDelete]
    }

    async fn handle(&self, ctx: &SessionLimitTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// AfterCreate handler
pub struct SessionLimitAfterCreateHandler5 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::SessionLimitEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<SessionLimitActionExecutor>>,
}

impl SessionLimitAfterCreateHandler5 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::SessionLimitEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<SessionLimitActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<SessionLimitTriggerContext, SessionLimitTriggerEvent> for SessionLimitAfterCreateHandler5 {
    fn events(&self) -> Vec<SessionLimitTriggerEvent> {
        vec![SessionLimitTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &SessionLimitTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'trigger': cleanup_stale_sessions
        Ok(())
    }
}

// State machine trigger handlers

/// Handler for entering enforced state
pub struct SessionLimitOnEnterEnforcedHandler {}

impl SessionLimitOnEnterEnforcedHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<SessionLimitTriggerContext, SessionLimitTriggerEvent> for SessionLimitOnEnterEnforcedHandler {
    fn events(&self) -> Vec<SessionLimitTriggerEvent> {
        vec![SessionLimitTriggerEvent::OnEnterState("enforced".to_string())]
    }

    async fn handle(&self, ctx: &SessionLimitTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Handler for entering unenforced state
pub struct SessionLimitOnEnterUnenforcedHandler {}

impl SessionLimitOnEnterUnenforcedHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<SessionLimitTriggerContext, SessionLimitTriggerEvent> for SessionLimitOnEnterUnenforcedHandler {
    fn events(&self) -> Vec<SessionLimitTriggerEvent> {
        vec![SessionLimitTriggerEvent::OnEnterState("unenforced".to_string())]
    }

    async fn handle(&self, ctx: &SessionLimitTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Action executor for SessionLimit triggers

pub fn session_limit_trigger_registry() -> SessionLimitTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(SessionLimitAfterCreateHandler1::new()));
        r.register(Arc::new(SessionLimitAfterCreateHandler2::new()));
        r.register(Arc::new(SessionLimitAfterCreateHandler3::new()));
        r.register(Arc::new(SessionLimitAfterDeleteHandler4::new()));
        r.register(Arc::new(SessionLimitAfterCreateHandler5::new()));
        r.register(Arc::new(SessionLimitOnEnterEnforcedHandler::new()));
        r.register(Arc::new(SessionLimitOnEnterUnenforcedHandler::new()));
    })
}

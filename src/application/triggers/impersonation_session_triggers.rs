//! ImpersonationSession trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::ImpersonationSession;

pub type ImpersonationSessionTriggerEvent      = TriggerEvent;
pub type ImpersonationSessionTriggerContext    = TriggerContext<ImpersonationSession>;
pub type ImpersonationSessionTriggerContextMut = TriggerContextMut<ImpersonationSession>;
pub type ImpersonationSessionActionExecutor    = ActionExecutor;
pub type ImpersonationSessionTriggerRegistry   = TriggerRegistry<ImpersonationSession>;
pub type ImpersonationSessionTriggerHandlerObj = dyn TriggerHandler<TriggerContext<ImpersonationSession>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct ImpersonationSessionAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::ImpersonationSessionEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<ImpersonationSessionActionExecutor>>,
}

impl ImpersonationSessionAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::ImpersonationSessionEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<ImpersonationSessionActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<ImpersonationSessionTriggerContext, ImpersonationSessionTriggerEvent> for ImpersonationSessionAfterCreateHandler1 {
    fn events(&self) -> Vec<ImpersonationSessionTriggerEvent> {
        vec![ImpersonationSessionTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &ImpersonationSessionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Unknown action type 'create': auditlog
        // Send notification
        if let Some(executor) = &self.action_executor {
            executor.notify(ctx, "default").await?;
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct ImpersonationSessionAfterCreateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::ImpersonationSessionEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<ImpersonationSessionActionExecutor>>,
}

impl ImpersonationSessionAfterCreateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::ImpersonationSessionEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<ImpersonationSessionActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<ImpersonationSessionTriggerContext, ImpersonationSessionTriggerEvent> for ImpersonationSessionAfterCreateHandler2 {
    fn events(&self) -> Vec<ImpersonationSessionTriggerEvent> {
        vec![ImpersonationSessionTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &ImpersonationSessionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Unknown action type 'create': auditlog
        Ok(())
    }
}

/// AfterCreate handler
pub struct ImpersonationSessionAfterCreateHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::ImpersonationSessionEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<ImpersonationSessionActionExecutor>>,
}

impl ImpersonationSessionAfterCreateHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::ImpersonationSessionEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<ImpersonationSessionActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<ImpersonationSessionTriggerContext, ImpersonationSessionTriggerEvent> for ImpersonationSessionAfterCreateHandler3 {
    fn events(&self) -> Vec<ImpersonationSessionTriggerEvent> {
        vec![ImpersonationSessionTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &ImpersonationSessionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Send notification
        if let Some(executor) = &self.action_executor {
            executor.notify(ctx, "default").await?;
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct ImpersonationSessionAfterCreateHandler4 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::ImpersonationSessionEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<ImpersonationSessionActionExecutor>>,
}

impl ImpersonationSessionAfterCreateHandler4 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::ImpersonationSessionEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<ImpersonationSessionActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<ImpersonationSessionTriggerContext, ImpersonationSessionTriggerEvent> for ImpersonationSessionAfterCreateHandler4 {
    fn events(&self) -> Vec<ImpersonationSessionTriggerEvent> {
        vec![ImpersonationSessionTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &ImpersonationSessionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'trigger': expire_impersonation_sessions
        Ok(())
    }
}

/// AfterCreate handler
pub struct ImpersonationSessionAfterCreateHandler5 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::ImpersonationSessionEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<ImpersonationSessionActionExecutor>>,
}

impl ImpersonationSessionAfterCreateHandler5 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::ImpersonationSessionEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<ImpersonationSessionActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<ImpersonationSessionTriggerContext, ImpersonationSessionTriggerEvent> for ImpersonationSessionAfterCreateHandler5 {
    fn events(&self) -> Vec<ImpersonationSessionTriggerEvent> {
        vec![ImpersonationSessionTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &ImpersonationSessionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'trigger': notify_expiring_soon
        Ok(())
    }
}

// State machine trigger handlers

/// Handler for entering active state
pub struct ImpersonationSessionOnEnterActiveHandler {
    pub event_publisher: Option<Arc<crate::domain::event::ImpersonationSessionEventPublisher>>,
    pub action_executor: Option<Arc<ImpersonationSessionActionExecutor>>,
}

impl ImpersonationSessionOnEnterActiveHandler {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<ImpersonationSessionTriggerContext, ImpersonationSessionTriggerEvent> for ImpersonationSessionOnEnterActiveHandler {
    fn events(&self) -> Vec<ImpersonationSessionTriggerEvent> {
        vec![ImpersonationSessionTriggerEvent::OnEnterState("active".to_string())]
    }

    async fn handle(&self, ctx: &ImpersonationSessionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit impersonationstartedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: impersonationstartedevent
            // <<< CUSTOM EMIT: impersonationstartedevent >>>
        }
        // Send notification
        if let Some(executor) = &self.action_executor {
            executor.notify(ctx, "default").await?;
        }
        Ok(())
    }
}

/// Handler for entering ended state
pub struct ImpersonationSessionOnEnterEndedHandler {
    pub event_publisher: Option<Arc<crate::domain::event::ImpersonationSessionEventPublisher>>,
}

impl ImpersonationSessionOnEnterEndedHandler {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<ImpersonationSessionTriggerContext, ImpersonationSessionTriggerEvent> for ImpersonationSessionOnEnterEndedHandler {
    fn events(&self) -> Vec<ImpersonationSessionTriggerEvent> {
        vec![ImpersonationSessionTriggerEvent::OnEnterState("ended".to_string())]
    }

    async fn handle(&self, ctx: &ImpersonationSessionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Set ended_at to current timestamp
        // <<< CUSTOM SET: ended_at = now >>>
        // ctx.entity.ended_at = Some(chrono::Utc::now();
        // Note: Add closing parenthesis if using Some()
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit impersonationendedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: impersonationendedevent
            // <<< CUSTOM EMIT: impersonationendedevent >>>
        }
        Ok(())
    }
}

/// Handler for entering expired state
pub struct ImpersonationSessionOnEnterExpiredHandler {
    pub event_publisher: Option<Arc<crate::domain::event::ImpersonationSessionEventPublisher>>,
}

impl ImpersonationSessionOnEnterExpiredHandler {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<ImpersonationSessionTriggerContext, ImpersonationSessionTriggerEvent> for ImpersonationSessionOnEnterExpiredHandler {
    fn events(&self) -> Vec<ImpersonationSessionTriggerEvent> {
        vec![ImpersonationSessionTriggerEvent::OnEnterState("expired".to_string())]
    }

    async fn handle(&self, ctx: &ImpersonationSessionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Set ended_at to current timestamp
        // <<< CUSTOM SET: ended_at = now >>>
        // ctx.entity.ended_at = Some(chrono::Utc::now();
        // Note: Add closing parenthesis if using Some()
        // <<< CUSTOM SET: termination_reason = 'session expired' >>>
        // ctx.entity.termination_reason = "session expired".to_string(); // or Some(...) if optional
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit impersonationexpiredevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: impersonationexpiredevent
            // <<< CUSTOM EMIT: impersonationexpiredevent >>>
        }
        Ok(())
    }
}

/// Handler for entering terminated state
pub struct ImpersonationSessionOnEnterTerminatedHandler {
    pub event_publisher: Option<Arc<crate::domain::event::ImpersonationSessionEventPublisher>>,
}

impl ImpersonationSessionOnEnterTerminatedHandler {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<ImpersonationSessionTriggerContext, ImpersonationSessionTriggerEvent> for ImpersonationSessionOnEnterTerminatedHandler {
    fn events(&self) -> Vec<ImpersonationSessionTriggerEvent> {
        vec![ImpersonationSessionTriggerEvent::OnEnterState("terminated".to_string())]
    }

    async fn handle(&self, ctx: &ImpersonationSessionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Set ended_at to current timestamp
        // <<< CUSTOM SET: ended_at = now >>>
        // ctx.entity.ended_at = Some(chrono::Utc::now();
        // Note: Add closing parenthesis if using Some()
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit impersonationterminatedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: impersonationterminatedevent
            // <<< CUSTOM EMIT: impersonationterminatedevent >>>
        }
        Ok(())
    }
}

/// Action executor for ImpersonationSession triggers

pub fn impersonation_session_trigger_registry() -> ImpersonationSessionTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(ImpersonationSessionAfterCreateHandler1::new()));
        r.register(Arc::new(ImpersonationSessionAfterCreateHandler2::new()));
        r.register(Arc::new(ImpersonationSessionAfterCreateHandler3::new()));
        r.register(Arc::new(ImpersonationSessionAfterCreateHandler4::new()));
        r.register(Arc::new(ImpersonationSessionAfterCreateHandler5::new()));
        r.register(Arc::new(ImpersonationSessionOnEnterActiveHandler::new()));
        r.register(Arc::new(ImpersonationSessionOnEnterEndedHandler::new()));
        r.register(Arc::new(ImpersonationSessionOnEnterExpiredHandler::new()));
        r.register(Arc::new(ImpersonationSessionOnEnterTerminatedHandler::new()));
    })
}

//! PasswordResetToken trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::PasswordResetToken;

pub type PasswordResetTokenTriggerEvent      = TriggerEvent;
pub type PasswordResetTokenTriggerContext    = TriggerContext<PasswordResetToken>;
pub type PasswordResetTokenTriggerContextMut = TriggerContextMut<PasswordResetToken>;
pub type PasswordResetTokenActionExecutor    = ActionExecutor;
pub type PasswordResetTokenTriggerRegistry   = TriggerRegistry<PasswordResetToken>;
pub type PasswordResetTokenTriggerHandlerObj = dyn TriggerHandler<TriggerContext<PasswordResetToken>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct PasswordResetTokenAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::PasswordResetTokenEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<PasswordResetTokenActionExecutor>>,
}

impl PasswordResetTokenAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::PasswordResetTokenEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<PasswordResetTokenActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<PasswordResetTokenTriggerContext, PasswordResetTokenTriggerEvent> for PasswordResetTokenAfterCreateHandler1 {
    fn events(&self) -> Vec<PasswordResetTokenTriggerEvent> {
        vec![PasswordResetTokenTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &PasswordResetTokenTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Send email notification
        if let Some(executor) = &self.action_executor {
            executor.send_email(ctx, "default").await?;
        }
        // Emit passwordresetrequestedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: passwordresetrequestedevent
            // <<< CUSTOM EMIT: passwordresetrequestedevent >>>
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct PasswordResetTokenAfterCreateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::PasswordResetTokenEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<PasswordResetTokenActionExecutor>>,
}

impl PasswordResetTokenAfterCreateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::PasswordResetTokenEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<PasswordResetTokenActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<PasswordResetTokenTriggerContext, PasswordResetTokenTriggerEvent> for PasswordResetTokenAfterCreateHandler2 {
    fn events(&self) -> Vec<PasswordResetTokenTriggerEvent> {
        vec![PasswordResetTokenTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &PasswordResetTokenTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'trigger': invalidate_other_reset_tokens
        // Emit passwordresetcompletedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: passwordresetcompletedevent
            // <<< CUSTOM EMIT: passwordresetcompletedevent >>>
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct PasswordResetTokenAfterCreateHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::PasswordResetTokenEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<PasswordResetTokenActionExecutor>>,
}

impl PasswordResetTokenAfterCreateHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::PasswordResetTokenEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<PasswordResetTokenActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<PasswordResetTokenTriggerContext, PasswordResetTokenTriggerEvent> for PasswordResetTokenAfterCreateHandler3 {
    fn events(&self) -> Vec<PasswordResetTokenTriggerEvent> {
        vec![PasswordResetTokenTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &PasswordResetTokenTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'trigger': cleanup_expired_tokens
        Ok(())
    }
}

// State machine trigger handlers

/// Handler for entering valid state
pub struct PasswordResetTokenOnEnterValidHandler {}

impl PasswordResetTokenOnEnterValidHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<PasswordResetTokenTriggerContext, PasswordResetTokenTriggerEvent> for PasswordResetTokenOnEnterValidHandler {
    fn events(&self) -> Vec<PasswordResetTokenTriggerEvent> {
        vec![PasswordResetTokenTriggerEvent::OnEnterState("valid".to_string())]
    }

    async fn handle(&self, ctx: &PasswordResetTokenTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Handler for entering used state
pub struct PasswordResetTokenOnEnterUsedHandler {}

impl PasswordResetTokenOnEnterUsedHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<PasswordResetTokenTriggerContext, PasswordResetTokenTriggerEvent> for PasswordResetTokenOnEnterUsedHandler {
    fn events(&self) -> Vec<PasswordResetTokenTriggerEvent> {
        vec![PasswordResetTokenTriggerEvent::OnEnterState("used".to_string())]
    }

    async fn handle(&self, ctx: &PasswordResetTokenTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Set used_at to current timestamp
        // <<< CUSTOM SET: used_at = now >>>
        // ctx.entity.used_at = Some(chrono::Utc::now();
        // Note: Add closing parenthesis if using Some()
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Action executor for PasswordResetToken triggers

pub fn password_reset_token_trigger_registry() -> PasswordResetTokenTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(PasswordResetTokenAfterCreateHandler1::new()));
        r.register(Arc::new(PasswordResetTokenAfterCreateHandler2::new()));
        r.register(Arc::new(PasswordResetTokenAfterCreateHandler3::new()));
        r.register(Arc::new(PasswordResetTokenOnEnterValidHandler::new()));
        r.register(Arc::new(PasswordResetTokenOnEnterUsedHandler::new()));
    })
}

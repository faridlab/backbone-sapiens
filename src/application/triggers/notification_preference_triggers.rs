//! NotificationPreference trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::NotificationPreference;

pub type NotificationPreferenceTriggerEvent      = TriggerEvent;
pub type NotificationPreferenceTriggerContext    = TriggerContext<NotificationPreference>;
pub type NotificationPreferenceTriggerContextMut = TriggerContextMut<NotificationPreference>;
pub type NotificationPreferenceActionExecutor    = ActionExecutor;
pub type NotificationPreferenceTriggerRegistry   = TriggerRegistry<NotificationPreference>;
pub type NotificationPreferenceTriggerHandlerObj = dyn TriggerHandler<TriggerContext<NotificationPreference>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct NotificationPreferenceAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::NotificationPreferenceEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<NotificationPreferenceActionExecutor>>,
}

impl NotificationPreferenceAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::NotificationPreferenceEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<NotificationPreferenceActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<NotificationPreferenceTriggerContext, NotificationPreferenceTriggerEvent> for NotificationPreferenceAfterCreateHandler1 {
    fn events(&self) -> Vec<NotificationPreferenceTriggerEvent> {
        vec![NotificationPreferenceTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &NotificationPreferenceTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// AfterUpdate handler
pub struct NotificationPreferenceAfterUpdateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::NotificationPreferenceEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<NotificationPreferenceActionExecutor>>,
}

impl NotificationPreferenceAfterUpdateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::NotificationPreferenceEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<NotificationPreferenceActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<NotificationPreferenceTriggerContext, NotificationPreferenceTriggerEvent> for NotificationPreferenceAfterUpdateHandler2 {
    fn events(&self) -> Vec<NotificationPreferenceTriggerEvent> {
        vec![NotificationPreferenceTriggerEvent::AfterUpdate]
    }

    async fn handle(&self, ctx: &NotificationPreferenceTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Custom action: invalidate_notification_cache
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: invalidate_notification_cache
        // <<< CUSTOM ACTION END >>>
        Ok(())
    }
}

/// AfterCreate handler
pub struct NotificationPreferenceAfterCreateHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::NotificationPreferenceEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<NotificationPreferenceActionExecutor>>,
}

impl NotificationPreferenceAfterCreateHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::NotificationPreferenceEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<NotificationPreferenceActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<NotificationPreferenceTriggerContext, NotificationPreferenceTriggerEvent> for NotificationPreferenceAfterCreateHandler3 {
    fn events(&self) -> Vec<NotificationPreferenceTriggerEvent> {
        vec![NotificationPreferenceTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &NotificationPreferenceTriggerContext) -> anyhow::Result<()> {
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
pub struct NotificationPreferenceAfterCreateHandler4 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::NotificationPreferenceEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<NotificationPreferenceActionExecutor>>,
}

impl NotificationPreferenceAfterCreateHandler4 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::NotificationPreferenceEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<NotificationPreferenceActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<NotificationPreferenceTriggerContext, NotificationPreferenceTriggerEvent> for NotificationPreferenceAfterCreateHandler4 {
    fn events(&self) -> Vec<NotificationPreferenceTriggerEvent> {
        vec![NotificationPreferenceTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &NotificationPreferenceTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

// State machine trigger handlers

/// Handler for entering enabled state
pub struct NotificationPreferenceOnEnterEnabledHandler {}

impl NotificationPreferenceOnEnterEnabledHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<NotificationPreferenceTriggerContext, NotificationPreferenceTriggerEvent> for NotificationPreferenceOnEnterEnabledHandler {
    fn events(&self) -> Vec<NotificationPreferenceTriggerEvent> {
        vec![NotificationPreferenceTriggerEvent::OnEnterState("enabled".to_string())]
    }

    async fn handle(&self, ctx: &NotificationPreferenceTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Handler for entering disabled state
pub struct NotificationPreferenceOnEnterDisabledHandler {}

impl NotificationPreferenceOnEnterDisabledHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<NotificationPreferenceTriggerContext, NotificationPreferenceTriggerEvent> for NotificationPreferenceOnEnterDisabledHandler {
    fn events(&self) -> Vec<NotificationPreferenceTriggerEvent> {
        vec![NotificationPreferenceTriggerEvent::OnEnterState("disabled".to_string())]
    }

    async fn handle(&self, ctx: &NotificationPreferenceTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Action executor for NotificationPreference triggers

pub fn notification_preference_trigger_registry() -> NotificationPreferenceTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(NotificationPreferenceAfterCreateHandler1::new()));
        r.register(Arc::new(NotificationPreferenceAfterUpdateHandler2::new()));
        r.register(Arc::new(NotificationPreferenceAfterCreateHandler3::new()));
        r.register(Arc::new(NotificationPreferenceAfterCreateHandler4::new()));
        r.register(Arc::new(NotificationPreferenceOnEnterEnabledHandler::new()));
        r.register(Arc::new(NotificationPreferenceOnEnterDisabledHandler::new()));
    })
}

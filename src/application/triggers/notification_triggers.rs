//! Notification trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::Notification;

pub type NotificationTriggerEvent      = TriggerEvent;
pub type NotificationTriggerContext    = TriggerContext<Notification>;
pub type NotificationTriggerContextMut = TriggerContextMut<Notification>;
pub type NotificationActionExecutor    = ActionExecutor;
pub type NotificationTriggerRegistry   = TriggerRegistry<Notification>;
pub type NotificationTriggerHandlerObj = dyn TriggerHandler<TriggerContext<Notification>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct NotificationAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::NotificationEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<NotificationActionExecutor>>,
}

impl NotificationAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::NotificationEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<NotificationActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<NotificationTriggerContext, NotificationTriggerEvent> for NotificationAfterCreateHandler1 {
    fn events(&self) -> Vec<NotificationTriggerEvent> {
        vec![NotificationTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &NotificationTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Custom action: set_default_priority_based_on_type
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: set_default_priority_based_on_type
        // <<< CUSTOM ACTION END >>>
        // Custom action: queue_for_delivery
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: queue_for_delivery
        // <<< CUSTOM ACTION END >>>
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit notificationcreatedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_created(ctx.entity.clone(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct NotificationAfterCreateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::NotificationEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<NotificationActionExecutor>>,
}

impl NotificationAfterCreateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::NotificationEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<NotificationActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<NotificationTriggerContext, NotificationTriggerEvent> for NotificationAfterCreateHandler2 {
    fn events(&self) -> Vec<NotificationTriggerEvent> {
        vec![NotificationTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &NotificationTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Set sent_at to current timestamp
        // <<< CUSTOM SET: sent_at = now >>>
        // ctx.entity.sent_at = Some(chrono::Utc::now();
        // Note: Add closing parenthesis if using Some()
        // <<< CUSTOM SET: status = 'sent' >>>
        // ctx.entity.status = "sent".to_string(); // or Some(...) if optional
        // Custom action: update_delivery_status
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: update_delivery_status
        // <<< CUSTOM ACTION END >>>
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit notificationsentevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: notificationsentevent
            // <<< CUSTOM EMIT: notificationsentevent >>>
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct NotificationAfterCreateHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::NotificationEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<NotificationActionExecutor>>,
}

impl NotificationAfterCreateHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::NotificationEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<NotificationActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<NotificationTriggerContext, NotificationTriggerEvent> for NotificationAfterCreateHandler3 {
    fn events(&self) -> Vec<NotificationTriggerEvent> {
        vec![NotificationTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &NotificationTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Set read_at to current timestamp
        // <<< CUSTOM SET: read_at = now >>>
        // ctx.entity.read_at = Some(chrono::Utc::now();
        // Note: Add closing parenthesis if using Some()
        // Custom action: update_read_status
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: update_read_status
        // <<< CUSTOM ACTION END >>>
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit notificationreadevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: notificationreadevent
            // <<< CUSTOM EMIT: notificationreadevent >>>
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct NotificationAfterCreateHandler4 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::NotificationEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<NotificationActionExecutor>>,
}

impl NotificationAfterCreateHandler4 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::NotificationEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<NotificationActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<NotificationTriggerContext, NotificationTriggerEvent> for NotificationAfterCreateHandler4 {
    fn events(&self) -> Vec<NotificationTriggerEvent> {
        vec![NotificationTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &NotificationTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Set failed_at to current timestamp
        // <<< CUSTOM SET: failed_at = now >>>
        // ctx.entity.failed_at = Some(chrono::Utc::now();
        // Note: Add closing parenthesis if using Some()
        // <<< CUSTOM SET: status = 'failed' >>>
        // ctx.entity.status = "failed".to_string(); // or Some(...) if optional
        // Custom action: store_failure_details
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: store_failure_details
        // <<< CUSTOM ACTION END >>>
        // Custom action: schedule_retry_if_applicable
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: schedule_retry_if_applicable
        // <<< CUSTOM ACTION END >>>
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit notificationfailedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: notificationfailedevent
            // <<< CUSTOM EMIT: notificationfailedevent >>>
        }
        Ok(())
    }
}

/// BeforeDelete handler
pub struct NotificationBeforeDeleteHandler5 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::NotificationEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<NotificationActionExecutor>>,
}

impl NotificationBeforeDeleteHandler5 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::NotificationEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<NotificationActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<NotificationTriggerContext, NotificationTriggerEvent> for NotificationBeforeDeleteHandler5 {
    fn events(&self) -> Vec<NotificationTriggerEvent> {
        vec![NotificationTriggerEvent::BeforeDelete]
    }

    async fn handle(&self, ctx: &NotificationTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// AfterDelete handler
pub struct NotificationAfterDeleteHandler6 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::NotificationEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<NotificationActionExecutor>>,
}

impl NotificationAfterDeleteHandler6 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::NotificationEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<NotificationActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<NotificationTriggerContext, NotificationTriggerEvent> for NotificationAfterDeleteHandler6 {
    fn events(&self) -> Vec<NotificationTriggerEvent> {
        vec![NotificationTriggerEvent::AfterDelete]
    }

    async fn handle(&self, ctx: &NotificationTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit notificationdeletedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_deleted(ctx.entity.id().to_string(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct NotificationAfterCreateHandler7 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::NotificationEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<NotificationActionExecutor>>,
}

impl NotificationAfterCreateHandler7 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::NotificationEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<NotificationActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<NotificationTriggerContext, NotificationTriggerEvent> for NotificationAfterCreateHandler7 {
    fn events(&self) -> Vec<NotificationTriggerEvent> {
        vec![NotificationTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &NotificationTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Custom action: validate_template_syntax
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: validate_template_syntax
        // <<< CUSTOM ACTION END >>>
        // Custom action: extract_placeholders
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: extract_placeholders
        // <<< CUSTOM ACTION END >>>
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Action executor for Notification triggers

pub fn notification_trigger_registry() -> NotificationTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(NotificationAfterCreateHandler1::new()));
        r.register(Arc::new(NotificationAfterCreateHandler2::new()));
        r.register(Arc::new(NotificationAfterCreateHandler3::new()));
        r.register(Arc::new(NotificationAfterCreateHandler4::new()));
        r.register(Arc::new(NotificationBeforeDeleteHandler5::new()));
        r.register(Arc::new(NotificationAfterDeleteHandler6::new()));
        r.register(Arc::new(NotificationAfterCreateHandler7::new()));
    })
}

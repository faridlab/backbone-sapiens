//! BackupCode trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::BackupCode;

pub type BackupCodeTriggerEvent      = TriggerEvent;
pub type BackupCodeTriggerContext    = TriggerContext<BackupCode>;
pub type BackupCodeTriggerContextMut = TriggerContextMut<BackupCode>;
pub type BackupCodeActionExecutor    = ActionExecutor;
pub type BackupCodeTriggerRegistry   = TriggerRegistry<BackupCode>;
pub type BackupCodeTriggerHandlerObj = dyn TriggerHandler<TriggerContext<BackupCode>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct BackupCodeAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::BackupCodeEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<BackupCodeActionExecutor>>,
}

impl BackupCodeAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::BackupCodeEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<BackupCodeActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<BackupCodeTriggerContext, BackupCodeTriggerEvent> for BackupCodeAfterCreateHandler1 {
    fn events(&self) -> Vec<BackupCodeTriggerEvent> {
        vec![BackupCodeTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &BackupCodeTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Send email notification
        if let Some(executor) = &self.action_executor {
            executor.send_email(ctx, "default").await?;
        }
        // Emit backupcodesgeneratedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: backupcodesgeneratedevent
            // <<< CUSTOM EMIT: backupcodesgeneratedevent >>>
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct BackupCodeAfterCreateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::BackupCodeEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<BackupCodeActionExecutor>>,
}

impl BackupCodeAfterCreateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::BackupCodeEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<BackupCodeActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<BackupCodeTriggerContext, BackupCodeTriggerEvent> for BackupCodeAfterCreateHandler2 {
    fn events(&self) -> Vec<BackupCodeTriggerEvent> {
        vec![BackupCodeTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &BackupCodeTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Set is_used = true
        // <<< CUSTOM SET: is_used = true >>>
        // ctx.entity.is_used = true;
        // Set used_at to current timestamp
        // <<< CUSTOM SET: used_at = now >>>
        // ctx.entity.used_at = Some(chrono::Utc::now();
        // Note: Add closing parenthesis if using Some()
        // <<< CUSTOM SET: used_ip_address = $context.ip_address >>>
        // ctx.entity.used_ip_address = $context.ip_address; // adjust type as needed
        // <<< CUSTOM SET: used_user_agent = $context.user_agent >>>
        // ctx.entity.used_user_agent = $context.user_agent; // adjust type as needed
        // Unknown action type 'increment': usage_count in user where id == user_id
        // Unknown action type 'update': last_mfa_used = now
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit backupcodeusedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: backupcodeusedevent
            // <<< CUSTOM EMIT: backupcodeusedevent >>>
        }
        // Custom action: regenerate_if_needed
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: regenerate_if_needed
        // <<< CUSTOM ACTION END >>>
        Ok(())
    }
}

/// AfterCreate handler
pub struct BackupCodeAfterCreateHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::BackupCodeEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<BackupCodeActionExecutor>>,
}

impl BackupCodeAfterCreateHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::BackupCodeEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<BackupCodeActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<BackupCodeTriggerContext, BackupCodeTriggerEvent> for BackupCodeAfterCreateHandler3 {
    fn events(&self) -> Vec<BackupCodeTriggerEvent> {
        vec![BackupCodeTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &BackupCodeTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Custom action: invalidate_old_codes
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: invalidate_old_codes
        // <<< CUSTOM ACTION END >>>
        // Emit backupcodesregeneratedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: backupcodesregeneratedevent
            // <<< CUSTOM EMIT: backupcodesregeneratedevent >>>
        }
        Ok(())
    }
}

/// BeforeDelete handler
pub struct BackupCodeBeforeDeleteHandler4 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::BackupCodeEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<BackupCodeActionExecutor>>,
}

impl BackupCodeBeforeDeleteHandler4 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::BackupCodeEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<BackupCodeActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<BackupCodeTriggerContext, BackupCodeTriggerEvent> for BackupCodeBeforeDeleteHandler4 {
    fn events(&self) -> Vec<BackupCodeTriggerEvent> {
        vec![BackupCodeTriggerEvent::BeforeDelete]
    }

    async fn handle(&self, ctx: &BackupCodeTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// AfterCreate handler
pub struct BackupCodeAfterCreateHandler5 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::BackupCodeEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<BackupCodeActionExecutor>>,
}

impl BackupCodeAfterCreateHandler5 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::BackupCodeEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<BackupCodeActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<BackupCodeTriggerContext, BackupCodeTriggerEvent> for BackupCodeAfterCreateHandler5 {
    fn events(&self) -> Vec<BackupCodeTriggerEvent> {
        vec![BackupCodeTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &BackupCodeTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit backupcodesdeletedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_deleted(ctx.entity.id().to_string(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// Action executor for BackupCode triggers

pub fn backup_code_trigger_registry() -> BackupCodeTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(BackupCodeAfterCreateHandler1::new()));
        r.register(Arc::new(BackupCodeAfterCreateHandler2::new()));
        r.register(Arc::new(BackupCodeAfterCreateHandler3::new()));
        r.register(Arc::new(BackupCodeBeforeDeleteHandler4::new()));
        r.register(Arc::new(BackupCodeAfterCreateHandler5::new()));
    })
}

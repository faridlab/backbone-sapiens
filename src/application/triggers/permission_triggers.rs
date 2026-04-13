//! Permission trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::Permission;

pub type PermissionTriggerEvent      = TriggerEvent;
pub type PermissionTriggerContext    = TriggerContext<Permission>;
pub type PermissionTriggerContextMut = TriggerContextMut<Permission>;
pub type PermissionActionExecutor    = ActionExecutor;
pub type PermissionTriggerRegistry   = TriggerRegistry<Permission>;
pub type PermissionTriggerHandlerObj = dyn TriggerHandler<TriggerContext<Permission>, TriggerEvent>;


// Lifecycle trigger handlers

/// BeforeCreate handler
pub struct PermissionBeforeCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::PermissionEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<PermissionActionExecutor>>,
}

impl PermissionBeforeCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::PermissionEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<PermissionActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<PermissionTriggerContext, PermissionTriggerEvent> for PermissionBeforeCreateHandler1 {
    fn events(&self) -> Vec<PermissionTriggerEvent> {
        vec![PermissionTriggerEvent::BeforeCreate]
    }

    async fn handle(&self, ctx: &PermissionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // <<< CUSTOM SET: resource = name.split >>>
        // ctx.entity.resource = name.split; // adjust type as needed
        // <<< CUSTOM SET: action = name.split >>>
        // ctx.entity.action = name.split; // adjust type as needed
        Ok(())
    }
}

/// AfterCreate handler
pub struct PermissionAfterCreateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::PermissionEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<PermissionActionExecutor>>,
}

impl PermissionAfterCreateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::PermissionEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<PermissionActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<PermissionTriggerContext, PermissionTriggerEvent> for PermissionAfterCreateHandler2 {
    fn events(&self) -> Vec<PermissionTriggerEvent> {
        vec![PermissionTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &PermissionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit permissioncreatedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_created(ctx.entity.clone(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// AfterUpdate handler
pub struct PermissionAfterUpdateHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::PermissionEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<PermissionActionExecutor>>,
}

impl PermissionAfterUpdateHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::PermissionEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<PermissionActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<PermissionTriggerContext, PermissionTriggerEvent> for PermissionAfterUpdateHandler3 {
    fn events(&self) -> Vec<PermissionTriggerEvent> {
        vec![PermissionTriggerEvent::AfterUpdate]
    }

    async fn handle(&self, ctx: &PermissionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// AfterDelete handler
pub struct PermissionAfterDeleteHandler4 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::PermissionEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<PermissionActionExecutor>>,
}

impl PermissionAfterDeleteHandler4 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::PermissionEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<PermissionActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<PermissionTriggerContext, PermissionTriggerEvent> for PermissionAfterDeleteHandler4 {
    fn events(&self) -> Vec<PermissionTriggerEvent> {
        vec![PermissionTriggerEvent::AfterDelete]
    }

    async fn handle(&self, ctx: &PermissionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit permissiondeletedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_deleted(ctx.entity.id().to_string(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// Action executor for Permission triggers

pub fn permission_trigger_registry() -> PermissionTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(PermissionBeforeCreateHandler1::new()));
        r.register(Arc::new(PermissionAfterCreateHandler2::new()));
        r.register(Arc::new(PermissionAfterUpdateHandler3::new()));
        r.register(Arc::new(PermissionAfterDeleteHandler4::new()));
    })
}

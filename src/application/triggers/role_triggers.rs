//! Role trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::Role;

pub type RoleTriggerEvent      = TriggerEvent;
pub type RoleTriggerContext    = TriggerContext<Role>;
pub type RoleTriggerContextMut = TriggerContextMut<Role>;
pub type RoleActionExecutor    = ActionExecutor;
pub type RoleTriggerRegistry   = TriggerRegistry<Role>;
pub type RoleTriggerHandlerObj = dyn TriggerHandler<TriggerContext<Role>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct RoleAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::RoleEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<RoleActionExecutor>>,
}

impl RoleAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::RoleEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<RoleActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<RoleTriggerContext, RoleTriggerEvent> for RoleAfterCreateHandler1 {
    fn events(&self) -> Vec<RoleTriggerEvent> {
        vec![RoleTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &RoleTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit rolecreatedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_created(ctx.entity.clone(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// AfterUpdate handler
pub struct RoleAfterUpdateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::RoleEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<RoleActionExecutor>>,
}

impl RoleAfterUpdateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::RoleEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<RoleActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<RoleTriggerContext, RoleTriggerEvent> for RoleAfterUpdateHandler2 {
    fn events(&self) -> Vec<RoleTriggerEvent> {
        vec![RoleTriggerEvent::AfterUpdate]
    }

    async fn handle(&self, ctx: &RoleTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit roleupdatedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_updated(ctx.entity.clone(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct RoleAfterCreateHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::RoleEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<RoleActionExecutor>>,
}

impl RoleAfterCreateHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::RoleEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<RoleActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<RoleTriggerContext, RoleTriggerEvent> for RoleAfterCreateHandler3 {
    fn events(&self) -> Vec<RoleTriggerEvent> {
        vec![RoleTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &RoleTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'trigger': unset_other_default_roles
        Ok(())
    }
}

/// BeforeDelete handler
pub struct RoleBeforeDeleteHandler4 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::RoleEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<RoleActionExecutor>>,
}

impl RoleBeforeDeleteHandler4 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::RoleEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<RoleActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<RoleTriggerContext, RoleTriggerEvent> for RoleBeforeDeleteHandler4 {
    fn events(&self) -> Vec<RoleTriggerEvent> {
        vec![RoleTriggerEvent::BeforeDelete]
    }

    async fn handle(&self, ctx: &RoleTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// AfterDelete handler
pub struct RoleAfterDeleteHandler5 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::RoleEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<RoleActionExecutor>>,
}

impl RoleAfterDeleteHandler5 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::RoleEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<RoleActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<RoleTriggerContext, RoleTriggerEvent> for RoleAfterDeleteHandler5 {
    fn events(&self) -> Vec<RoleTriggerEvent> {
        vec![RoleTriggerEvent::AfterDelete]
    }

    async fn handle(&self, ctx: &RoleTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit roledeletedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_deleted(ctx.entity.id().to_string(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// Action executor for Role triggers

pub fn role_trigger_registry() -> RoleTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(RoleAfterCreateHandler1::new()));
        r.register(Arc::new(RoleAfterUpdateHandler2::new()));
        r.register(Arc::new(RoleAfterCreateHandler3::new()));
        r.register(Arc::new(RoleBeforeDeleteHandler4::new()));
        r.register(Arc::new(RoleAfterDeleteHandler5::new()));
    })
}

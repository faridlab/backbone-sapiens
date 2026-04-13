//! Profile trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::Profile;

pub type ProfileTriggerEvent      = TriggerEvent;
pub type ProfileTriggerContext    = TriggerContext<Profile>;
pub type ProfileTriggerContextMut = TriggerContextMut<Profile>;
pub type ProfileActionExecutor    = ActionExecutor;
pub type ProfileTriggerRegistry   = TriggerRegistry<Profile>;
pub type ProfileTriggerHandlerObj = dyn TriggerHandler<TriggerContext<Profile>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct ProfileAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::ProfileEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<ProfileActionExecutor>>,
}

impl ProfileAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::ProfileEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<ProfileActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<ProfileTriggerContext, ProfileTriggerEvent> for ProfileAfterCreateHandler1 {
    fn events(&self) -> Vec<ProfileTriggerEvent> {
        vec![ProfileTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &ProfileTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.user_id);
        // Emit profilecreatedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_created(ctx.entity.clone(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// AfterUpdate handler
pub struct ProfileAfterUpdateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::ProfileEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<ProfileActionExecutor>>,
}

impl ProfileAfterUpdateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::ProfileEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<ProfileActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<ProfileTriggerContext, ProfileTriggerEvent> for ProfileAfterUpdateHandler2 {
    fn events(&self) -> Vec<ProfileTriggerEvent> {
        vec![ProfileTriggerEvent::AfterUpdate]
    }

    async fn handle(&self, ctx: &ProfileTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.user_id);
        // Emit profileupdatedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_updated(ctx.entity.clone(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct ProfileAfterCreateHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::ProfileEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<ProfileActionExecutor>>,
}

impl ProfileAfterCreateHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::ProfileEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<ProfileActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<ProfileTriggerContext, ProfileTriggerEvent> for ProfileAfterCreateHandler3 {
    fn events(&self) -> Vec<ProfileTriggerEvent> {
        vec![ProfileTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &ProfileTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'trigger': process_profile_picture
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.user_id);
        Ok(())
    }
}

/// AfterDelete handler
pub struct ProfileAfterDeleteHandler4 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::ProfileEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<ProfileActionExecutor>>,
}

impl ProfileAfterDeleteHandler4 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::ProfileEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<ProfileActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<ProfileTriggerContext, ProfileTriggerEvent> for ProfileAfterDeleteHandler4 {
    fn events(&self) -> Vec<ProfileTriggerEvent> {
        vec![ProfileTriggerEvent::AfterDelete]
    }

    async fn handle(&self, ctx: &ProfileTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.user_id);
        // Emit profiledeletedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_deleted(ctx.entity.id().to_string(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// Action executor for Profile triggers

pub fn profile_trigger_registry() -> ProfileTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(ProfileAfterCreateHandler1::new()));
        r.register(Arc::new(ProfileAfterUpdateHandler2::new()));
        r.register(Arc::new(ProfileAfterCreateHandler3::new()));
        r.register(Arc::new(ProfileAfterDeleteHandler4::new()));
    })
}

//! UserSAMLLink trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::UserSAMLLink;

pub type UserSAMLLinkTriggerEvent      = TriggerEvent;
pub type UserSAMLLinkTriggerContext    = TriggerContext<UserSAMLLink>;
pub type UserSAMLLinkTriggerContextMut = TriggerContextMut<UserSAMLLink>;
pub type UserSAMLLinkActionExecutor    = ActionExecutor;
pub type UserSAMLLinkTriggerRegistry   = TriggerRegistry<UserSAMLLink>;
pub type UserSAMLLinkTriggerHandlerObj = dyn TriggerHandler<TriggerContext<UserSAMLLink>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct UserSAMLLinkAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::UserSAMLLinkEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<UserSAMLLinkActionExecutor>>,
}

impl UserSAMLLinkAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::UserSAMLLinkEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<UserSAMLLinkActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<UserSAMLLinkTriggerContext, UserSAMLLinkTriggerEvent> for UserSAMLLinkAfterCreateHandler1 {
    fn events(&self) -> Vec<UserSAMLLinkTriggerEvent> {
        vec![UserSAMLLinkTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &UserSAMLLinkTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit samllinkcreatedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_created(ctx.entity.clone(), ctx.user_id.clone()).await?;
        }
        // Custom action: invalidate_user_auth_cache
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: invalidate_user_auth_cache
        // <<< CUSTOM ACTION END >>>
        Ok(())
    }
}

/// AfterUpdate handler
pub struct UserSAMLLinkAfterUpdateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::UserSAMLLinkEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<UserSAMLLinkActionExecutor>>,
}

impl UserSAMLLinkAfterUpdateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::UserSAMLLinkEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<UserSAMLLinkActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<UserSAMLLinkTriggerContext, UserSAMLLinkTriggerEvent> for UserSAMLLinkAfterUpdateHandler2 {
    fn events(&self) -> Vec<UserSAMLLinkTriggerEvent> {
        vec![UserSAMLLinkTriggerEvent::AfterUpdate]
    }

    async fn handle(&self, ctx: &UserSAMLLinkTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Custom action: invalidate_user_auth_cache
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: invalidate_user_auth_cache
        // <<< CUSTOM ACTION END >>>
        Ok(())
    }
}

/// AfterCreate handler
pub struct UserSAMLLinkAfterCreateHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::UserSAMLLinkEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<UserSAMLLinkActionExecutor>>,
}

impl UserSAMLLinkAfterCreateHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::UserSAMLLinkEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<UserSAMLLinkActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<UserSAMLLinkTriggerContext, UserSAMLLinkTriggerEvent> for UserSAMLLinkAfterCreateHandler3 {
    fn events(&self) -> Vec<UserSAMLLinkTriggerEvent> {
        vec![UserSAMLLinkTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &UserSAMLLinkTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit samllinkdeactivatedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: samllinkdeactivatedevent
            // <<< CUSTOM EMIT: samllinkdeactivatedevent >>>
        }
        // Custom action: invalidate_user_auth_cache
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: invalidate_user_auth_cache
        // <<< CUSTOM ACTION END >>>
        // Send notification
        if let Some(executor) = &self.action_executor {
            executor.notify(ctx, "default").await?;
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct UserSAMLLinkAfterCreateHandler4 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::UserSAMLLinkEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<UserSAMLLinkActionExecutor>>,
}

impl UserSAMLLinkAfterCreateHandler4 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::UserSAMLLinkEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<UserSAMLLinkActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<UserSAMLLinkTriggerContext, UserSAMLLinkTriggerEvent> for UserSAMLLinkAfterCreateHandler4 {
    fn events(&self) -> Vec<UserSAMLLinkTriggerEvent> {
        vec![UserSAMLLinkTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &UserSAMLLinkTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'trigger': sync_saml_user_attributes
        Ok(())
    }
}

// State machine trigger handlers

/// Handler for entering active state
pub struct UserSAMLLinkOnEnterActiveHandler {
    pub event_publisher: Option<Arc<crate::domain::event::UserSAMLLinkEventPublisher>>,
}

impl UserSAMLLinkOnEnterActiveHandler {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<UserSAMLLinkTriggerContext, UserSAMLLinkTriggerEvent> for UserSAMLLinkOnEnterActiveHandler {
    fn events(&self) -> Vec<UserSAMLLinkTriggerEvent> {
        vec![UserSAMLLinkTriggerEvent::OnEnterState("active".to_string())]
    }

    async fn handle(&self, ctx: &UserSAMLLinkTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit samllinkcreatedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_created(ctx.entity.clone(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// Handler for entering inactive state
pub struct UserSAMLLinkOnEnterInactiveHandler {
    pub event_publisher: Option<Arc<crate::domain::event::UserSAMLLinkEventPublisher>>,
}

impl UserSAMLLinkOnEnterInactiveHandler {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<UserSAMLLinkTriggerContext, UserSAMLLinkTriggerEvent> for UserSAMLLinkOnEnterInactiveHandler {
    fn events(&self) -> Vec<UserSAMLLinkTriggerEvent> {
        vec![UserSAMLLinkTriggerEvent::OnEnterState("inactive".to_string())]
    }

    async fn handle(&self, ctx: &UserSAMLLinkTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit samllinkdeactivatedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: samllinkdeactivatedevent
            // <<< CUSTOM EMIT: samllinkdeactivatedevent >>>
        }
        Ok(())
    }
}

/// Action executor for UserSAMLLink triggers

pub fn user_s_a_m_l_link_trigger_registry() -> UserSAMLLinkTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(UserSAMLLinkAfterCreateHandler1::new()));
        r.register(Arc::new(UserSAMLLinkAfterUpdateHandler2::new()));
        r.register(Arc::new(UserSAMLLinkAfterCreateHandler3::new()));
        r.register(Arc::new(UserSAMLLinkAfterCreateHandler4::new()));
        r.register(Arc::new(UserSAMLLinkOnEnterActiveHandler::new()));
        r.register(Arc::new(UserSAMLLinkOnEnterInactiveHandler::new()));
    })
}

//! UserOAuthLink trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::UserOAuthLink;

pub type UserOAuthLinkTriggerEvent      = TriggerEvent;
pub type UserOAuthLinkTriggerContext    = TriggerContext<UserOAuthLink>;
pub type UserOAuthLinkTriggerContextMut = TriggerContextMut<UserOAuthLink>;
pub type UserOAuthLinkActionExecutor    = ActionExecutor;
pub type UserOAuthLinkTriggerRegistry   = TriggerRegistry<UserOAuthLink>;
pub type UserOAuthLinkTriggerHandlerObj = dyn TriggerHandler<TriggerContext<UserOAuthLink>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct UserOAuthLinkAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::UserOAuthLinkEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<UserOAuthLinkActionExecutor>>,
}

impl UserOAuthLinkAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::UserOAuthLinkEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<UserOAuthLinkActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<UserOAuthLinkTriggerContext, UserOAuthLinkTriggerEvent> for UserOAuthLinkAfterCreateHandler1 {
    fn events(&self) -> Vec<UserOAuthLinkTriggerEvent> {
        vec![UserOAuthLinkTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &UserOAuthLinkTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Custom action: sync_oauth_profile
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: sync_oauth_profile
        // <<< CUSTOM ACTION END >>>
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit oauthlinkcreatedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_created(ctx.entity.clone(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct UserOAuthLinkAfterCreateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::UserOAuthLinkEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<UserOAuthLinkActionExecutor>>,
}

impl UserOAuthLinkAfterCreateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::UserOAuthLinkEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<UserOAuthLinkActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<UserOAuthLinkTriggerContext, UserOAuthLinkTriggerEvent> for UserOAuthLinkAfterCreateHandler2 {
    fn events(&self) -> Vec<UserOAuthLinkTriggerEvent> {
        vec![UserOAuthLinkTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &UserOAuthLinkTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Custom action: schedule_profile_sync
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: schedule_profile_sync
        // <<< CUSTOM ACTION END >>>
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// AfterCreate handler
pub struct UserOAuthLinkAfterCreateHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::UserOAuthLinkEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<UserOAuthLinkActionExecutor>>,
}

impl UserOAuthLinkAfterCreateHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::UserOAuthLinkEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<UserOAuthLinkActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<UserOAuthLinkTriggerContext, UserOAuthLinkTriggerEvent> for UserOAuthLinkAfterCreateHandler3 {
    fn events(&self) -> Vec<UserOAuthLinkTriggerEvent> {
        vec![UserOAuthLinkTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &UserOAuthLinkTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Custom action: set_other_links_as_non_primary
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: set_other_links_as_non_primary
        // <<< CUSTOM ACTION END >>>
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit oauthlinkmadeprimaryevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: oauthlinkmadeprimaryevent
            // <<< CUSTOM EMIT: oauthlinkmadeprimaryevent >>>
        }
        Ok(())
    }
}

/// BeforeDelete handler
pub struct UserOAuthLinkBeforeDeleteHandler4 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::UserOAuthLinkEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<UserOAuthLinkActionExecutor>>,
}

impl UserOAuthLinkBeforeDeleteHandler4 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::UserOAuthLinkEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<UserOAuthLinkActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<UserOAuthLinkTriggerContext, UserOAuthLinkTriggerEvent> for UserOAuthLinkBeforeDeleteHandler4 {
    fn events(&self) -> Vec<UserOAuthLinkTriggerEvent> {
        vec![UserOAuthLinkTriggerEvent::BeforeDelete]
    }

    async fn handle(&self, ctx: &UserOAuthLinkTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Custom action: validate_not_last_auth_method
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: validate_not_last_auth_method
        // <<< CUSTOM ACTION END >>>
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// AfterDelete handler
pub struct UserOAuthLinkAfterDeleteHandler5 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::UserOAuthLinkEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<UserOAuthLinkActionExecutor>>,
}

impl UserOAuthLinkAfterDeleteHandler5 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::UserOAuthLinkEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<UserOAuthLinkActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<UserOAuthLinkTriggerContext, UserOAuthLinkTriggerEvent> for UserOAuthLinkAfterDeleteHandler5 {
    fn events(&self) -> Vec<UserOAuthLinkTriggerEvent> {
        vec![UserOAuthLinkTriggerEvent::AfterDelete]
    }

    async fn handle(&self, ctx: &UserOAuthLinkTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Custom action: revoke_oauth_tokens
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: revoke_oauth_tokens
        // <<< CUSTOM ACTION END >>>
        // Custom action: notify_oauth_link_unlinked
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: notify_oauth_link_unlinked
        // <<< CUSTOM ACTION END >>>
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit oauthlinkdeletedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_deleted(ctx.entity.id().to_string(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct UserOAuthLinkAfterCreateHandler6 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::UserOAuthLinkEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<UserOAuthLinkActionExecutor>>,
}

impl UserOAuthLinkAfterCreateHandler6 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::UserOAuthLinkEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<UserOAuthLinkActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<UserOAuthLinkTriggerContext, UserOAuthLinkTriggerEvent> for UserOAuthLinkAfterCreateHandler6 {
    fn events(&self) -> Vec<UserOAuthLinkTriggerEvent> {
        vec![UserOAuthLinkTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &UserOAuthLinkTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'update': last_synced = now
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit oauthprofilesyncedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: oauthprofilesyncedevent
            // <<< CUSTOM EMIT: oauthprofilesyncedevent >>>
        }
        Ok(())
    }
}

// State machine trigger handlers

/// Handler for entering linked state
pub struct UserOAuthLinkOnEnterLinkedHandler {
    pub event_publisher: Option<Arc<crate::domain::event::UserOAuthLinkEventPublisher>>,
}

impl UserOAuthLinkOnEnterLinkedHandler {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<UserOAuthLinkTriggerContext, UserOAuthLinkTriggerEvent> for UserOAuthLinkOnEnterLinkedHandler {
    fn events(&self) -> Vec<UserOAuthLinkTriggerEvent> {
        vec![UserOAuthLinkTriggerEvent::OnEnterState("linked".to_string())]
    }

    async fn handle(&self, ctx: &UserOAuthLinkTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Custom action: sync_oauth_profile
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: sync_oauth_profile
        // <<< CUSTOM ACTION END >>>
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit oauthlinkcreatedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_created(ctx.entity.clone(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// Handler for entering unlinked state
pub struct UserOAuthLinkOnEnterUnlinkedHandler {
    pub event_publisher: Option<Arc<crate::domain::event::UserOAuthLinkEventPublisher>>,
}

impl UserOAuthLinkOnEnterUnlinkedHandler {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<UserOAuthLinkTriggerContext, UserOAuthLinkTriggerEvent> for UserOAuthLinkOnEnterUnlinkedHandler {
    fn events(&self) -> Vec<UserOAuthLinkTriggerEvent> {
        vec![UserOAuthLinkTriggerEvent::OnEnterState("unlinked".to_string())]
    }

    async fn handle(&self, ctx: &UserOAuthLinkTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Custom action: revoke_oauth_tokens
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: revoke_oauth_tokens
        // <<< CUSTOM ACTION END >>>
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit oauthlinkdeletedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_deleted(ctx.entity.id().to_string(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// Handler for entering requires_verification state
pub struct UserOAuthLinkOnEnterRequiresVerificationHandler {
    pub action_executor: Option<Arc<UserOAuthLinkActionExecutor>>,
}

impl UserOAuthLinkOnEnterRequiresVerificationHandler {
    pub fn new() -> Self {
        Self {
            action_executor: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<UserOAuthLinkTriggerContext, UserOAuthLinkTriggerEvent> for UserOAuthLinkOnEnterRequiresVerificationHandler {
    fn events(&self) -> Vec<UserOAuthLinkTriggerEvent> {
        vec![UserOAuthLinkTriggerEvent::OnEnterState("requires_verification".to_string())]
    }

    async fn handle(&self, ctx: &UserOAuthLinkTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Send email notification
        if let Some(executor) = &self.action_executor {
            executor.send_email(ctx, "default").await?;
        }
        // Custom action: schedule_job
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: schedule_job
        // <<< CUSTOM ACTION END >>>
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Action executor for UserOAuthLink triggers

pub fn user_o_auth_link_trigger_registry() -> UserOAuthLinkTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(UserOAuthLinkAfterCreateHandler1::new()));
        r.register(Arc::new(UserOAuthLinkAfterCreateHandler2::new()));
        r.register(Arc::new(UserOAuthLinkAfterCreateHandler3::new()));
        r.register(Arc::new(UserOAuthLinkBeforeDeleteHandler4::new()));
        r.register(Arc::new(UserOAuthLinkAfterDeleteHandler5::new()));
        r.register(Arc::new(UserOAuthLinkAfterCreateHandler6::new()));
        r.register(Arc::new(UserOAuthLinkOnEnterLinkedHandler::new()));
        r.register(Arc::new(UserOAuthLinkOnEnterUnlinkedHandler::new()));
        r.register(Arc::new(UserOAuthLinkOnEnterRequiresVerificationHandler::new()));
    })
}

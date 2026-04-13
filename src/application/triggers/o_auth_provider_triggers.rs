//! OAuthProvider trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::OAuthProvider;

pub type OAuthProviderTriggerEvent      = TriggerEvent;
pub type OAuthProviderTriggerContext    = TriggerContext<OAuthProvider>;
pub type OAuthProviderTriggerContextMut = TriggerContextMut<OAuthProvider>;
pub type OAuthProviderActionExecutor    = ActionExecutor;
pub type OAuthProviderTriggerRegistry   = TriggerRegistry<OAuthProvider>;
pub type OAuthProviderTriggerHandlerObj = dyn TriggerHandler<TriggerContext<OAuthProvider>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct OAuthProviderAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::OAuthProviderEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<OAuthProviderActionExecutor>>,
}

impl OAuthProviderAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::OAuthProviderEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<OAuthProviderActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<OAuthProviderTriggerContext, OAuthProviderTriggerEvent> for OAuthProviderAfterCreateHandler1 {
    fn events(&self) -> Vec<OAuthProviderTriggerEvent> {
        vec![OAuthProviderTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &OAuthProviderTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit oauthprovidercreatedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_created(ctx.entity.clone(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct OAuthProviderAfterCreateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::OAuthProviderEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<OAuthProviderActionExecutor>>,
}

impl OAuthProviderAfterCreateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::OAuthProviderEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<OAuthProviderActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<OAuthProviderTriggerContext, OAuthProviderTriggerEvent> for OAuthProviderAfterCreateHandler2 {
    fn events(&self) -> Vec<OAuthProviderTriggerEvent> {
        vec![OAuthProviderTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &OAuthProviderTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit oauthproviderstatuschangedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: oauthproviderstatuschangedevent
            // <<< CUSTOM EMIT: oauthproviderstatuschangedevent >>>
        }
        Ok(())
    }
}

/// BeforeDelete handler
pub struct OAuthProviderBeforeDeleteHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::OAuthProviderEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<OAuthProviderActionExecutor>>,
}

impl OAuthProviderBeforeDeleteHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::OAuthProviderEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<OAuthProviderActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<OAuthProviderTriggerContext, OAuthProviderTriggerEvent> for OAuthProviderBeforeDeleteHandler3 {
    fn events(&self) -> Vec<OAuthProviderTriggerEvent> {
        vec![OAuthProviderTriggerEvent::BeforeDelete]
    }

    async fn handle(&self, ctx: &OAuthProviderTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Custom action: validate_no_active_links
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: validate_no_active_links
        // <<< CUSTOM ACTION END >>>
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// AfterDelete handler
pub struct OAuthProviderAfterDeleteHandler4 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::OAuthProviderEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<OAuthProviderActionExecutor>>,
}

impl OAuthProviderAfterDeleteHandler4 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::OAuthProviderEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<OAuthProviderActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<OAuthProviderTriggerContext, OAuthProviderTriggerEvent> for OAuthProviderAfterDeleteHandler4 {
    fn events(&self) -> Vec<OAuthProviderTriggerEvent> {
        vec![OAuthProviderTriggerEvent::AfterDelete]
    }

    async fn handle(&self, ctx: &OAuthProviderTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Custom action: delete_all_user_links
        // <<< CUSTOM ACTION START >>>
        // Implement custom action: delete_all_user_links
        // <<< CUSTOM ACTION END >>>
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        // Emit oauthproviderdeletedevent event
        if let Some(publisher) = &self.event_publisher {
            publisher.publish_deleted(ctx.entity.id().to_string(), ctx.user_id.clone()).await?;
        }
        Ok(())
    }
}

// State machine trigger handlers

/// Handler for entering active state
pub struct OAuthProviderOnEnterActiveHandler {}

impl OAuthProviderOnEnterActiveHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<OAuthProviderTriggerContext, OAuthProviderTriggerEvent> for OAuthProviderOnEnterActiveHandler {
    fn events(&self) -> Vec<OAuthProviderTriggerEvent> {
        vec![OAuthProviderTriggerEvent::OnEnterState("active".to_string())]
    }

    async fn handle(&self, ctx: &OAuthProviderTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Handler for entering inactive state
pub struct OAuthProviderOnEnterInactiveHandler {}

impl OAuthProviderOnEnterInactiveHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<OAuthProviderTriggerContext, OAuthProviderTriggerEvent> for OAuthProviderOnEnterInactiveHandler {
    fn events(&self) -> Vec<OAuthProviderTriggerEvent> {
        vec![OAuthProviderTriggerEvent::OnEnterState("inactive".to_string())]
    }

    async fn handle(&self, ctx: &OAuthProviderTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Handler for entering maintenance state
pub struct OAuthProviderOnEnterMaintenanceHandler {}

impl OAuthProviderOnEnterMaintenanceHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<OAuthProviderTriggerContext, OAuthProviderTriggerEvent> for OAuthProviderOnEnterMaintenanceHandler {
    fn events(&self) -> Vec<OAuthProviderTriggerEvent> {
        vec![OAuthProviderTriggerEvent::OnEnterState("maintenance".to_string())]
    }

    async fn handle(&self, ctx: &OAuthProviderTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Action executor for OAuthProvider triggers

pub fn o_auth_provider_trigger_registry() -> OAuthProviderTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(OAuthProviderAfterCreateHandler1::new()));
        r.register(Arc::new(OAuthProviderAfterCreateHandler2::new()));
        r.register(Arc::new(OAuthProviderBeforeDeleteHandler3::new()));
        r.register(Arc::new(OAuthProviderAfterDeleteHandler4::new()));
        r.register(Arc::new(OAuthProviderOnEnterActiveHandler::new()));
        r.register(Arc::new(OAuthProviderOnEnterInactiveHandler::new()));
        r.register(Arc::new(OAuthProviderOnEnterMaintenanceHandler::new()));
    })
}

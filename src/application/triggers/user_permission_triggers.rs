//! UserPermission trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::UserPermission;

pub type UserPermissionTriggerEvent      = TriggerEvent;
pub type UserPermissionTriggerContext    = TriggerContext<UserPermission>;
pub type UserPermissionTriggerContextMut = TriggerContextMut<UserPermission>;
pub type UserPermissionActionExecutor    = ActionExecutor;
pub type UserPermissionTriggerRegistry   = TriggerRegistry<UserPermission>;
pub type UserPermissionTriggerHandlerObj = dyn TriggerHandler<TriggerContext<UserPermission>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct UserPermissionAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::UserPermissionEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<UserPermissionActionExecutor>>,
}

impl UserPermissionAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::UserPermissionEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<UserPermissionActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<UserPermissionTriggerContext, UserPermissionTriggerEvent> for UserPermissionAfterCreateHandler1 {
    fn events(&self) -> Vec<UserPermissionTriggerEvent> {
        vec![UserPermissionTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &UserPermissionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Send notification
        if let Some(executor) = &self.action_executor {
            executor.notify(ctx, "default").await?;
        }
        // Emit userpermissiongrantedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: userpermissiongrantedevent
            // <<< CUSTOM EMIT: userpermissiongrantedevent >>>
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct UserPermissionAfterCreateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::UserPermissionEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<UserPermissionActionExecutor>>,
}

impl UserPermissionAfterCreateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::UserPermissionEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<UserPermissionActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<UserPermissionTriggerContext, UserPermissionTriggerEvent> for UserPermissionAfterCreateHandler2 {
    fn events(&self) -> Vec<UserPermissionTriggerEvent> {
        vec![UserPermissionTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &UserPermissionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Emit userpermissionrevokedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: userpermissionrevokedevent
            // <<< CUSTOM EMIT: userpermissionrevokedevent >>>
        }
        Ok(())
    }
}

// State machine trigger handlers

/// Handler for entering active state
pub struct UserPermissionOnEnterActiveHandler {}

impl UserPermissionOnEnterActiveHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<UserPermissionTriggerContext, UserPermissionTriggerEvent> for UserPermissionOnEnterActiveHandler {
    fn events(&self) -> Vec<UserPermissionTriggerEvent> {
        vec![UserPermissionTriggerEvent::OnEnterState("active".to_string())]
    }

    async fn handle(&self, ctx: &UserPermissionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Handler for entering revoked state
pub struct UserPermissionOnEnterRevokedHandler {}

impl UserPermissionOnEnterRevokedHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<UserPermissionTriggerContext, UserPermissionTriggerEvent> for UserPermissionOnEnterRevokedHandler {
    fn events(&self) -> Vec<UserPermissionTriggerEvent> {
        vec![UserPermissionTriggerEvent::OnEnterState("revoked".to_string())]
    }

    async fn handle(&self, ctx: &UserPermissionTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Set revoked_at to current timestamp
        // <<< CUSTOM SET: revoked_at = now >>>
        // ctx.entity.revoked_at = Some(chrono::Utc::now();
        // Note: Add closing parenthesis if using Some()
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Action executor for UserPermission triggers

pub fn user_permission_trigger_registry() -> UserPermissionTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(UserPermissionAfterCreateHandler1::new()));
        r.register(Arc::new(UserPermissionAfterCreateHandler2::new()));
        r.register(Arc::new(UserPermissionOnEnterActiveHandler::new()));
        r.register(Arc::new(UserPermissionOnEnterRevokedHandler::new()));
    })
}

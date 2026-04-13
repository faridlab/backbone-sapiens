//! MFADevice trigger handlers

use std::sync::Arc;
use async_trait::async_trait;
use backbone_core::trigger::{
    ActionExecutor, TriggerContext, TriggerContextMut, TriggerEvent, TriggerHandler,
    TriggerRegistry,
};

use crate::domain::entity::MFADevice;

pub type MFADeviceTriggerEvent      = TriggerEvent;
pub type MFADeviceTriggerContext    = TriggerContext<MFADevice>;
pub type MFADeviceTriggerContextMut = TriggerContextMut<MFADevice>;
pub type MFADeviceActionExecutor    = ActionExecutor;
pub type MFADeviceTriggerRegistry   = TriggerRegistry<MFADevice>;
pub type MFADeviceTriggerHandlerObj = dyn TriggerHandler<TriggerContext<MFADevice>, TriggerEvent>;


// Lifecycle trigger handlers

/// AfterCreate handler
pub struct MFADeviceAfterCreateHandler1 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::MFADeviceEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<MFADeviceActionExecutor>>,
}

impl MFADeviceAfterCreateHandler1 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::MFADeviceEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<MFADeviceActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<MFADeviceTriggerContext, MFADeviceTriggerEvent> for MFADeviceAfterCreateHandler1 {
    fn events(&self) -> Vec<MFADeviceTriggerEvent> {
        vec![MFADeviceTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &MFADeviceTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'trigger': update_user_mfa_status
        // Emit mfadeviceenrolledevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: mfadeviceenrolledevent
            // <<< CUSTOM EMIT: mfadeviceenrolledevent >>>
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct MFADeviceAfterCreateHandler2 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::MFADeviceEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<MFADeviceActionExecutor>>,
}

impl MFADeviceAfterCreateHandler2 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::MFADeviceEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<MFADeviceActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<MFADeviceTriggerContext, MFADeviceTriggerEvent> for MFADeviceAfterCreateHandler2 {
    fn events(&self) -> Vec<MFADeviceTriggerEvent> {
        vec![MFADeviceTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &MFADeviceTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Unknown action type 'trigger': update_user_mfa_status
        // Emit mfadevicestatuschangedevent event
        if let Some(_publisher) = &self.event_publisher {
            // Custom event: mfadevicestatuschangedevent
            // <<< CUSTOM EMIT: mfadevicestatuschangedevent >>>
        }
        Ok(())
    }
}

/// AfterCreate handler
pub struct MFADeviceAfterCreateHandler3 {
    /// Event publisher for emitting domain events
    pub event_publisher: Option<Arc<crate::domain::event::MFADeviceEventPublisher>>,
    /// Action executor for side effects
    pub action_executor: Option<Arc<MFADeviceActionExecutor>>,
}

impl MFADeviceAfterCreateHandler3 {
    pub fn new() -> Self {
        Self {
            event_publisher: None,
            action_executor: None,
        }
    }

    pub fn with_event_publisher(mut self, publisher: Arc<crate::domain::event::MFADeviceEventPublisher>) -> Self {
        self.event_publisher = Some(publisher);
        self
    }

    pub fn with_action_executor(mut self, executor: Arc<MFADeviceActionExecutor>) -> Self {
        self.action_executor = Some(executor);
        self
    }
}

#[async_trait]
impl TriggerHandler<MFADeviceTriggerContext, MFADeviceTriggerEvent> for MFADeviceAfterCreateHandler3 {
    fn events(&self) -> Vec<MFADeviceTriggerEvent> {
        vec![MFADeviceTriggerEvent::AfterCreate]
    }

    async fn handle(&self, ctx: &MFADeviceTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

// State machine trigger handlers

/// Handler for entering pending state
pub struct MFADeviceOnEnterPendingHandler {}

impl MFADeviceOnEnterPendingHandler {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl TriggerHandler<MFADeviceTriggerContext, MFADeviceTriggerEvent> for MFADeviceOnEnterPendingHandler {
    fn events(&self) -> Vec<MFADeviceTriggerEvent> {
        vec![MFADeviceTriggerEvent::OnEnterState("pending".to_string())]
    }

    async fn handle(&self, ctx: &MFADeviceTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Handler for entering active state
pub struct MFADeviceOnEnterActiveHandler {
    pub action_executor: Option<Arc<MFADeviceActionExecutor>>,
}

impl MFADeviceOnEnterActiveHandler {
    pub fn new() -> Self {
        Self {
            action_executor: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<MFADeviceTriggerContext, MFADeviceTriggerEvent> for MFADeviceOnEnterActiveHandler {
    fn events(&self) -> Vec<MFADeviceTriggerEvent> {
        vec![MFADeviceTriggerEvent::OnEnterState("active".to_string())]
    }

    async fn handle(&self, ctx: &MFADeviceTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Set verified_at to current timestamp
        // <<< CUSTOM SET: verified_at = now >>>
        // ctx.entity.verified_at = Some(chrono::Utc::now();
        // Note: Add closing parenthesis if using Some()
        // Send email notification
        if let Some(executor) = &self.action_executor {
            executor.send_email(ctx, "default").await?;
        }
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Handler for entering disabled state
pub struct MFADeviceOnEnterDisabledHandler {
    pub action_executor: Option<Arc<MFADeviceActionExecutor>>,
}

impl MFADeviceOnEnterDisabledHandler {
    pub fn new() -> Self {
        Self {
            action_executor: None,
        }
    }
}

#[async_trait]
impl TriggerHandler<MFADeviceTriggerContext, MFADeviceTriggerEvent> for MFADeviceOnEnterDisabledHandler {
    fn events(&self) -> Vec<MFADeviceTriggerEvent> {
        vec![MFADeviceTriggerEvent::OnEnterState("disabled".to_string())]
    }

    async fn handle(&self, ctx: &MFADeviceTriggerContext) -> anyhow::Result<()> {
        let _ = &ctx; // Mark as used to avoid unused warning
        // Send email notification
        if let Some(executor) = &self.action_executor {
            executor.send_email(ctx, "default").await?;
        }
        tracing::info!("Trigger executed for entity: {:?}", ctx.entity.id);
        Ok(())
    }
}

/// Action executor for MFADevice triggers

pub fn m_f_a_device_trigger_registry() -> MFADeviceTriggerRegistry {
    TriggerRegistry::build(|r| {
        r.register(Arc::new(MFADeviceAfterCreateHandler1::new()));
        r.register(Arc::new(MFADeviceAfterCreateHandler2::new()));
        r.register(Arc::new(MFADeviceAfterCreateHandler3::new()));
        r.register(Arc::new(MFADeviceOnEnterPendingHandler::new()));
        r.register(Arc::new(MFADeviceOnEnterActiveHandler::new()));
        r.register(Arc::new(MFADeviceOnEnterDisabledHandler::new()));
    })
}

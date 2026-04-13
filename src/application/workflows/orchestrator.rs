//! Workflow Orchestrator
//!
//! Generic workflow orchestration supporting:
//! - Multiple workflow types
//! - Persistent state management
//! - Event-driven execution
//! - Saga pattern (compensation on failure)
//! - Timeout handling
//! - Retry policies

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};

/// Workflow status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowStatus {
    Pending,
    Running,
    Waiting,
    Completed,
    Failed,
    Cancelled,
    Compensating,
    Compensated,
}

/// Workflow instance metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInstance {
    /// Unique workflow instance ID
    pub id: String,
    /// Workflow type name
    pub workflow_type: String,
    /// Current status
    pub status: WorkflowStatus,
    /// Current step name
    pub current_step: Option<String>,
    /// Workflow context/variables
    pub context: serde_json::Value,
    /// Completed steps
    pub completed_steps: Vec<String>,
    /// Step execution history with timestamps
    pub history: Vec<StepExecution>,
    /// Error message if failed
    pub error: Option<String>,
    /// Correlation ID for event matching
    pub correlation_id: Option<String>,
    /// Wait timeout (if waiting)
    pub wait_until: Option<DateTime<Utc>>,
    /// Retry count
    pub retry_count: u32,
    /// Maximum retries allowed
    pub max_retries: u32,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
    /// Parent workflow ID (for sub-workflows)
    pub parent_id: Option<String>,
}

/// Step execution record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepExecution {
    pub step_name: String,
    pub status: StepStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
    pub output: Option<serde_json::Value>,
}

/// Step execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepStatus {
    Running,
    Completed,
    Failed,
    Skipped,
    Compensated,
}

impl WorkflowInstance {
    /// Create a new workflow instance
    pub fn new(id: impl Into<String>, workflow_type: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            workflow_type: workflow_type.into(),
            status: WorkflowStatus::Pending,
            current_step: None,
            context: serde_json::json!({}),
            completed_steps: Vec::new(),
            history: Vec::new(),
            error: None,
            correlation_id: None,
            wait_until: None,
            retry_count: 0,
            max_retries: 3,
            created_at: now,
            updated_at: now,
            parent_id: None,
        }
    }

    /// Set a context variable
    pub fn set_context(&mut self, key: &str, value: serde_json::Value) {
        if let serde_json::Value::Object(ref mut map) = self.context {
            map.insert(key.to_string(), value);
        }
        self.updated_at = Utc::now();
    }

    /// Get a context variable
    pub fn get_context(&self, key: &str) -> Option<&serde_json::Value> {
        self.context.as_object().and_then(|m| m.get(key))
    }

    /// Check if workflow is complete
    pub fn is_complete(&self) -> bool {
        matches!(
            self.status,
            WorkflowStatus::Completed
                | WorkflowStatus::Failed
                | WorkflowStatus::Cancelled
                | WorkflowStatus::Compensated
        )
    }

    /// Check if workflow is running
    pub fn is_running(&self) -> bool {
        matches!(
            self.status,
            WorkflowStatus::Running | WorkflowStatus::Waiting
        )
    }

    /// Check if wait has timed out
    pub fn is_wait_expired(&self) -> bool {
        match self.wait_until {
            Some(wait_until) => Utc::now() > wait_until,
            None => false,
        }
    }

    /// Record step start
    pub fn start_step(&mut self, step_name: impl Into<String>) {
        let step = step_name.into();
        self.current_step = Some(step.clone());
        self.history.push(StepExecution {
            step_name: step,
            status: StepStatus::Running,
            started_at: Utc::now(),
            completed_at: None,
            error: None,
            output: None,
        });
        self.updated_at = Utc::now();
    }

    /// Record step completion
    pub fn complete_step(&mut self, output: Option<serde_json::Value>) {
        if let Some(execution) = self.history.last_mut() {
            execution.status = StepStatus::Completed;
            execution.completed_at = Some(Utc::now());
            execution.output = output;
            self.completed_steps.push(execution.step_name.clone());
        }
        self.current_step = None;
        self.updated_at = Utc::now();
    }

    /// Record step failure
    pub fn fail_step(&mut self, error: impl Into<String>) {
        if let Some(execution) = self.history.last_mut() {
            execution.status = StepStatus::Failed;
            execution.completed_at = Some(Utc::now());
            execution.error = Some(error.into());
        }
        self.updated_at = Utc::now();
    }

    /// Enter waiting state
    pub fn wait(&mut self, timeout: Duration, correlation_id: Option<String>) {
        self.status = WorkflowStatus::Waiting;
        self.wait_until = Some(Utc::now() + timeout);
        self.correlation_id = correlation_id;
        self.updated_at = Utc::now();
    }

    /// Check if should retry
    pub fn can_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }

    /// Increment retry count
    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
        self.updated_at = Utc::now();
    }
}

/// Workflow event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowEvent {
    pub event_type: String,
    pub correlation_id: Option<String>,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

impl WorkflowEvent {
    pub fn new(event_type: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            event_type: event_type.into(),
            correlation_id: None,
            payload,
            timestamp: Utc::now(),
        }
    }

    pub fn with_correlation_id(mut self, id: impl Into<String>) -> Self {
        self.correlation_id = Some(id.into());
        self
    }
}

/// Orchestrator errors
#[derive(Debug, Clone, thiserror::Error)]
pub enum OrchestratorError {
    #[error("Workflow not found: {0}")]
    WorkflowNotFound(String),

    #[error("Workflow type not registered: {0}")]
    WorkflowTypeNotRegistered(String),

    #[error("Step execution failed: {0}")]
    StepFailed(String),

    #[error("Compensation failed: {0}")]
    CompensationFailed(String),

    #[error("Workflow timed out")]
    Timeout,

    #[error("Maximum retries exceeded")]
    MaxRetriesExceeded,

    #[error("Invalid state transition: {from} -> {to}")]
    InvalidTransition { from: String, to: String },

    #[error("Persistence error: {0}")]
    PersistenceError(String),
}

/// Step result returned by step handlers
#[derive(Debug, Clone)]
pub enum StepResult {
    /// Move to the next step
    Next(String),
    /// Wait for an event
    Wait {
        timeout: Duration,
        correlation_id: Option<String>,
    },
    /// Complete the workflow
    Complete,
    /// Fail the workflow
    Fail(String),
    /// Retry the current step
    Retry,
    /// Skip to a specific step
    Skip(String),
}

/// Trait for workflow step handlers
#[async_trait::async_trait]
pub trait WorkflowHandler: Send + Sync {
    /// Get workflow type name
    fn workflow_type(&self) -> &str;

    /// Get the initial step name
    fn initial_step(&self) -> &str;

    /// Execute a step
    async fn execute_step(
        &self,
        step_name: &str,
        instance: &mut WorkflowInstance,
    ) -> Result<StepResult, OrchestratorError>;

    /// Handle an event for a waiting workflow
    async fn handle_event(
        &self,
        instance: &mut WorkflowInstance,
        event: &WorkflowEvent,
    ) -> Result<StepResult, OrchestratorError>;

    /// Execute compensation for the workflow
    async fn compensate(
        &self,
        instance: &mut WorkflowInstance,
    ) -> Result<(), OrchestratorError>;

    /// Get compensation order (steps to compensate in reverse order)
    fn compensation_order(&self) -> Vec<&str> {
        vec![]
    }
}

/// Trait for workflow persistence
#[async_trait::async_trait]
pub trait WorkflowRepository: Send + Sync {
    /// Save a workflow instance
    async fn save(&self, instance: &WorkflowInstance) -> Result<(), OrchestratorError>;

    /// Get a workflow instance by ID
    async fn get(&self, id: &str) -> Result<Option<WorkflowInstance>, OrchestratorError>;

    /// Find workflows waiting for an event
    async fn find_waiting(
        &self,
        correlation_id: &str,
    ) -> Result<Vec<WorkflowInstance>, OrchestratorError>;

    /// Find expired waiting workflows
    async fn find_expired(&self) -> Result<Vec<WorkflowInstance>, OrchestratorError>;

    /// List workflows by type and status
    async fn list(
        &self,
        workflow_type: Option<&str>,
        status: Option<WorkflowStatus>,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<WorkflowInstance>, OrchestratorError>;

    /// Delete a workflow instance
    async fn delete(&self, id: &str) -> Result<(), OrchestratorError>;
}

/// In-memory workflow repository for testing
pub struct InMemoryWorkflowRepository {
    instances: RwLock<HashMap<String, WorkflowInstance>>,
}

impl InMemoryWorkflowRepository {
    pub fn new() -> Self {
        Self {
            instances: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryWorkflowRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl WorkflowRepository for InMemoryWorkflowRepository {
    async fn save(&self, instance: &WorkflowInstance) -> Result<(), OrchestratorError> {
        self.instances
            .write()
            .await
            .insert(instance.id.clone(), instance.clone());
        Ok(())
    }

    async fn get(&self, id: &str) -> Result<Option<WorkflowInstance>, OrchestratorError> {
        Ok(self.instances.read().await.get(id).cloned())
    }

    async fn find_waiting(
        &self,
        correlation_id: &str,
    ) -> Result<Vec<WorkflowInstance>, OrchestratorError> {
        let instances = self.instances.read().await;
        Ok(instances
            .values()
            .filter(|i| {
                i.status == WorkflowStatus::Waiting
                    && i.correlation_id.as_deref() == Some(correlation_id)
            })
            .cloned()
            .collect())
    }

    async fn find_expired(&self) -> Result<Vec<WorkflowInstance>, OrchestratorError> {
        let now = Utc::now();
        let instances = self.instances.read().await;
        Ok(instances
            .values()
            .filter(|i| {
                i.status == WorkflowStatus::Waiting
                    && i.wait_until.map(|t| t < now).unwrap_or(false)
            })
            .cloned()
            .collect())
    }

    async fn list(
        &self,
        workflow_type: Option<&str>,
        status: Option<WorkflowStatus>,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<WorkflowInstance>, OrchestratorError> {
        let instances = self.instances.read().await;
        let filtered: Vec<_> = instances
            .values()
            .filter(|i| {
                workflow_type.map_or(true, |t| i.workflow_type == t)
                    && status.map_or(true, |s| i.status == s)
            })
            .skip(offset)
            .take(limit)
            .cloned()
            .collect();
        Ok(filtered)
    }

    async fn delete(&self, id: &str) -> Result<(), OrchestratorError> {
        self.instances.write().await.remove(id);
        Ok(())
    }
}

/// Workflow Orchestrator
pub struct WorkflowOrchestrator {
    handlers: RwLock<HashMap<String, Arc<dyn WorkflowHandler>>>,
    repository: Arc<dyn WorkflowRepository>,
}

impl WorkflowOrchestrator {
    /// Create a new orchestrator with a repository
    pub fn new(repository: Arc<dyn WorkflowRepository>) -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
            repository,
        }
    }

    /// Register a workflow handler
    pub async fn register_handler(&self, handler: Arc<dyn WorkflowHandler>) {
        let workflow_type = handler.workflow_type().to_string();
        self.handlers.write().await.insert(workflow_type, handler);
    }

    /// Start a new workflow
    pub async fn start_workflow(
        &self,
        workflow_type: &str,
        instance_id: impl Into<String>,
        initial_context: serde_json::Value,
    ) -> Result<WorkflowInstance, OrchestratorError> {
        let handlers = self.handlers.read().await;
        let handler = handlers
            .get(workflow_type)
            .ok_or_else(|| OrchestratorError::WorkflowTypeNotRegistered(workflow_type.to_string()))?;

        let mut instance = WorkflowInstance::new(instance_id, workflow_type);
        instance.context = initial_context;
        instance.status = WorkflowStatus::Running;
        instance.current_step = Some(handler.initial_step().to_string());

        self.repository.save(&instance).await?;
        Ok(instance)
    }

    /// Execute the current step of a workflow
    pub async fn execute_step(
        &self,
        instance_id: &str,
    ) -> Result<WorkflowInstance, OrchestratorError> {
        let mut instance = self
            .repository
            .get(instance_id)
            .await?
            .ok_or_else(|| OrchestratorError::WorkflowNotFound(instance_id.to_string()))?;

        if instance.is_complete() {
            return Ok(instance);
        }

        let handlers = self.handlers.read().await;
        let handler = handlers
            .get(&instance.workflow_type)
            .ok_or_else(|| {
                OrchestratorError::WorkflowTypeNotRegistered(instance.workflow_type.clone())
            })?;

        let step_name = match &instance.current_step {
            Some(step) => step.clone(),
            None => return Ok(instance),
        };

        instance.start_step(&step_name);

        let result = handler.execute_step(&step_name, &mut instance).await;

        match result {
            Ok(StepResult::Next(next_step)) => {
                instance.complete_step(None);
                instance.current_step = Some(next_step);
                instance.status = WorkflowStatus::Running;
            }
            Ok(StepResult::Wait {
                timeout,
                correlation_id,
            }) => {
                instance.complete_step(None);
                instance.wait(timeout, correlation_id);
            }
            Ok(StepResult::Complete) => {
                instance.complete_step(None);
                instance.current_step = None;
                instance.status = WorkflowStatus::Completed;
            }
            Ok(StepResult::Fail(error)) => {
                instance.fail_step(&error);
                instance.error = Some(error);
                instance.status = WorkflowStatus::Failed;
            }
            Ok(StepResult::Retry) => {
                if instance.can_retry() {
                    instance.increment_retry();
                    instance.status = WorkflowStatus::Running;
                } else {
                    instance.fail_step("Maximum retries exceeded");
                    instance.error = Some("Maximum retries exceeded".to_string());
                    instance.status = WorkflowStatus::Failed;
                }
            }
            Ok(StepResult::Skip(skip_to)) => {
                instance.complete_step(None);
                instance.current_step = Some(skip_to);
                instance.status = WorkflowStatus::Running;
            }
            Err(e) => {
                instance.fail_step(&e.to_string());
                instance.error = Some(e.to_string());
                instance.status = WorkflowStatus::Failed;
            }
        }

        self.repository.save(&instance).await?;
        Ok(instance)
    }

    /// Run a workflow to completion (or until waiting)
    pub async fn run_workflow(
        &self,
        instance_id: &str,
    ) -> Result<WorkflowInstance, OrchestratorError> {
        loop {
            let instance = self.execute_step(instance_id).await?;
            if instance.is_complete() || instance.status == WorkflowStatus::Waiting {
                return Ok(instance);
            }
        }
    }

    /// Handle an event and resume any waiting workflows
    pub async fn handle_event(
        &self,
        event: &WorkflowEvent,
    ) -> Result<Vec<WorkflowInstance>, OrchestratorError> {
        let correlation_id = match &event.correlation_id {
            Some(id) => id,
            None => return Ok(vec![]),
        };

        let waiting = self.repository.find_waiting(correlation_id).await?;
        let mut resumed = Vec::new();

        for mut instance in waiting {
            let handlers = self.handlers.read().await;
            if let Some(handler) = handlers.get(&instance.workflow_type) {
                let result = handler.handle_event(&mut instance, event).await;

                match result {
                    Ok(StepResult::Next(next_step)) => {
                        instance.current_step = Some(next_step);
                        instance.status = WorkflowStatus::Running;
                        instance.wait_until = None;
                        instance.correlation_id = None;
                    }
                    Ok(StepResult::Complete) => {
                        instance.status = WorkflowStatus::Completed;
                        instance.wait_until = None;
                        instance.correlation_id = None;
                    }
                    Ok(StepResult::Fail(error)) => {
                        instance.error = Some(error);
                        instance.status = WorkflowStatus::Failed;
                    }
                    _ => {}
                }

                self.repository.save(&instance).await?;
                resumed.push(instance);
            }
        }

        Ok(resumed)
    }

    /// Process expired waiting workflows
    pub async fn process_expired(&self) -> Result<Vec<WorkflowInstance>, OrchestratorError> {
        let expired = self.repository.find_expired().await?;
        let mut processed = Vec::new();

        for mut instance in expired {
            instance.fail_step("Wait timeout expired");
            instance.error = Some("Workflow wait timeout expired".to_string());
            instance.status = WorkflowStatus::Failed;
            self.repository.save(&instance).await?;
            processed.push(instance);
        }

        Ok(processed)
    }

    /// Compensate a failed workflow (saga rollback)
    pub async fn compensate(
        &self,
        instance_id: &str,
    ) -> Result<WorkflowInstance, OrchestratorError> {
        let mut instance = self
            .repository
            .get(instance_id)
            .await?
            .ok_or_else(|| OrchestratorError::WorkflowNotFound(instance_id.to_string()))?;

        if instance.status != WorkflowStatus::Failed {
            return Err(OrchestratorError::InvalidTransition {
                from: format!("{:?}", instance.status),
                to: "Compensating".to_string(),
            });
        }

        instance.status = WorkflowStatus::Compensating;
        self.repository.save(&instance).await?;

        let handlers = self.handlers.read().await;
        let handler = handlers
            .get(&instance.workflow_type)
            .ok_or_else(|| {
                OrchestratorError::WorkflowTypeNotRegistered(instance.workflow_type.clone())
            })?;

        match handler.compensate(&mut instance).await {
            Ok(()) => {
                instance.status = WorkflowStatus::Compensated;
            }
            Err(e) => {
                instance.error = Some(format!("Compensation failed: {}", e));
            }
        }

        self.repository.save(&instance).await?;
        Ok(instance)
    }

    /// Cancel a workflow
    pub async fn cancel_workflow(
        &self,
        instance_id: &str,
    ) -> Result<WorkflowInstance, OrchestratorError> {
        let mut instance = self
            .repository
            .get(instance_id)
            .await?
            .ok_or_else(|| OrchestratorError::WorkflowNotFound(instance_id.to_string()))?;

        if instance.is_complete() {
            return Err(OrchestratorError::InvalidTransition {
                from: format!("{:?}", instance.status),
                to: "Cancelled".to_string(),
            });
        }

        instance.status = WorkflowStatus::Cancelled;
        instance.updated_at = Utc::now();
        self.repository.save(&instance).await?;
        Ok(instance)
    }

    /// Get a workflow instance
    pub async fn get_workflow(
        &self,
        instance_id: &str,
    ) -> Result<Option<WorkflowInstance>, OrchestratorError> {
        self.repository.get(instance_id).await
    }

    /// List workflows
    pub async fn list_workflows(
        &self,
        workflow_type: Option<&str>,
        status: Option<WorkflowStatus>,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<WorkflowInstance>, OrchestratorError> {
        self.repository.list(workflow_type, status, limit, offset).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestWorkflowHandler;

    #[async_trait::async_trait]
    impl WorkflowHandler for TestWorkflowHandler {
        fn workflow_type(&self) -> &str {
            "test_workflow"
        }

        fn initial_step(&self) -> &str {
            "step_1"
        }

        async fn execute_step(
            &self,
            step_name: &str,
            instance: &mut WorkflowInstance,
        ) -> Result<StepResult, OrchestratorError> {
            match step_name {
                "step_1" => {
                    instance.set_context("step_1_done", serde_json::json!(true));
                    Ok(StepResult::Next("step_2".to_string()))
                }
                "step_2" => {
                    instance.set_context("step_2_done", serde_json::json!(true));
                    Ok(StepResult::Next("step_3".to_string()))
                }
                "step_3" => Ok(StepResult::Complete),
                _ => Ok(StepResult::Fail(format!("Unknown step: {}", step_name))),
            }
        }

        async fn handle_event(
            &self,
            _instance: &mut WorkflowInstance,
            _event: &WorkflowEvent,
        ) -> Result<StepResult, OrchestratorError> {
            Ok(StepResult::Complete)
        }

        async fn compensate(
            &self,
            instance: &mut WorkflowInstance,
        ) -> Result<(), OrchestratorError> {
            instance.set_context("compensated", serde_json::json!(true));
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_workflow_execution() {
        let repository = Arc::new(InMemoryWorkflowRepository::new());
        let orchestrator = WorkflowOrchestrator::new(repository);
        orchestrator
            .register_handler(Arc::new(TestWorkflowHandler))
            .await;

        // Start workflow
        let instance = orchestrator
            .start_workflow("test_workflow", "test-1", serde_json::json!({}))
            .await
            .unwrap();
        assert_eq!(instance.status, WorkflowStatus::Running);
        assert_eq!(instance.current_step, Some("step_1".to_string()));

        // Run to completion
        let instance = orchestrator.run_workflow("test-1").await.unwrap();
        assert_eq!(instance.status, WorkflowStatus::Completed);
        assert!(instance.current_step.is_none());
        assert_eq!(instance.completed_steps.len(), 3);
    }

    #[tokio::test]
    async fn test_workflow_cancellation() {
        let repository = Arc::new(InMemoryWorkflowRepository::new());
        let orchestrator = WorkflowOrchestrator::new(repository);
        orchestrator
            .register_handler(Arc::new(TestWorkflowHandler))
            .await;

        let _instance = orchestrator
            .start_workflow("test_workflow", "test-cancel", serde_json::json!({}))
            .await
            .unwrap();

        let instance = orchestrator.cancel_workflow("test-cancel").await.unwrap();
        assert_eq!(instance.status, WorkflowStatus::Cancelled);
    }

    #[tokio::test]
    async fn test_workflow_not_found() {
        let repository = Arc::new(InMemoryWorkflowRepository::new());
        let orchestrator = WorkflowOrchestrator::new(repository);

        let result = orchestrator.execute_step("nonexistent").await;
        assert!(matches!(result, Err(OrchestratorError::WorkflowNotFound(_))));
    }

    #[test]
    fn test_workflow_instance_context() {
        let mut instance = WorkflowInstance::new("test", "test_workflow");
        instance.set_context("key", serde_json::json!("value"));
        assert_eq!(instance.get_context("key"), Some(&serde_json::json!("value")));
    }

    #[test]
    fn test_workflow_instance_status() {
        let mut instance = WorkflowInstance::new("test", "test_workflow");
        assert!(!instance.is_complete());
        assert!(!instance.is_running());

        instance.status = WorkflowStatus::Running;
        assert!(instance.is_running());

        instance.status = WorkflowStatus::Completed;
        assert!(instance.is_complete());
    }

    #[test]
    fn test_step_execution_record() {
        let mut instance = WorkflowInstance::new("test", "test_workflow");
        instance.start_step("step_1");
        assert_eq!(instance.history.len(), 1);
        assert_eq!(instance.history[0].status, StepStatus::Running);

        instance.complete_step(Some(serde_json::json!({"result": "ok"})));
        assert_eq!(instance.history[0].status, StepStatus::Completed);
        assert!(instance.history[0].completed_at.is_some());
    }
}

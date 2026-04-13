//! Permission Context value object
//!
//! Provides context information for permission evaluation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Permission context for evaluating access control
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionContext {
    pub user_id: Uuid,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub action: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub session_id: Option<Uuid>,
    pub additional_context: HashMap<String, serde_json::Value>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl PermissionContext {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            resource_type: None,
            resource_id: None,
            action: None,
            ip_address: None,
            user_agent: None,
            session_id: None,
            additional_context: HashMap::new(),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Set resource context
    pub fn with_resource(mut self, resource_type: String, resource_id: Uuid) -> Self {
        self.resource_type = Some(resource_type);
        self.resource_id = Some(resource_id);
        self
    }

    /// Set action context
    pub fn with_action(mut self, action: String) -> Self {
        self.action = Some(action);
        self
    }

    /// Set request context
    pub fn with_request_context(mut self, ip_address: String, user_agent: String) -> Self {
        self.ip_address = Some(ip_address);
        self.user_agent = Some(user_agent);
        self
    }

    /// Set session context
    pub fn with_session(mut self, session_id: Uuid) -> Self {
        self.session_id = Some(session_id);
        self
    }

    /// Add additional context
    pub fn with_additional_context(mut self, key: String, value: serde_json::Value) -> Self {
        self.additional_context.insert(key, value);
        self
    }

    /// Get additional context value
    pub fn get_additional_context(&self, key: &str) -> Option<&serde_json::Value> {
        self.additional_context.get(key)
    }

    /// Check if context includes resource information
    pub fn has_resource_context(&self) -> bool {
        self.resource_type.is_some() && self.resource_id.is_some()
    }

    /// Check if context includes action information
    pub fn has_action_context(&self) -> bool {
        self.action.is_some()
    }

    /// Check if context includes request information
    pub fn has_request_context(&self) -> bool {
        self.ip_address.is_some() || self.user_agent.is_some()
    }

    /// Create a simple context for user-level permission checks
    pub fn for_user(user_id: Uuid) -> Self {
        Self::new(user_id)
    }

    /// Create a context for resource-level permission checks
    pub fn for_resource(user_id: Uuid, resource_type: String, resource_id: Uuid, action: String) -> Self {
        Self::new(user_id)
            .with_resource(resource_type, resource_id)
            .with_action(action)
    }
}

impl Default for PermissionContext {
    fn default() -> Self {
        Self::new(Uuid::new_v4())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_context_creation() {
        let user_id = Uuid::new_v4();
        let context = PermissionContext::new(user_id);

        assert_eq!(context.user_id, user_id);
        assert!(!context.has_resource_context());
        assert!(!context.has_action_context());
    }

    #[test]
    fn test_permission_context_with_resource() {
        let user_id = Uuid::new_v4();
        let resource_type = "document".to_string();
        let resource_id = Uuid::new_v4();
        let action = "read".to_string();

        let context = PermissionContext::for_resource(
            user_id,
            resource_type.clone(),
            resource_id,
            action.clone(),
        );

        assert_eq!(context.user_id, user_id);
        assert_eq!(context.resource_type, Some(resource_type));
        assert_eq!(context.resource_id, Some(resource_id));
        assert_eq!(context.action, Some(action));
        assert!(context.has_resource_context());
        assert!(context.has_action_context());
    }

    #[test]
    fn test_permission_context_with_request() {
        let user_id = Uuid::new_v4();
        let ip_address = "192.168.1.1".to_string();
        let user_agent = "Mozilla/5.0...".to_string();

        let context = PermissionContext::new(user_id)
            .with_request_context(ip_address.clone(), user_agent.clone());

        assert_eq!(context.user_id, user_id);
        assert_eq!(context.ip_address, Some(ip_address));
        assert_eq!(context.user_agent, Some(user_agent));
        assert!(context.has_request_context());
    }

    #[test]
    fn test_additional_context() {
        let user_id = Uuid::new_v4();
        let key = "department".to_string();
        let value = serde_json::Value::String("engineering".to_string());

        let context = PermissionContext::new(user_id)
            .with_additional_context(key.clone(), value.clone());

        assert_eq!(context.get_additional_context(&key), Some(&value));
    }
}
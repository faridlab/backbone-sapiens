//! Conflict Resolution entity
//!
//! Handles conflict resolution strategies for account merging and data conflicts.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Conflict resolution strategy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Keep target value
    KeepTarget,
    /// Keep source value
    KeepSource,
    /// Keep both values (for multi-value fields)
    KeepBoth,
    /// Merge values intelligently
    MergeValues,
    /// Manual resolution required
    ManualResolution,
    /// Timestamp-based resolution (keep newest/oldest)
    TimestampBased,
}

/// Conflict type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConflictType {
    /// Data value conflict
    DataConflict,
    /// Duplicate record
    DuplicateRecord,
    /// Reference conflict
    ReferenceConflict,
    /// Validation conflict
    ValidationConflict,
    /// Business rule conflict
    BusinessRuleConflict,
}

/// Conflict resolution result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConflictResolutionResult {
    pub conflict_id: Uuid,
    pub field_name: String,
    pub conflict_type: ConflictType,
    pub resolution: ConflictResolution,
    pub source_value: Option<serde_json::Value>,
    pub target_value: Option<serde_json::Value>,
    pub resolved_value: Option<serde_json::Value>,
    pub requires_manual_review: bool,
    pub resolved_by_user_id: Option<Uuid>,
    pub resolution_notes: Option<String>,
}

impl ConflictResolutionResult {
    pub fn new(
        conflict_id: Uuid,
        field_name: String,
        conflict_type: ConflictType,
        source_value: Option<serde_json::Value>,
        target_value: Option<serde_json::Value>,
    ) -> Self {
        Self {
            conflict_id,
            field_name,
            conflict_type,
            resolution: ConflictResolution::ManualResolution,
            source_value,
            target_value,
            resolved_value: None,
            requires_manual_review: true,
            resolved_by_user_id: None,
            resolution_notes: None,
        }
    }

    /// Set resolution strategy
    pub fn with_resolution(mut self, resolution: ConflictResolution) -> Self {
        self.resolution = resolution.clone();
        self.requires_manual_review = matches!(resolution, ConflictResolution::ManualResolution);
        self
    }

    /// Set resolved value
    pub fn with_resolved_value(mut self, value: serde_json::Value) -> Self {
        self.resolved_value = Some(value);
        self
    }

    /// Mark as resolved by admin
    pub fn resolved_by_admin(mut self, admin_user_id: Uuid, notes: String) -> Self {
        self.resolved_by_user_id = Some(admin_user_id);
        self.resolution_notes = Some(notes);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conflict_resolution() {
        let conflict_id = Uuid::new_v4();
        let source_value = serde_json::Value::String("old_value".to_string());
        let target_value = serde_json::Value::String("new_value".to_string());

        let result = ConflictResolutionResult::new(
            conflict_id,
            "email".to_string(),
            ConflictType::DataConflict,
            Some(source_value.clone()),
            Some(target_value.clone()),
        );

        assert_eq!(result.field_name, "email");
        assert_eq!(result.conflict_type, ConflictType::DataConflict);
        assert_eq!(result.source_value, Some(source_value));
        assert_eq!(result.target_value, Some(target_value));
        assert!(result.requires_manual_review);
    }

    #[test]
    fn test_resolution_strategies() {
        assert_eq!(ConflictResolution::KeepTarget, ConflictResolution::KeepTarget);
        assert_ne!(ConflictResolution::KeepTarget, ConflictResolution::KeepSource);
    }
}
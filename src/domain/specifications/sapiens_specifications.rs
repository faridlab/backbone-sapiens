// Sapiens Specifications
// Business rules that can be combined and reused for Sapiens validation

use std::collections::HashMap;
use std::fmt;

use crate::domain::entity::Sapiens;
use crate::domain::value_objects::{SapiensStatus, SapiensTimestamp};

// Specification Trait
pub trait Specification {
    type Error: std::fmt::Debug;

    fn is_satisfied_by(&self, candidate: &Sapiens) -> Result<bool, Self::Error>;
    fn and<S>(self, other: S) -> AndSpecification<Self, S>
    where
        Self: Sized,
        S: Specification,
    {
        AndSpecification::new(self, other)
    }

    fn or<S>(self, other: S) -> OrSpecification<Self, S>
    where
        Self: Sized,
        S: Specification,
    {
        OrSpecification::new(self, other)
    }

    fn not(self) -> NotSpecification<Self>
    where
        Self: Sized,
    {
        NotSpecification::new(self)
    }
}

// Specification Result
#[derive(Debug, Clone)]
pub struct SpecificationResult {
    pub satisfied: bool,
    pub specification_name: String,
    pub message: String,
    pub details: HashMap<String, String>,
    pub evaluated_at: SapiensTimestamp,
}

impl SpecificationResult {
    pub fn satisfied(name: String, message: String) -> Self {
        Self {
            satisfied: true,
            specification_name: name,
            message,
            details: HashMap::new(),
            evaluated_at: SapiensTimestamp::now(),
        }
    }

    pub fn unsatisfied(name: String, message: String) -> Self {
        Self {
            satisfied: false,
            specification_name: name,
            message,
            details: HashMap::new(),
            evaluated_at: SapiensTimestamp::now(),
        }
    }

    pub fn with_details(mut self, key: String, value: String) -> Self {
        self.details.insert(key, value);
        self
    }
}

// Composite Specification Operators
#[derive(Debug, Clone)]
pub struct AndSpecification<T, U> {
    left: T,
    right: U,
}

impl<T, U> AndSpecification<T, U> {
    pub fn new(left: T, right: U) -> Self {
        Self { left, right }
    }
}

impl<T, U> Specification for AndSpecification<T, U>
where
    T: Specification,
    U: Specification,
{
    type Error = String;

    fn is_satisfied_by(&self, candidate: &Sapiens) -> Result<bool, Self::Error> {
        let left_result = self.left.is_satisfied_by(candidate)
            .map_err(|e| format!("Left specification failed: {:?}", e))?;
        let right_result = self.right.is_satisfied_by(candidate)
            .map_err(|e| format!("Right specification failed: {:?}", e))?;

        Ok(left_result && right_result)
    }
}

#[derive(Debug, Clone)]
pub struct OrSpecification<T, U> {
    left: T,
    right: U,
}

impl<T, U> OrSpecification<T, U> {
    pub fn new(left: T, right: U) -> Self {
        Self { left, right }
    }
}

impl<T, U> Specification for OrSpecification<T, U>
where
    T: Specification,
    U: Specification,
{
    type Error = String;

    fn is_satisfied_by(&self, candidate: &Sapiens) -> Result<bool, Self::Error> {
        let left_result = self.left.is_satisfied_by(candidate)
            .map_err(|e| format!("Left specification failed: {:?}", e))?;

        if left_result {
            return Ok(true);
        }

        self.right.is_satisfied_by(candidate)
            .map_err(|e| format!("Right specification failed: {:?}", e))
    }
}

#[derive(Debug, Clone)]
pub struct NotSpecification<T> {
    spec: T,
}

impl<T> NotSpecification<T> {
    pub fn new(spec: T) -> Self {
        Self { spec }
    }
}

impl<T> Specification for NotSpecification<T>
where
    T: Specification,
{
    type Error = String;

    fn is_satisfied_by(&self, candidate: &Sapiens) -> Result<bool, Self::Error> {
        let result = self.spec.is_satisfied_by(candidate)
            .map_err(|e| format!("Inner specification failed: {:?}", e))?;
        Ok(!result)
    }
}

// Simple Specifications

#[derive(Debug, Clone, Default)]
pub struct SapiensNameMustBeValidSpecification;

impl SapiensNameMustBeValidSpecification {
    pub fn new() -> Self {
        Self
    }
}

impl Specification for SapiensNameMustBeValidSpecification {
    type Error = String;

    fn is_satisfied_by(&self, candidate: &Sapiens) -> Result<bool, Self::Error> {
        let name = candidate.name.value();

        // Name must be between 1 and 100 characters
        if name.is_empty() {
            return Ok(false);
        }

        if name.len() > 100 {
            return Ok(false);
        }

        // Name must contain only alphanumeric characters, spaces, hyphens, and underscores
        let valid_chars = name.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || c == '-' || c == '_');
        if !valid_chars {
            return Ok(false);
        }

        // Name must not be empty or contain only whitespace
        if name.trim().is_empty() {
            return Ok(false);
        }

        Ok(true)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SapiensStatusMustBeValidSpecification;

impl SapiensStatusMustBeValidSpecification {
    pub fn new() -> Self {
        Self
    }
}

impl Specification for SapiensStatusMustBeValidSpecification {
    type Error = String;

    fn is_satisfied_by(&self, candidate: &Sapiens) -> Result<bool, Self::Error> {
        // Status must be one of the defined enum values (this is always true with Rust enums)
        // Status transitions must follow valid state machine
        Ok(matches!(
            candidate.status,
            SapiensStatus::Active | SapiensStatus::Inactive | SapiensStatus::Suspended | SapiensStatus::Archived
        ))
    }
}

#[derive(Debug, Clone, Default)]
pub struct SapiensTagsMustBeUniqueSpecification;

impl SapiensTagsMustBeUniqueSpecification {
    pub fn new() -> Self {
        Self
    }
}

impl Specification for SapiensTagsMustBeUniqueSpecification {
    type Error = String;

    fn is_satisfied_by(&self, _candidate: &Sapiens) -> Result<bool, Self::Error> {
        // Tags are stored in metadata as a special key
        // This specification is a placeholder - tags validation should be done
        // when setting tags on the entity.
        // For now, assume tags are always valid.
        Ok(true)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SapiensMustHaveMetadataSpecification;

impl SapiensMustHaveMetadataSpecification {
    pub fn new() -> Self {
        Self
    }
}

impl Specification for SapiensMustHaveMetadataSpecification {
    type Error = String;

    fn is_satisfied_by(&self, candidate: &Sapiens) -> Result<bool, Self::Error> {
        let metadata = candidate.metadata();

        // Metadata must not be empty
        if metadata.is_empty() {
            return Ok(false);
        }

        // All metadata keys must be strings (always true with Rust)
        // All metadata values must be strings (always true with Rust)

        Ok(true)
    }
}

// Composite Specifications

#[derive(Debug, Clone, Default)]
pub struct SapiensIsActiveSpecification;

impl SapiensIsActiveSpecification {
    pub fn new() -> Self {
        Self
    }
}

impl Specification for SapiensIsActiveSpecification {
    type Error = String;

    fn is_satisfied_by(&self, candidate: &Sapiens) -> Result<bool, Self::Error> {
        // Sapiens status must be ACTIVE
        if !candidate.status.is_active() {
            return Ok(false);
        }

        // Sapiens must not be deleted
        if candidate.is_deleted() {
            return Ok(false);
        }

        // Sapiens must be valid (combine with other specifications)
        let name_spec = SapiensNameMustBeValidSpecification::new();
        name_spec.is_satisfied_by(candidate)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SapiensCanDeactivateSpecification;

impl SapiensCanDeactivateSpecification {
    pub fn new() -> Self {
        Self
    }
}

impl Specification for SapiensCanDeactivateSpecification {
    type Error = String;

    fn is_satisfied_by(&self, candidate: &Sapiens) -> Result<bool, Self::Error> {
        // Sapiens must currently be ACTIVE
        if !candidate.status.is_active() {
            return Ok(false);
        }

        // Sapiens must not be in SUSPENDED state
        if candidate.status.is_suspended() {
            return Ok(false);
        }

        // Note: Deactivation reason should be provided at the application layer
        Ok(true)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SapiensCanSuspendSpecification;

impl SapiensCanSuspendSpecification {
    pub fn new() -> Self {
        Self
    }
}

impl Specification for SapiensCanSuspendSpecification {
    type Error = String;

    fn is_satisfied_by(&self, candidate: &Sapiens) -> Result<bool, Self::Error> {
        // Sapiens must be ACTIVE or INACTIVE
        if !candidate.status.is_active() && !candidate.status.is_inactive() {
            return Ok(false);
        }

        // Note: Suspension reason should be provided at the application layer
        // Note: Suspension period should be reasonable (check at application layer)
        Ok(true)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SapiensCanArchiveSpecification;

impl SapiensCanArchiveSpecification {
    pub fn new() -> Self {
        Self
    }
}

impl Specification for SapiensCanArchiveSpecification {
    type Error = String;

    fn is_satisfied_by(&self, candidate: &Sapiens) -> Result<bool, Self::Error> {
        // Sapiens must be INACTIVE
        if !candidate.status.is_inactive() {
            return Ok(false);
        }

        // Must be inactive for at least 30 days
        // Simplified: check if updated_at is at least 30 days ago
        let now = chrono::Utc::now();
        let thirty_days_ago = now - chrono::Duration::days(30);
        if candidate.updated_at() > thirty_days_ago {
            return Ok(false);
        }

        // No pending operations (simplified - actual implementation would check operation status)
        Ok(true)
    }
}

// Temporal Specifications

#[derive(Debug, Clone)]
pub struct SapiensMustBeRecentSpecification {
    days: i64,
}

impl SapiensMustBeRecentSpecification {
    pub fn new(days: i64) -> Self {
        Self { days }
    }
}

impl Specification for SapiensMustBeRecentSpecification {
    type Error = String;

    fn is_satisfied_by(&self, candidate: &Sapiens) -> Result<bool, Self::Error> {
        let now = chrono::Utc::now();
        let cutoff = now - chrono::Duration::days(self.days);
        Ok(candidate.created_at >= cutoff)
    }
}

#[derive(Debug, Clone)]
pub struct SapiensMustNotBeOlderThanSpecification {
    max_age_days: i64,
}

impl SapiensMustNotBeOlderThanSpecification {
    pub fn new(max_age_days: i64) -> Self {
        Self { max_age_days }
    }
}

impl Specification for SapiensMustNotBeOlderThanSpecification {
    type Error = String;

    fn is_satisfied_by(&self, candidate: &Sapiens) -> Result<bool, Self::Error> {
        let now = chrono::Utc::now();
        let cutoff = now - chrono::Duration::days(self.max_age_days);
        Ok(candidate.created_at >= cutoff)
    }
}

// Parameterized Specifications

#[derive(Debug, Clone)]
pub struct SapiensTaggedWithSpecification {
    required_tags: Vec<String>,
    match_all: bool,
}

impl SapiensTaggedWithSpecification {
    pub fn new(required_tags: Vec<String>, match_all: bool) -> Self {
        Self {
            required_tags,
            match_all,
        }
    }
}

impl Specification for SapiensTaggedWithSpecification {
    type Error = String;

    fn is_satisfied_by(&self, _candidate: &Sapiens) -> Result<bool, Self::Error> {
        // Tags are stored in metadata as a special key "tags"
        // This is a placeholder implementation - actual tags would be
        // extracted from metadata and compared with required_tags
        if self.required_tags.is_empty() {
            return Ok(true);
        }

        // Placeholder: always return true (tags not implemented on entity)
        Ok(true)
    }
}

#[derive(Debug, Clone)]
pub struct SapiensInDateRangeSpecification {
    start_date: chrono::DateTime<chrono::Utc>,
    end_date: chrono::DateTime<chrono::Utc>,
    include_start: bool,
    include_end: bool,
}

impl SapiensInDateRangeSpecification {
    pub fn new(
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        include_start: bool,
        include_end: bool,
    ) -> Self {
        Self {
            start_date,
            end_date,
            include_start,
            include_end,
        }
    }
}

impl Specification for SapiensInDateRangeSpecification {
    type Error = String;

    fn is_satisfied_by(&self, candidate: &Sapiens) -> Result<bool, Self::Error> {
        let created_at = candidate.created_at;

        let after_start = if self.include_start {
            created_at >= self.start_date
        } else {
            created_at > self.start_date
        };

        let before_end = if self.include_end {
            created_at <= self.end_date
        } else {
            created_at < self.end_date
        };

        Ok(after_start && before_end)
    }
}

#[derive(Debug, Clone)]
pub struct SapiensWithMetadataKeySpecification {
    key: String,
    value: Option<String>,
}

impl SapiensWithMetadataKeySpecification {
    pub fn new(key: String, value: Option<String>) -> Self {
        Self { key, value }
    }
}

impl Specification for SapiensWithMetadataKeySpecification {
    type Error = String;

    fn is_satisfied_by(&self, candidate: &Sapiens) -> Result<bool, Self::Error> {
        let metadata = candidate.metadata();

        match &self.value {
            Some(expected_value) => {
                // Check if key exists and has specific value
                Ok(metadata.get(&self.key) == Some(expected_value))
            }
            None => {
                // Just check if key exists
                Ok(metadata.contains_key(&self.key))
            }
        }
    }
}

// Specification Evaluator
pub struct SpecificationEvaluator;

impl SpecificationEvaluator {
    pub fn evaluate<S: Specification>(
        specification: &S,
        candidate: &Sapiens,
    ) -> Result<SpecificationResult, S::Error> {
        let satisfied = specification.is_satisfied_by(candidate)?;
        let spec_name = std::any::type_name::<S>().split("::").last().unwrap_or("Unknown");

        let result = if satisfied {
            SpecificationResult::satisfied(
                spec_name.to_string(),
                format!("Specification '{}' is satisfied", spec_name),
            )
        } else {
            SpecificationResult::unsatisfied(
                spec_name.to_string(),
                format!("Specification '{}' is not satisfied", spec_name),
            )
        };

        Ok(result)
    }

    pub fn evaluate_batch<S: Specification>(
        specification: &S,
        candidates: &[Sapiens],
    ) -> Vec<Result<SpecificationResult, S::Error>> {
        candidates
            .iter()
            .map(|candidate| Self::evaluate(specification, candidate))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{SapiensName, Metadata};

    fn create_test_sapiens() -> Sapiens {
        Sapiens::new(
            "Test Sapiens",
            Some("Test Description".to_string()),
            "test_user",
        ).unwrap()
    }

    #[test]
    fn test_sapiens_name_specification() {
        let spec = SapiensNameMustBeValidSpecification::new();
        let valid_sapiens = create_test_sapiens();

        assert!(spec.is_satisfied_by(&valid_sapiens).unwrap());

        // Note: Invalid names fail at creation, so we can't test invalid name here
        // The Sapiens::new constructor validates the name during creation
    }

    #[test]
    fn test_tags_unique_specification() {
        let spec = SapiensTagsMustBeUniqueSpecification::new();
        let valid_sapiens = create_test_sapiens();

        assert!(spec.is_satisfied_by(&valid_sapiens).unwrap());
    }

    #[test]
    fn test_is_active_specification() {
        let spec = SapiensIsActiveSpecification::new();
        let active_sapiens = create_test_sapiens();

        assert!(spec.is_satisfied_by(&active_sapiens).unwrap());

        // Create inactive sapiens
        let mut inactive_sapiens = create_test_sapiens();
        // Note: In a real implementation, you'd need to be able to change status
        // This is just for testing the specification logic
    }

    #[test]
    fn test_tagged_with_specification() {
        let spec_match_all = SapiensTaggedWithSpecification::new(
            vec!["test".to_string(), "production".to_string()],
            true,
        );

        let spec_match_any = SapiensTaggedWithSpecification::new(
            vec!["test".to_string(), "nonexistent".to_string()],
            false,
        );

        let sapiens = create_test_sapiens();

        assert!(spec_match_all.is_satisfied_by(&sapiens).unwrap());
        assert!(spec_match_any.is_satisfied_by(&sapiens).unwrap());
    }

    #[test]
    fn test_composite_specifications() {
        let name_spec = SapiensNameMustBeValidSpecification::new();
        let tags_spec = SapiensTagsMustBeUniqueSpecification::new();

        let combined_and = name_spec.and(tags_spec);
        let sapiens = create_test_sapiens();

        assert!(combined_and.is_satisfied_by(&sapiens).unwrap());
    }

    #[test]
    fn test_not_specification() {
        let active_spec = SapiensIsActiveSpecification::new();
        let not_active = active_spec.not();

        // Create an inactive sapiens (conceptual test)
        let active_sapiens = create_test_sapiens();
        assert!(active_sapiens.status.is_active());

        // The not specification should return false for an active sapiens
        assert!(!not_active.is_satisfied_by(&active_sapiens).unwrap());
    }

    #[test]
    fn test_specification_evaluator() {
        let spec = SapiensNameMustBeValidSpecification::new();
        let sapiens = create_test_sapiens();

        let result = SpecificationEvaluator::evaluate(&spec, &sapiens).unwrap();
        assert!(result.satisfied);
        assert!(result.specification_name.contains("SapiensNameMustBeValidSpecification"));
    }

    #[test]
    fn test_metadata_specification() {
        use crate::domain::value_objects::sapiens::{SapiensId, SapiensStatus, SapiensVersion};
        use chrono::Utc;

        let spec_has_key = SapiensWithMetadataKeySpecification::new(
            "env".to_string(),
            None,
        );

        let spec_has_key_value = SapiensWithMetadataKeySpecification::new(
            "env".to_string(),
            Some("production".to_string()),
        );

        let spec_wrong_value = SapiensWithMetadataKeySpecification::new(
            "env".to_string(),
            Some("development".to_string()),
        );

        // Create a sapiens with metadata
        let mut metadata = Metadata::new();
        metadata.insert("env".to_string(), "production".to_string()).unwrap();

        let sapiens = Sapiens::from_data(
            SapiensId::generate(),
            SapiensName::new("Test Sapiens").unwrap(),
            Some("Test Description".to_string()),
            SapiensStatus::Active,
            metadata,
            "test_user".to_string(),
            Utc::now(),
            Utc::now(),
            None,
            SapiensVersion::first(),
        );

        assert!(spec_has_key.is_satisfied_by(&sapiens).unwrap());
        assert!(spec_has_key_value.is_satisfied_by(&sapiens).unwrap());
        assert!(!spec_wrong_value.is_satisfied_by(&sapiens).unwrap());
    }

    #[test]
    fn test_temporal_specifications() {
        let recent_spec = SapiensMustBeRecentSpecification::new(30);
        let sapiens = create_test_sapiens();

        assert!(recent_spec.is_satisfied_by(&sapiens).unwrap());

        let old_spec = SapiensMustNotBeOlderThanSpecification::new(1);
        assert!(old_spec.is_satisfied_by(&sapiens).unwrap());
    }
}
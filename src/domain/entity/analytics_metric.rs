use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::AnalyticsMetricType;
use super::AggregationLevel;
use super::AuditMetadata;

/// Strongly-typed ID for AnalyticsMetric
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnalyticsMetricId(pub Uuid);

impl AnalyticsMetricId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for AnalyticsMetricId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for AnalyticsMetricId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for AnalyticsMetricId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<AnalyticsMetricId> for Uuid {
    fn from(id: AnalyticsMetricId) -> Self { id.0 }
}

impl AsRef<Uuid> for AnalyticsMetricId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for AnalyticsMetricId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AnalyticsMetric {
    pub id: Uuid,
    pub metric_name: String,
    pub metric_type: AnalyticsMetricType,
    pub value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<serde_json::Value>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub aggregation_level: AggregationLevel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<Uuid>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl AnalyticsMetric {
    /// Create a builder for AnalyticsMetric
    pub fn builder() -> AnalyticsMetricBuilder {
        AnalyticsMetricBuilder::default()
    }

    /// Create a new AnalyticsMetric with required fields
    pub fn new(metric_name: String, metric_type: AnalyticsMetricType, value: f64, period_start: DateTime<Utc>, period_end: DateTime<Utc>, aggregation_level: AggregationLevel) -> Self {
        Self {
            id: Uuid::new_v4(),
            metric_name,
            metric_type,
            value,
            unit: None,
            dimensions: None,
            period_start,
            period_end,
            aggregation_level,
            organization_id: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> AnalyticsMetricId {
        AnalyticsMetricId(self.id)
    }

    /// Get when this entity was created
    pub fn created_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.created_at.as_ref()
    }

    /// Get when this entity was last updated
    pub fn updated_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.updated_at.as_ref()
    }

    /// Check if this entity is soft deleted
    pub fn is_deleted(&self) -> bool {
        self.metadata.deleted_at.is_some()
    }

    /// Check if this entity is active (not deleted)
    pub fn is_active(&self) -> bool {
        self.metadata.deleted_at.is_none()
    }

    /// Get when this entity was deleted
    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.deleted_at.as_ref()
    }

    /// Get who created this entity
    pub fn created_by(&self) -> Option<&Uuid> {
        self.metadata.created_by.as_ref()
    }

    /// Get who last updated this entity
    pub fn updated_by(&self) -> Option<&Uuid> {
        self.metadata.updated_by.as_ref()
    }

    /// Get who deleted this entity
    pub fn deleted_by(&self) -> Option<&Uuid> {
        self.metadata.deleted_by.as_ref()
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the unit field (chainable)
    pub fn with_unit(mut self, value: String) -> Self {
        self.unit = Some(value);
        self
    }

    /// Set the dimensions field (chainable)
    pub fn with_dimensions(mut self, value: serde_json::Value) -> Self {
        self.dimensions = Some(value);
        self
    }

    /// Set the organization_id field (chainable)
    pub fn with_organization_id(mut self, value: Uuid) -> Self {
        self.organization_id = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "metric_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.metric_name = v; }
                }
                "metric_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.metric_type = v; }
                }
                "value" => {
                    if let Ok(v) = serde_json::from_value(value) { self.value = v; }
                }
                "unit" => {
                    if let Ok(v) = serde_json::from_value(value) { self.unit = v; }
                }
                "dimensions" => {
                    if let Ok(v) = serde_json::from_value(value) { self.dimensions = v; }
                }
                "period_start" => {
                    if let Ok(v) = serde_json::from_value(value) { self.period_start = v; }
                }
                "period_end" => {
                    if let Ok(v) = serde_json::from_value(value) { self.period_end = v; }
                }
                "aggregation_level" => {
                    if let Ok(v) = serde_json::from_value(value) { self.aggregation_level = v; }
                }
                "organization_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.organization_id = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for AnalyticsMetric {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "AnalyticsMetric"
    }
}

impl backbone_core::PersistentEntity for AnalyticsMetric {
    fn entity_id(&self) -> String {
        self.id.to_string()
    }
    fn set_entity_id(&mut self, id: String) {
        if let Ok(uuid) = uuid::Uuid::parse_str(&id) {
            self.id = uuid;
        }
    }
    fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.created_at
    }
    fn set_created_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.created_at = Some(ts);
    }
    fn updated_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.updated_at
    }
    fn set_updated_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.updated_at = Some(ts);
    }
    fn deleted_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.deleted_at
    }
    fn set_deleted_at(&mut self, ts: Option<chrono::DateTime<chrono::Utc>>) {
        self.metadata.deleted_at = ts;
    }
}

impl backbone_orm::EntityRepoMeta for AnalyticsMetric {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("organization_id".to_string(), "uuid".to_string());
        m.insert("metric_type".to_string(), "analytics_metric_type".to_string());
        m.insert("aggregation_level".to_string(), "aggregation_level".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["metric_name"]
    }
}

/// Builder for AnalyticsMetric entity
///
/// Provides a fluent API for constructing AnalyticsMetric instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct AnalyticsMetricBuilder {
    metric_name: Option<String>,
    metric_type: Option<AnalyticsMetricType>,
    value: Option<f64>,
    unit: Option<String>,
    dimensions: Option<serde_json::Value>,
    period_start: Option<DateTime<Utc>>,
    period_end: Option<DateTime<Utc>>,
    aggregation_level: Option<AggregationLevel>,
    organization_id: Option<Uuid>,
}

impl AnalyticsMetricBuilder {
    /// Set the metric_name field (required)
    pub fn metric_name(mut self, value: String) -> Self {
        self.metric_name = Some(value);
        self
    }

    /// Set the metric_type field (required)
    pub fn metric_type(mut self, value: AnalyticsMetricType) -> Self {
        self.metric_type = Some(value);
        self
    }

    /// Set the value field (required)
    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Set the unit field (optional)
    pub fn unit(mut self, value: String) -> Self {
        self.unit = Some(value);
        self
    }

    /// Set the dimensions field (optional)
    pub fn dimensions(mut self, value: serde_json::Value) -> Self {
        self.dimensions = Some(value);
        self
    }

    /// Set the period_start field (required)
    pub fn period_start(mut self, value: DateTime<Utc>) -> Self {
        self.period_start = Some(value);
        self
    }

    /// Set the period_end field (required)
    pub fn period_end(mut self, value: DateTime<Utc>) -> Self {
        self.period_end = Some(value);
        self
    }

    /// Set the aggregation_level field (required)
    pub fn aggregation_level(mut self, value: AggregationLevel) -> Self {
        self.aggregation_level = Some(value);
        self
    }

    /// Set the organization_id field (optional)
    pub fn organization_id(mut self, value: Uuid) -> Self {
        self.organization_id = Some(value);
        self
    }

    /// Build the AnalyticsMetric entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<AnalyticsMetric, String> {
        let metric_name = self.metric_name.ok_or_else(|| "metric_name is required".to_string())?;
        let metric_type = self.metric_type.ok_or_else(|| "metric_type is required".to_string())?;
        let value = self.value.ok_or_else(|| "value is required".to_string())?;
        let period_start = self.period_start.ok_or_else(|| "period_start is required".to_string())?;
        let period_end = self.period_end.ok_or_else(|| "period_end is required".to_string())?;
        let aggregation_level = self.aggregation_level.ok_or_else(|| "aggregation_level is required".to_string())?;

        Ok(AnalyticsMetric {
            id: Uuid::new_v4(),
            metric_name,
            metric_type,
            value,
            unit: self.unit,
            dimensions: self.dimensions,
            period_start,
            period_end,
            aggregation_level,
            organization_id: self.organization_id,
            metadata: AuditMetadata::default(),
        })
    }
}

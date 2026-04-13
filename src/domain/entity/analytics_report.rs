use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::AnalyticsReportType;
use super::ReportFormat;
use super::ReportStatus;
use super::AuditMetadata;

/// Strongly-typed ID for AnalyticsReport
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnalyticsReportId(pub Uuid);

impl AnalyticsReportId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for AnalyticsReportId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for AnalyticsReportId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for AnalyticsReportId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<AnalyticsReportId> for Uuid {
    fn from(id: AnalyticsReportId) -> Self { id.0 }
}

impl AsRef<Uuid> for AnalyticsReportId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for AnalyticsReportId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AnalyticsReport {
    pub id: Uuid,
    pub report_name: String,
    pub report_type: AnalyticsReportType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    pub format: ReportFormat,
    pub generated_by: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i32>,
    pub status: ReportStatus,
    pub download_count: i32,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl AnalyticsReport {
    /// Create a builder for AnalyticsReport
    pub fn builder() -> AnalyticsReportBuilder {
        AnalyticsReportBuilder::default()
    }

    /// Create a new AnalyticsReport with required fields
    pub fn new(report_name: String, report_type: AnalyticsReportType, format: ReportFormat, generated_by: Uuid, status: ReportStatus, download_count: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            report_name,
            report_type,
            description: None,
            parameters: None,
            format,
            generated_by,
            generated_at: None,
            expires_at: None,
            file_path: None,
            file_size: None,
            status,
            download_count,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> AnalyticsReportId {
        AnalyticsReportId(self.id)
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

    /// Get the current status
    pub fn status(&self) -> &ReportStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the description field (chainable)
    pub fn with_description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the parameters field (chainable)
    pub fn with_parameters(mut self, value: serde_json::Value) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Set the generated_at field (chainable)
    pub fn with_generated_at(mut self, value: DateTime<Utc>) -> Self {
        self.generated_at = Some(value);
        self
    }

    /// Set the expires_at field (chainable)
    pub fn with_expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the file_path field (chainable)
    pub fn with_file_path(mut self, value: String) -> Self {
        self.file_path = Some(value);
        self
    }

    /// Set the file_size field (chainable)
    pub fn with_file_size(mut self, value: i32) -> Self {
        self.file_size = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "report_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.report_name = v; }
                }
                "report_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.report_type = v; }
                }
                "description" => {
                    if let Ok(v) = serde_json::from_value(value) { self.description = v; }
                }
                "parameters" => {
                    if let Ok(v) = serde_json::from_value(value) { self.parameters = v; }
                }
                "format" => {
                    if let Ok(v) = serde_json::from_value(value) { self.format = v; }
                }
                "generated_by" => {
                    if let Ok(v) = serde_json::from_value(value) { self.generated_by = v; }
                }
                "generated_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.generated_at = v; }
                }
                "expires_at" => {
                    if let Ok(v) = serde_json::from_value(value) { self.expires_at = v; }
                }
                "file_path" => {
                    if let Ok(v) = serde_json::from_value(value) { self.file_path = v; }
                }
                "file_size" => {
                    if let Ok(v) = serde_json::from_value(value) { self.file_size = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                "download_count" => {
                    if let Ok(v) = serde_json::from_value(value) { self.download_count = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for AnalyticsReport {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "AnalyticsReport"
    }
}

impl backbone_core::PersistentEntity for AnalyticsReport {
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

impl backbone_orm::EntityRepoMeta for AnalyticsReport {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("report_type".to_string(), "analytics_report_type".to_string());
        m.insert("format".to_string(), "report_format".to_string());
        m.insert("status".to_string(), "report_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["report_name"]
    }
}

/// Builder for AnalyticsReport entity
///
/// Provides a fluent API for constructing AnalyticsReport instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct AnalyticsReportBuilder {
    report_name: Option<String>,
    report_type: Option<AnalyticsReportType>,
    description: Option<String>,
    parameters: Option<serde_json::Value>,
    format: Option<ReportFormat>,
    generated_by: Option<Uuid>,
    generated_at: Option<DateTime<Utc>>,
    expires_at: Option<DateTime<Utc>>,
    file_path: Option<String>,
    file_size: Option<i32>,
    status: Option<ReportStatus>,
    download_count: Option<i32>,
}

impl AnalyticsReportBuilder {
    /// Set the report_name field (required)
    pub fn report_name(mut self, value: String) -> Self {
        self.report_name = Some(value);
        self
    }

    /// Set the report_type field (required)
    pub fn report_type(mut self, value: AnalyticsReportType) -> Self {
        self.report_type = Some(value);
        self
    }

    /// Set the description field (optional)
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    /// Set the parameters field (optional)
    pub fn parameters(mut self, value: serde_json::Value) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Set the format field (required)
    pub fn format(mut self, value: ReportFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Set the generated_by field (required)
    pub fn generated_by(mut self, value: Uuid) -> Self {
        self.generated_by = Some(value);
        self
    }

    /// Set the generated_at field (optional)
    pub fn generated_at(mut self, value: DateTime<Utc>) -> Self {
        self.generated_at = Some(value);
        self
    }

    /// Set the expires_at field (optional)
    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Set the file_path field (optional)
    pub fn file_path(mut self, value: String) -> Self {
        self.file_path = Some(value);
        self
    }

    /// Set the file_size field (optional)
    pub fn file_size(mut self, value: i32) -> Self {
        self.file_size = Some(value);
        self
    }

    /// Set the status field (default: `ReportStatus::default()`)
    pub fn status(mut self, value: ReportStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the download_count field (default: `0`)
    pub fn download_count(mut self, value: i32) -> Self {
        self.download_count = Some(value);
        self
    }

    /// Build the AnalyticsReport entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<AnalyticsReport, String> {
        let report_name = self.report_name.ok_or_else(|| "report_name is required".to_string())?;
        let report_type = self.report_type.ok_or_else(|| "report_type is required".to_string())?;
        let format = self.format.ok_or_else(|| "format is required".to_string())?;
        let generated_by = self.generated_by.ok_or_else(|| "generated_by is required".to_string())?;

        Ok(AnalyticsReport {
            id: Uuid::new_v4(),
            report_name,
            report_type,
            description: self.description,
            parameters: self.parameters,
            format,
            generated_by,
            generated_at: self.generated_at,
            expires_at: self.expires_at,
            file_path: self.file_path,
            file_size: self.file_size,
            status: self.status.unwrap_or(ReportStatus::default()),
            download_count: self.download_count.unwrap_or(0),
            metadata: AuditMetadata::default(),
        })
    }
}

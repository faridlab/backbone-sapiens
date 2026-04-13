//! Test Configuration Registry
//!
//! Central registry mapping test paths to implementations for dynamic test discovery.
//! Supports all 13 Sapiens entities with 11 CRUD endpoints each (143 total endpoints).

use std::collections::HashMap;

use crate::integration::framework::Test;
use crate::integration::tests::{
    UserApiTest, RoleApiTest, PermissionApiTest, SessionApiTest,
    UserRoleApiTest, UserPermissionApiTest, RolePermissionApiTest,
    AuditLogApiTest, MfaDeviceApiTest, PasswordResetTokenApiTest,
    UserSettingsApiTest, SystemSettingsApiTest,
    AnalyticsEventApiTest, OAuthProviderApiTest, NotificationApiTest, WorkflowApiTest,
    V2ApiTest,
};

/// Test type categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestType {
    /// API endpoint tests
    Api,
    /// Admin API endpoint tests
    AdminApi,
    /// Integration tests with external services
    Integration,
    /// End-to-end tests
    E2e,
    /// RBAC/Authorization tests
    Rbac,
    /// Security-related tests
    Security,
    /// Settings/Configuration tests
    Settings,
    /// Analytics and Operations tests
    Analytics,
}

impl std::fmt::Display for TestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestType::Api => write!(f, "api"),
            TestType::AdminApi => write!(f, "admin_api"),
            TestType::Integration => write!(f, "integration"),
            TestType::E2e => write!(f, "e2e"),
            TestType::Rbac => write!(f, "rbac"),
            TestType::Security => write!(f, "security"),
            TestType::Settings => write!(f, "settings"),
            TestType::Analytics => write!(f, "analytics"),
        }
    }
}

/// Configuration for a specific test
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// Module name
    pub module: String,

    /// Test type
    pub test_type: TestType,

    /// Subdirectory for results
    pub results_subdir: String,

    /// Description
    pub description: String,

    /// Entity name being tested
    pub entity: String,

    /// Number of expected endpoints (default: 11 for Backbone CRUD)
    pub expected_endpoints: usize,
}

impl TestConfig {
    pub fn new_api(entity: &str, module: &str, description: &str) -> Self {
        Self {
            module: module.to_string(),
            test_type: TestType::Api,
            results_subdir: format!("{}/api", entity.to_lowercase()),
            description: description.to_string(),
            entity: entity.to_string(),
            expected_endpoints: 11, // Backbone CRUD standard
        }
    }

    pub fn with_type(mut self, test_type: TestType) -> Self {
        self.test_type = test_type;
        self
    }

    pub fn with_expected_endpoints(mut self, count: usize) -> Self {
        self.expected_endpoints = count;
        self
    }
}

/// Factory function type for creating tests
pub type TestFactory = fn() -> Box<dyn Test>;

/// Test registration entry
pub struct TestRegistration {
    /// Test configuration
    pub config: TestConfig,

    /// Factory function to create test instance
    pub factory: TestFactory,
}

/// Central test registry
pub struct TestRegistry {
    /// Registered tests by path
    tests: HashMap<String, TestRegistration>,
}

impl TestRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            tests: HashMap::new(),
        }
    }

    /// Create registry with default tests for all 13 Sapiens entities
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        // ============================================================
        // Core Entities
        // ============================================================

        // User API tests
        registry.register(
            "sapiens.api.users",
            TestRegistration {
                config: TestConfig::new_api("User", "user_api_test", "User CRUD API tests"),
                factory: || Box::new(UserApiTest::new()),
            },
        );

        // Role API tests
        registry.register(
            "sapiens.api.roles",
            TestRegistration {
                config: TestConfig::new_api("Role", "role_api_test", "Role CRUD API tests"),
                factory: || Box::new(RoleApiTest::new()),
            },
        );

        // Permission API tests
        registry.register(
            "sapiens.api.permissions",
            TestRegistration {
                config: TestConfig::new_api("Permission", "permission_api_test", "Permission CRUD API tests"),
                factory: || Box::new(PermissionApiTest::new()),
            },
        );

        // ============================================================
        // Session Management
        // ============================================================

        // Session API tests
        registry.register(
            "sapiens.api.sessions",
            TestRegistration {
                config: TestConfig::new_api("Session", "session_api_test", "Session CRUD API tests"),
                factory: || Box::new(SessionApiTest::new()),
            },
        );

        // ============================================================
        // RBAC Junction Tables
        // ============================================================

        // UserRole API tests
        registry.register(
            "sapiens.api.user_roles",
            TestRegistration {
                config: TestConfig::new_api("UserRole", "user_role_api_test", "User-Role assignment CRUD API tests")
                    .with_type(TestType::Rbac)
                    .with_expected_endpoints(8), // Junction tables often have fewer endpoints
                factory: || Box::new(UserRoleApiTest::new()),
            },
        );

        // UserPermission API tests
        registry.register(
            "sapiens.api.user_permissions",
            TestRegistration {
                config: TestConfig::new_api("UserPermission", "user_permission_api_test", "Direct User-Permission grant CRUD API tests")
                    .with_type(TestType::Rbac),
                factory: || Box::new(UserPermissionApiTest::new()),
            },
        );

        // RolePermission API tests
        registry.register(
            "sapiens.api.role_permissions",
            TestRegistration {
                config: TestConfig::new_api("RolePermission", "role_permission_api_test", "Role-Permission mapping CRUD API tests")
                    .with_type(TestType::Rbac)
                    .with_expected_endpoints(8),
                factory: || Box::new(RolePermissionApiTest::new()),
            },
        );

        // ============================================================
        // Security & Audit
        // ============================================================

        // AuditLog API tests
        registry.register(
            "sapiens.api.audit_logs",
            TestRegistration {
                config: TestConfig::new_api("AuditLog", "audit_log_api_test", "Audit log CRUD API tests")
                    .with_type(TestType::Security),
                factory: || Box::new(AuditLogApiTest::new()),
            },
        );

        // MfaDevice API tests
        registry.register(
            "sapiens.api.mfa_devices",
            TestRegistration {
                config: TestConfig::new_api("MfaDevice", "mfa_device_api_test", "MFA device CRUD API tests")
                    .with_type(TestType::Security),
                factory: || Box::new(MfaDeviceApiTest::new()),
            },
        );

        // PasswordResetToken API tests
        registry.register(
            "sapiens.api.password_reset_tokens",
            TestRegistration {
                config: TestConfig::new_api("PasswordResetToken", "password_reset_token_api_test", "Password reset token CRUD API tests")
                    .with_type(TestType::Security)
                    .with_expected_endpoints(8), // Security tokens often have limited operations
                factory: || Box::new(PasswordResetTokenApiTest::new()),
            },
        );

        // ============================================================
        // Settings
        // ============================================================

        // UserSettings API tests
        registry.register(
            "sapiens.api.user_settings",
            TestRegistration {
                config: TestConfig::new_api("UserSettings", "user_settings_api_test", "User settings/preferences CRUD API tests")
                    .with_type(TestType::Settings),
                factory: || Box::new(UserSettingsApiTest::new()),
            },
        );

        // SystemSettings API tests
        registry.register(
            "sapiens.api.system_settings",
            TestRegistration {
                config: TestConfig::new_api("SystemSettings", "system_settings_api_test", "System configuration CRUD API tests")
                    .with_type(TestType::Settings),
                factory: || Box::new(SystemSettingsApiTest::new()),
            },
        );

        // ============================================================
        // Operations & Analytics
        // ============================================================

        // AnalyticsEvent API tests
        registry.register(
            "sapiens.api.analytics_events",
            TestRegistration {
                config: TestConfig::new_api("AnalyticsEvent", "analytics_event_api_test", "Analytics event CRUD API tests")
                    .with_type(TestType::Analytics)
                    .with_expected_endpoints(15), // Analytics has additional endpoints
                factory: || Box::new(AnalyticsEventApiTest::new()),
            },
        );

        // OAuthProvider API tests
        registry.register(
            "sapiens.api.oauth_providers",
            TestRegistration {
                config: TestConfig::new_api("OAuthProvider", "oauth_provider_api_test", "OAuth provider CRUD API tests")
                    .with_type(TestType::Integration),
                factory: || Box::new(OAuthProviderApiTest::new()),
            },
        );

        // Notification API tests
        registry.register(
            "sapiens.api.notifications",
            TestRegistration {
                config: TestConfig::new_api("Notification", "notification_api_test", "Notification CRUD API tests")
                    .with_type(TestType::Integration)
                    .with_expected_endpoints(15), // Notifications have additional endpoints
                factory: || Box::new(NotificationApiTest::new()),
            },
        );

        // Workflow API tests
        registry.register(
            "sapiens.api.workflows",
            TestRegistration {
                config: TestConfig::new_api("Workflow", "workflow_api_test", "Workflow CRUD API tests")
                    .with_type(TestType::Integration)
                    .with_expected_endpoints(15), // Workflows have additional endpoints
                factory: || Box::new(WorkflowApiTest::new()),
            },
        );

        // ============================================================
        // V2.0 Features
        // ============================================================

        // V2.0 Features API tests
        registry.register(
            "sapiens.api.v2_features",
            TestRegistration {
                config: TestConfig::new_api("V2Features", "v2_api_test", "V2.0 feature API tests")
                    .with_type(TestType::Security)
                    .with_expected_endpoints(20), // 6 entities with additional endpoints
                factory: || Box::new(V2ApiTest::new()),
            },
        );

        registry
    }

    /// Register a test
    pub fn register(&mut self, path: &str, registration: TestRegistration) {
        self.tests.insert(path.to_string(), registration);
    }

    /// Get a test by path
    pub fn get(&self, path: &str) -> Option<&TestRegistration> {
        self.tests.get(path)
    }

    /// Create a test instance by path
    pub fn create(&self, path: &str) -> Option<Box<dyn Test>> {
        self.tests.get(path).map(|reg| (reg.factory)())
    }

    /// List all registered test paths
    pub fn list_paths(&self) -> Vec<&String> {
        let mut paths: Vec<_> = self.tests.keys().collect();
        paths.sort(); // Sort for consistent ordering
        paths
    }

    /// List tests by type
    pub fn list_by_type(&self, test_type: TestType) -> Vec<(&String, &TestRegistration)> {
        let mut tests: Vec<_> = self.tests
            .iter()
            .filter(|(_, reg)| reg.config.test_type == test_type)
            .collect();
        tests.sort_by(|a, b| a.0.cmp(b.0));
        tests
    }

    /// Get all test configurations
    pub fn all_configs(&self) -> Vec<(&String, &TestConfig)> {
        let mut configs: Vec<_> = self.tests.iter().map(|(k, v)| (k, &v.config)).collect();
        configs.sort_by(|a, b| a.0.cmp(b.0));
        configs
    }

    /// Get total expected endpoint count
    pub fn total_expected_endpoints(&self) -> usize {
        self.tests.values().map(|r| r.config.expected_endpoints).sum()
    }

    /// Get entity count
    pub fn entity_count(&self) -> usize {
        self.tests.len()
    }
}

impl Default for TestRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}

// ============================================================
// Test Runner
// ============================================================

/// Runs integration tests using the registry
pub struct TestRunner {
    /// Test registry
    registry: TestRegistry,

    /// API base URL override
    api_base_url: Option<String>,
}

impl TestRunner {
    /// Create a new test runner with default registry
    pub fn new() -> Self {
        Self {
            registry: TestRegistry::with_defaults(),
            api_base_url: None,
        }
    }

    /// Create with custom registry
    pub fn with_registry(registry: TestRegistry) -> Self {
        Self {
            registry,
            api_base_url: None,
        }
    }

    /// Set API base URL
    pub fn with_api_url(mut self, url: &str) -> Self {
        self.api_base_url = Some(url.to_string());
        self
    }

    /// Run a specific test by path
    pub async fn run(&self, path: &str) -> Option<crate::integration::framework::TestSuiteResult> {
        let mut test = self.registry.create(path)?;
        Some(test.execute().await)
    }

    /// Run all registered tests
    pub async fn run_all(&self) -> Vec<(String, crate::integration::framework::TestSuiteResult)> {
        let mut results = Vec::new();

        for path in self.registry.list_paths() {
            if let Some(reg) = self.registry.get(path) {
                if let Some(mut test) = self.registry.create(path) {
                    println!("Running: {} - {}", path, reg.config.description);
                    let result = test.execute().await;
                    results.push((path.clone(), result));
                }
            }
        }

        results
    }

    /// Run all tests of a specific type
    pub async fn run_by_type(&self, test_type: TestType) -> Vec<(String, crate::integration::framework::TestSuiteResult)> {
        let mut results = Vec::new();

        for (path, reg) in self.registry.list_by_type(test_type) {
            if let Some(mut test) = self.registry.create(path) {
                println!("Running [{}]: {} - {}", reg.config.test_type, path, reg.config.description);
                let result = test.execute().await;
                results.push((path.clone(), result));
            }
        }

        results
    }

    /// Print test summary
    pub fn print_summary(&self, results: &[(String, crate::integration::framework::TestSuiteResult)]) {
        println!("\n");
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║                  INTEGRATION TEST SUMMARY                         ║");
        println!("║   Entities: {} | Expected Endpoints: {}                          ║",
            self.registry.entity_count(),
            self.registry.total_expected_endpoints()
        );
        println!("╠══════════════════════════════════════════════════════════════════╣");

        let mut total_passed = 0;
        let mut total_failed = 0;
        let mut total_duration = 0.0;

        for (path, suite) in results {
            let status = if suite.all_passed() { "✓" } else { "✗" };
            let passed = suite.passed_count();
            let failed = suite.failed_count();

            println!(
                "║ {} {:40} {:3} passed {:3} failed",
                status, path, passed, failed
            );

            total_passed += passed;
            total_failed += failed;
            total_duration += suite.total_duration();
        }

        println!("╠══════════════════════════════════════════════════════════════════╣");

        let status_emoji = if total_failed == 0 { "✅" } else { "❌" };
        println!(
            "║ {} TOTAL: {} passed, {} failed in {:.2}s",
            status_emoji, total_passed, total_failed, total_duration
        );
        println!("╚══════════════════════════════════════════════════════════════════╝");
        println!();
    }

    /// Print registered tests info
    pub fn print_registry_info(&self) {
        println!("\n╔══════════════════════════════════════════════════════════════════╗");
        println!("║                    REGISTERED TEST SUITES                         ║");
        println!("╠══════════════════════════════════════════════════════════════════╣");

        for (path, config) in self.registry.all_configs() {
            println!(
                "║ {:40} [{:10}] {} endpoints",
                path, config.test_type, config.expected_endpoints
            );
        }

        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!(
            "║ Total: {} test suites, {} expected endpoints",
            self.registry.entity_count(),
            self.registry.total_expected_endpoints()
        );
        println!("╚══════════════════════════════════════════════════════════════════╝\n");
    }
}

impl Default for TestRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_defaults() {
        let registry = TestRegistry::with_defaults();

        // Core entities
        assert!(registry.get("sapiens.api.users").is_some());
        assert!(registry.get("sapiens.api.roles").is_some());
        assert!(registry.get("sapiens.api.permissions").is_some());

        // Session
        assert!(registry.get("sapiens.api.sessions").is_some());

        // RBAC
        assert!(registry.get("sapiens.api.user_roles").is_some());
        assert!(registry.get("sapiens.api.user_permissions").is_some());
        assert!(registry.get("sapiens.api.role_permissions").is_some());

        // Security
        assert!(registry.get("sapiens.api.audit_logs").is_some());
        assert!(registry.get("sapiens.api.mfa_devices").is_some());
        assert!(registry.get("sapiens.api.password_reset_tokens").is_some());

        // Settings
        assert!(registry.get("sapiens.api.user_settings").is_some());
        assert!(registry.get("sapiens.api.system_settings").is_some());

        // Operations & Analytics
        assert!(registry.get("sapiens.api.analytics_events").is_some());
        assert!(registry.get("sapiens.api.oauth_providers").is_some());
        assert!(registry.get("sapiens.api.notifications").is_some());
        assert!(registry.get("sapiens.api.workflows").is_some());
    }

    #[test]
    fn test_entity_count() {
        let registry = TestRegistry::with_defaults();
        assert_eq!(registry.entity_count(), 17); // 17 test suites (all entities enabled including V2)
    }

    #[test]
    fn test_list_paths_sorted() {
        let registry = TestRegistry::with_defaults();
        let paths = registry.list_paths();

        // Should be sorted alphabetically
        let paths_vec: Vec<_> = paths.iter().map(|p| p.as_str()).collect();
        let mut sorted = paths_vec.clone();
        sorted.sort();
        assert_eq!(paths_vec, sorted);
    }

    #[test]
    fn test_list_by_type() {
        let registry = TestRegistry::with_defaults();

        let api_tests = registry.list_by_type(TestType::Api);
        assert!(!api_tests.is_empty());

        let rbac_tests = registry.list_by_type(TestType::Rbac);
        assert_eq!(rbac_tests.len(), 3); // UserRole, UserPermission, RolePermission

        let security_tests = registry.list_by_type(TestType::Security);
        assert_eq!(security_tests.len(), 4); // AuditLog, MfaDevice, PasswordResetToken, V2Features

        let settings_tests = registry.list_by_type(TestType::Settings);
        assert_eq!(settings_tests.len(), 2); // UserSettings, SystemSettings
    }

    #[test]
    fn test_create_test_instance() {
        let registry = TestRegistry::with_defaults();

        let user_test = registry.create("sapiens.api.users");
        assert!(user_test.is_some());

        let permission_test = registry.create("sapiens.api.permissions");
        assert!(permission_test.is_some());

        let nonexistent = registry.create("sapiens.api.nonexistent");
        assert!(nonexistent.is_none());
    }

    #[test]
    fn test_config_builder() {
        let config = TestConfig::new_api("User", "user_api_test", "User tests")
            .with_type(TestType::Security)
            .with_expected_endpoints(5);

        assert_eq!(config.entity, "User");
        assert_eq!(config.test_type, TestType::Security);
        assert_eq!(config.expected_endpoints, 5);
    }
}

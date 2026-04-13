//! Users seeder
//!
//! Seeds comprehensive initial user data into the database.
//! Includes test users, demo data, and administrative accounts.

use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

use super::Seeder;
use crate::domain::entity::{User, UserStatus};

/// Seeder for users
pub struct SeedUsersSeeder;

impl SeedUsersSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SeedUsersSeeder {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Seeder for SeedUsersSeeder {
    fn name(&self) -> &'static str {
        "SeedUsersSeeder"
    }

    fn order(&self) -> i32 {
        3
    }

    async fn should_run(&self, pool: &PgPool) -> Result<bool> {
        // Check if users table has any data
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(pool)
            .await?;
        Ok(count.0 == 0)
    }

    async fn run(&self, pool: &PgPool) -> Result<()> {
        println!("🌱️  Seeding users...");

        // Create comprehensive user seed data
        let users = self.generate_seed_users();

        let mut created_count = 0;
        let mut failed_count = 0;

        for user in users {
            match self.create_user(pool, &user).await {
                Ok(_) => {
                    created_count += 1;
                    if created_count % 10 == 0 {
                        println!("   ✓ Created {} users", created_count);
                    }
                }
                Err(e) => {
                    failed_count += 1;
                    println!("   ❌ Failed to create user {}: {}", user.username, e);
                }
            }
        }

        println!("   ✅ User seeding complete: {} created, {} failed", created_count, failed_count);

        // Run SQL seed files for any additional setup
        if let Ok(sql) = std::fs::read_to_string("migrations/seeds/003_seed_users.sql") {
            println!("   📄 Running additional user SQL setup...");
            sqlx::raw_sql(&sql).execute(pool).await?;
        }

        if let Ok(user_roles_sql) = std::fs::read_to_string("migrations/seeds/004_seed_user_roles.sql") {
            println!("   📄 Running user roles SQL setup...");
            sqlx::raw_sql(&user_roles_sql).execute(pool).await?;
        }

        Ok(())
    }

    async fn rollback(&self, pool: &PgPool) -> Result<()> {
        // Delete in reverse order due to foreign keys
        sqlx::query("DELETE FROM user_roles")
            .execute(pool)
            .await?;
        sqlx::query("DELETE FROM users")
            .execute(pool)
            .await?;
        Ok(())
    }
}

// Helper functions for SeedUsersSeeder (not part of the trait)
impl SeedUsersSeeder {
    /// Create a user in the database
    async fn create_user(&self, pool: &PgPool, user: &User) -> Result<()> {
        let query = r#"
            INSERT INTO users (
                id, username, email, password_hash, status, email_verified,
                failed_login_attempts, locked_until, last_login, metadata
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
            )
            ON CONFLICT (id) DO NOTHING
        "#;

        sqlx::query(query)
            .bind(user.id)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(&user.status)
            .bind(user.email_verified)
            .bind(user.failed_login_attempts)
            .bind(user.locked_until)
            .bind(user.last_login)
            .bind(&user.metadata)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Generate comprehensive seed user data
    fn generate_seed_users(&self) -> Vec<User> {
        let mut users = Vec::new();

        // 1. Administrative users
        users.push(User {
            id: Uuid::parse_str("3ab63f4a-71d7-5138-8a65-5c8a6fe4a96c").unwrap(),
            username: "root".to_string(),
            email: "root@startapp.id".to_string(),
            password_hash: self.hash_password("PiQS5SVL012D"),
            status: UserStatus::Active,
            email_verified: true,
            failed_login_attempts: 0,
            locked_until: None,
            last_login: None,
            metadata: serde_json::json!({
                "is_admin": true,
                "is_system": true,
                "roles": ["admin"],
                "created_by": "system",
                "created_at": chrono::Utc::now(),
                "updated_at": chrono::Utc::now()
            }),
        });

        users.push(User {
            id: Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
            username: "admin".to_string(),
            email: "admin@startapp.id".to_string(),
            password_hash: self.hash_password("admin123"),
            status: UserStatus::Active,
            email_verified: true,
            failed_login_attempts: 0,
            locked_until: None,
            last_login: None,
            metadata: serde_json::json!({
                "is_admin": true,
                "is_system": false,
                "roles": ["admin", "moderator"],
                "created_by": "system",
                "created_at": chrono::Utc::now(),
                "updated_at": chrono::Utc::now()
            }),
        });

        // 2. Test and development users
        let test_users = vec![
            ("developer1", "dev1@test.local", "dev123"),
            ("developer2", "dev2@test.local", "dev123"),
            ("developer3", "dev3@test.local", "dev123"),
            ("tester1", "test1@test.local", "test123"),
            ("tester2", "test2@test.local", "test123"),
            ("tester3", "test3@test.local", "test123"),
        ];

        for (i, (username, email, password)) in test_users.iter().enumerate() {
            users.push(User {
                id: Uuid::new_v4(),
                username: username.to_string(),
                email: email.to_string(),
                password_hash: self.hash_password(password),
                status: UserStatus::Active,
                email_verified: true,
                failed_login_attempts: 0,
                locked_until: None,
                last_login: None,
                metadata: serde_json::json!({
                    "is_admin": false,
                    "is_test": true,
                    "roles": ["developer"],
                    "created_by": "system",
                    "test_data": true,
                    "created_at": chrono::Utc::now(),
                    "updated_at": chrono::Utc::now()
                }),
            });
        }

        // 3. Demo users with different statuses
        let demo_users = vec![
            ("john_doe", "john.doe@example.com", UserStatus::Active, true),
            ("jane_smith", "jane.smith@example.com", UserStatus::Active, true),
            ("bob_johnson", "bob.johnson@example.com", UserStatus::Active, true),
            ("alice_brown", "alice.brown@example.com", UserStatus::PendingVerification, false),
            ("charlie_davis", "charlie.davis@example.com", UserStatus::Inactive, true),
            ("eve_wilson", "eve.wilson@example.com", UserStatus::Suspended, true),
        ];

        for (username, email, status, email_verified) in demo_users {
            users.push(User {
                id: Uuid::new_v4(),
                username: username.to_string(),
                email: email.to_string(),
                password_hash: self.hash_password("password123"),
                status,
                email_verified,
                failed_login_attempts: if status == UserStatus::Suspended { 3 } else { 0 },
                locked_until: if status == UserStatus::Suspended {
                    Some(Utc::now() + chrono::Duration::days(1))
                } else {
                    None
                },
                last_login: None,
                metadata: serde_json::json!({
                    "is_demo": true,
                    "roles": ["user"],
                    "created_by": "system",
                    "sample_user": true
                }),
            });
        }

        // 4. Additional demo users for testing
        for i in 1..=20 {
            let username = format!("demo_user_{:03}", i);
            let email = format!("demo.user.{:03}@demo.local", i);

            users.push(User {
                id: Uuid::new_v4(),
                username,
                email,
                password_hash: self.hash_password("demo123"),
                status: UserStatus::Active,
                email_verified: if i <= 10 { true } else { false },
                failed_login_attempts: 0,
                locked_until: None,
                last_login: None,
                metadata: serde_json::json!({
                    "is_demo": true,
                    "roles": ["user"],
                    "created_by": "system",
                    "demo_index": i
                }),
            });
        }

        users
    }

    /// Hash a password using the same method as the application
    fn hash_password(&self, password: &str) -> String {
        // This would use the same password hashing method as the main application
        // For now, return a placeholder hash (in real implementation, use Argon2)
        format!("hashed_{}", password) // Placeholder - implement proper hashing
    }
}
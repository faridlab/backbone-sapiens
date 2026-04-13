//! Roles seeder
//!
//! Seeds initial role data into the database.

use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use super::Seeder;

/// Seeder for roles
pub struct SeedRolesSeeder;

impl SeedRolesSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SeedRolesSeeder {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Seeder for SeedRolesSeeder {
    fn name(&self) -> &'static str {
        "SeedRolesSeeder"
    }

    fn order(&self) -> i32 {
        2
    }

    async fn should_run(&self, pool: &PgPool) -> Result<bool> {
        // Check if roles table has any data
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM roles")
            .fetch_one(pool)
            .await?;
        Ok(count.0 == 0)
    }

    async fn run(&self, pool: &PgPool) -> Result<()> {
        // Run the SQL seed file
        let sql = include_str!("../../migrations/seeds/001_seed_roles.sql");
        sqlx::raw_sql(sql).execute(pool).await?;
        Ok(())
    }

    async fn rollback(&self, pool: &PgPool) -> Result<()> {
        sqlx::query("DELETE FROM roles")
            .execute(pool)
            .await?;
        Ok(())
    }
}

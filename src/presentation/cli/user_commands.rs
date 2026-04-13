//! User CLI Commands
//!
//! Command-line interface for User CRUD operations.
//! Provides administrative tools for managing users from the terminal.

use std::sync::Arc;
use anyhow::Result;

use crate::application::services::{
    UserApplicationService,
    CreateUserCommand,
    UpdateUserCommand,
    ListUsersQuery,
    UserFilters,
};
use crate::domain::entity::UserAggregate;

// ============================================================
// CLI Command Definitions
// ============================================================

/// User management commands
#[derive(Debug)]
pub struct UserCommands {
    pub action: UserAction,
}

/// Available user actions
#[derive(Debug)]
pub enum UserAction {
    /// Create a new user
    Create {
        username: String,
        email: String,
        password: String,
        first_name: String,
        last_name: String,
    },
    /// Get a user by ID
    Get { id: String },
    /// List users with optional filtering
    List {
        page: i32,
        limit: i32,
        search: Option<String>,
        status: Option<String>,
    },
    /// Update an existing user
    Update {
        id: String,
        first_name: Option<String>,
        last_name: Option<String>,
        phone_number: Option<String>,
    },
    /// Delete a user (soft delete)
    Delete { id: String },
    /// List deleted users (trash)
    ListTrash { page: i32, limit: i32 },
    /// Restore a deleted user
    Restore { id: String },
    /// Verify user email
    VerifyEmail { id: String },
    /// Suspend user account
    Suspend { id: String, reason: String },
    /// Activate user account
    Activate { id: String },
}

// ============================================================
// CLI Handler
// ============================================================

/// Handler for user CLI commands
pub struct UserCliHandler {
    user_service: Arc<UserApplicationService>,
}

impl UserCliHandler {
    /// Create a new handler with the user application service
    pub fn new(user_service: Arc<UserApplicationService>) -> Self {
        Self { user_service }
    }

    /// Handle a user command
    pub async fn handle(&self, command: UserCommands) -> Result<()> {
        match command.action {
            UserAction::Create { username, email, password, first_name, last_name } => {
                self.create_user(username, email, password, first_name, last_name).await
            }
            UserAction::Get { id } => {
                self.get_user(&id).await
            }
            UserAction::List { page, limit, search, status } => {
                self.list_users(page, limit, search, status).await
            }
            UserAction::Update { id, first_name, last_name, phone_number } => {
                self.update_user(&id, first_name, last_name, phone_number).await
            }
            UserAction::Delete { id } => {
                self.delete_user(&id).await
            }
            UserAction::ListTrash { page, limit } => {
                self.list_deleted_users(page, limit).await
            }
            UserAction::Restore { id } => {
                self.restore_user(&id).await
            }
            UserAction::VerifyEmail { id } => {
                self.verify_email(&id).await
            }
            UserAction::Suspend { id, reason } => {
                self.suspend_user(&id, reason).await
            }
            UserAction::Activate { id } => {
                self.activate_user(&id).await
            }
        }
    }

    // ============================================================
    // Command Handlers
    // ============================================================

    async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
        first_name: String,
        last_name: String,
    ) -> Result<()> {
        // In production, hash the password properly using backbone-auth
        let password_hash = format!("$argon2id$v=19$m=19456,t=2,p=1$placeholder${}", password);

        let command = CreateUserCommand {
            username,
            email,
            password_hash,
            first_name,
            last_name,
            phone_number: None,
            profile_picture_url: None,
        };

        match self.user_service.create_user(command).await {
            Ok(user) => {
                println!("User created successfully!");
                self.display_user(&user);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to create user: {}", e);
                Err(e.into())
            }
        }
    }

    async fn get_user(&self, id: &str) -> Result<()> {
        match self.user_service.get_user(id).await {
            Ok(user) => {
                self.display_user(&user);
                Ok(())
            }
            Err(e) => {
                eprintln!("User not found: {}", e);
                Err(e.into())
            }
        }
    }

    async fn list_users(
        &self,
        page: i32,
        limit: i32,
        search: Option<String>,
        status: Option<String>,
    ) -> Result<()> {
        let filters = if search.is_some() || status.is_some() {
            let mut f = UserFilters::new();
            if let Some(s) = search {
                f = f.with_search(s);
            }
            if let Some(st) = status.clone() {
                f = f.with_status(st);
            }
            Some(f)
        } else {
            None
        };

        let query = ListUsersQuery {
            page,
            limit,
            status,
            email_verified: None,
            filters,
            sort_by: None,
            sort_direction: None,
        };

        match self.user_service.list_users(query).await {
            Ok(result) => {
                println!("Found {} users (page {}/{}):",
                    result.users.len(), result.page, result.total_pages);
                println!("{}", "-".repeat(60));

                for user in &result.users {
                    self.display_user_compact(user);
                }

                println!("{}", "-".repeat(60));
                println!("Total: {} | Page {}/{}", result.total, result.page, result.total_pages);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to list users: {}", e);
                Err(e.into())
            }
        }
    }

    async fn update_user(
        &self,
        id: &str,
        first_name: Option<String>,
        last_name: Option<String>,
        phone_number: Option<String>,
    ) -> Result<()> {
        let command = UpdateUserCommand {
            id: id.to_string(),
            first_name,
            last_name,
            phone_number,
            profile_picture_url: None,
        };

        match self.user_service.update_user(command).await {
            Ok(user) => {
                println!("User updated successfully!");
                self.display_user(&user);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to update user: {}", e);
                Err(e.into())
            }
        }
    }

    async fn delete_user(&self, id: &str) -> Result<()> {
        match self.user_service.delete_user(id).await {
            Ok(()) => {
                println!("User deleted successfully!");
                println!("Tip: Use 'restore' command to recover the user if needed.");
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to delete user: {}", e);
                Err(e.into())
            }
        }
    }

    async fn list_deleted_users(&self, page: i32, limit: i32) -> Result<()> {
        match self.user_service.list_deleted_users(page, limit).await {
            Ok(result) => {
                println!("Deleted users (trash):");
                println!("{}", "-".repeat(60));

                for user in &result.users {
                    self.display_user_compact(user);
                }

                println!("{}", "-".repeat(60));
                println!("Total deleted: {} | Page {}/{}", result.total, result.page, result.total_pages);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to list deleted users: {}", e);
                Err(e.into())
            }
        }
    }

    async fn restore_user(&self, id: &str) -> Result<()> {
        match self.user_service.restore_user(id).await {
            Ok(user) => {
                println!("User restored successfully!");
                self.display_user(&user);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to restore user: {}", e);
                Err(e.into())
            }
        }
    }

    async fn verify_email(&self, id: &str) -> Result<()> {
        match self.user_service.verify_email(id).await {
            Ok(()) => {
                println!("Email verified successfully for user {}", id);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to verify email: {}", e);
                Err(e.into())
            }
        }
    }

    async fn suspend_user(&self, id: &str, reason: String) -> Result<()> {
        match self.user_service.suspend_user(id, reason).await {
            Ok(()) => {
                println!("User {} suspended successfully!", id);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to suspend user: {}", e);
                Err(e.into())
            }
        }
    }

    async fn activate_user(&self, id: &str) -> Result<()> {
        match self.user_service.activate_user(id).await {
            Ok(()) => {
                println!("User {} activated successfully!", id);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to activate user: {}", e);
                Err(e.into())
            }
        }
    }

    // ============================================================
    // Display Helpers
    // ============================================================

    fn display_user(&self, user: &UserAggregate) {
        println!("User Details:");
        println!("  ID:         {}", user.id);
        println!("  Username:   {}", user.username);
        println!("  Email:      {}", user.email);
        println!("  Name:       {}", user.full_name());
        println!("  Status:     {:?}", user.status);
        println!("  Verified:   {}", user.email_verified);
        println!("  MFA:        {}", user.mfa_enabled());
    }

    fn display_user_compact(&self, user: &UserAggregate) {
        println!("{} | {} | {} | {:?}",
            user.id,
            user.username,
            user.email,
            user.status
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_action_parsing() {
        // Test that UserAction variants can be created
        let action = UserAction::Get { id: "test-123".to_string() };
        match action {
            UserAction::Get { id } => assert_eq!(id, "test-123"),
            _ => panic!("Expected Get action"),
        }
    }

    #[test]
    fn test_user_commands_structure() {
        let commands = UserCommands {
            action: UserAction::List {
                page: 1,
                limit: 20,
                search: Some("test".to_string()),
                status: None,
            },
        };

        match commands.action {
            UserAction::List { page, limit, .. } => {
                assert_eq!(page, 1);
                assert_eq!(limit, 20);
            }
            _ => panic!("Expected List action"),
        }
    }
}

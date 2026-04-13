// UserPermission CLI Commands
// Command-line interface for UserPermission CRUD operations

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

use crate::application::{
    commands::{
        CreateUserPermissionCommand, UpdateUserPermissionCommand, DeleteUserPermissionCommand,
        BulkCreateUserPermissionCommand, UpsertUserPermissionCommand, RestoreUserPermissionCommand,
        EmptyTrashCommand, UserPermissionDto, UserPermissionFilters,
    },
    queries::{
        GetUserPermissionQuery, ListUserPermissionQuery, ListDeletedUserPermissionQuery,
    },
    user_permission_services::UserPermissionApplicationServices,
};

/// UserPermission management commands
#[derive(Parser)]
#[command(name = "user_permission")]
#[command(about = "Manage user_permissions in the system")]
pub struct UserPermissionCommands {
    #[command(subcommand)]
    pub action: UserPermissionAction,
}

#[derive(Subcommand)]
pub enum UserPermissionAction {
    /// Create a new user_permission
    Create {
        /// UserPermission data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// UserPermission data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Get a user_permission by ID
    Get {
        /// UserPermission ID
        id: String,
    },
    /// List user_permissions with optional filtering
    List {
        /// Page number (default: 1)
        #[arg(short, long, default_value = "1")]
        page: usize,
        /// Page size (default: 20)
        #[arg(short, long, default_value = "20")]
        page_size: usize,
        /// Sort field
        #[arg(short, long)]
        sort_by: Option<String>,
        /// Sort direction (asc|desc)
        #[arg(long, default_value = "asc")]
        sort_direction: String,
        /// Search term
        #[arg(short, long)]
        search: Option<String>,
        /// Filter JSON
        #[arg(long)]
        filters: Option<String>,
    },
    /// Update an existing user_permission
    Update {
        /// UserPermission ID
        id: String,
        /// UserPermission data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// UserPermission data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Delete a user_permission (soft delete)
    Delete {
        /// UserPermission ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Bulk create user_permissions from file or stdin
    BulkCreate {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Upsert user_permissions (update or insert)
    Upsert {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// List deleted user_permissions (trash)
    ListTrash {
        /// Page number (default: 1)
        #[arg(short, long, default_value = "1")]
        page: usize,
        /// Page size (default: 20)
        #[arg(short, long, default_value = "20")]
        page_size: usize,
    },
    /// Restore a deleted user_permission
    Restore {
        /// UserPermission ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Empty trash (permanently delete all user_permissions)
    EmptyTrash {
        /// Confirmation flag
        #[arg(long)]
        confirm: bool,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
}

pub struct UserPermissionCliHandler {
    services: UserPermissionApplicationServices,
}

impl UserPermissionCliHandler {
    pub fn new(services: UserPermissionApplicationServices) -> Self {
        Self { services }
    }

    pub async fn handle(&self, commands: UserPermissionCommands) -> Result<()> {
        match commands.action {
            UserPermissionAction::Create { data, file, user_id } => {
                self.create_user_permission(data, file, user_id).await
            }
            UserPermissionAction::Get { id } => {
                self.get_user_permission(&id).await
            }
            UserPermissionAction::List { page, page_size, sort_by, sort_direction, search, filters } => {
                self.list_user_permissions(page, page_size, sort_by, sort_direction, search, filters).await
            }
            UserPermissionAction::Update { id, data, file, user_id } => {
                self.update_user_permission(&id, data, file, user_id).await
            }
            UserPermissionAction::Delete { id, user_id } => {
                self.delete_user_permission(&id, user_id).await
            }
            UserPermissionAction::BulkCreate { file, user_id } => {
                self.bulk_create_user_permissions(file, user_id).await
            }
            UserPermissionAction::Upsert { file, user_id } => {
                self.upsert_user_permissions(file, user_id).await
            }
            UserPermissionAction::ListTrash { page, page_size } => {
                self.list_trash(page, page_size).await
            }
            UserPermissionAction::Restore { id, user_id } => {
                self.restore_user_permission(&id, user_id).await
            }
            UserPermissionAction::EmptyTrash { confirm, user_id } => {
                self.empty_trash(confirm, user_id).await
            }
        }
    }

    async fn create_user_permission(&self, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let user_permission_data = self.parse_user_permission_data(data, file)?;

        let command = CreateUserPermissionCommand {
            // TODO: Map parsed data to command fields
            custom_fields: user_permission_data,
            created_by: user_id,
        };

        let response = self.services.create_user_permission_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ UserPermission created successfully!".green());
            if let Some(user_permission) = response.user_permission {
                self.display_user_permission(&user_permission)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn get_user_permission(&self, id: &str) -> Result<()> {
        let query = GetUserPermissionQuery {
            id: id.to_string(),
        };

        let response = self.services.get_user_permission_handler().handle(query).await?;

        if response.success {
            if let Some(user_permission) = response.user_permission {
                self.display_user_permission(&user_permission)?;
            } else {
                println!("{}", "UserPermission not found".yellow());
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_user_permissions(&self, page: usize, page_size: usize, sort_by: Option<String>, sort_direction: String, search: Option<String>, filters: Option<String>) -> Result<()> {
        let filters = if let Some(filters_json) = filters {
            Some(serde_json::from_str::<serde_json::Value>(&filters_json)?)
        } else {
            None
        };

        let filters = if let Some(filter_value) = filters {
            // TODO: Convert JSON filters to application filters
            Some(UserPermissionFilters::new().with_search(search.unwrap_or_default()))
        } else if search.is_some() {
            Some(UserPermissionFilters::new().with_search(search.unwrap_or_default()))
        } else {
            None
        };

        let query = ListUserPermissionQuery {
            page,
            page_size,
            sort_by,
            sort_direction,
            filters,
        };

        let response = self.services.list_user_permissions_handler().handle(query).await?;

        if response.success {
            println!("{}", format!("📄 Found {} user_permissions (page {}/{}):",
                response.user_permissions.len(), response.page, response.total_pages).cyan());

            for user_permission in &response.user_permissions {
                self.display_user_permission_compact(user_permission)?;
                println!("{}", "-".repeat(80));
            }

            if response.has_next || response.has_previous {
                println!();
                if response.has_previous {
                    println!("{}", format!("← Previous page: {}", response.page - 1).blue());
                }
                println!("{}", format!("Current page: {} of {}", response.page, response.total_pages).white());
                if response.has_next {
                    println!("{}", format!("Next page: {} →", response.page + 1).blue());
                }
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn update_user_permission(&self, id: &str, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let user_permission_data = self.parse_user_permission_data(data, file)?;

        let command = UpdateUserPermissionCommand {
            id: id.to_string(),
            // TODO: Map parsed data to command fields
            custom_fields: user_permission_data,
            updated_by: user_id,
        };

        let response = self.services.update_user_permission_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ UserPermission updated successfully!".green());
            if let Some(user_permission) = response.user_permission {
                self.display_user_permission(&user_permission)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn delete_user_permission(&self, id: &str, user_id: String) -> Result<()> {
        let command = DeleteUserPermissionCommand {
            id: id.to_string(),
            deleted_by: user_id,
        };

        let response = self.services.delete_user_permission_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ UserPermission deleted successfully!".green());
            println!("{}", "💡 Use 'restore' command to recover if needed".blue());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn bulk_create_user_permissions(&self, file: Option<String>, user_id: String) -> Result<()> {
        let user_permissions_data = self.parse_bulk_user_permissions_data(file)?;

        let commands: Vec<CreateUserPermissionCommand> = user_permissions_data
            .into_iter()
            .map(|data| CreateUserPermissionCommand {
                custom_fields: data,
                created_by: user_id.clone(),
            })
            .collect();

        let command = BulkCreateUserPermissionCommand {
            user_permissions: commands,
        };

        let response = self.services.bulk_create_user_permissions_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ {} user_permissions created successfully!", response.created_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn upsert_user_permissions(&self, file: Option<String>, user_id: String) -> Result<()> {
        let user_permissions_data = self.parse_bulk_user_permissions_data(file)?;

        let command = UpsertUserPermissionCommand {
            // TODO: Map parsed data to command fields
            custom_fields: user_permissions_data,
            user_id,
        };

        let response = self.services.upsert_user_permission_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Upsert completed! Created: {}, Updated: {}", response.created, response.updated).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_trash(&self, page: usize, page_size: usize) -> Result<()> {
        // This would use ListDeletedUserPermissionQuery
        println!("{}", "🗑️  Deleted user_permissions (trash):".yellow());
        println!("{}", "Feature not yet implemented - TODO: Add ListDeletedUserPermissionQuery handler".yellow());
        Ok(())
    }

    async fn restore_user_permission(&self, id: &str, user_id: String) -> Result<()> {
        let command = RestoreUserPermissionCommand {
            id: id.to_string(),
            restored_by: user_id,
        };

        let response = self.services.restore_user_permission_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ UserPermission restored successfully!".green());
            if let Some(user_permission) = response.user_permission {
                self.display_user_permission(&user_permission)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn empty_trash(&self, confirm: bool, user_id: String) -> Result<()> {
        if !confirm {
            println!("{}", "⚠️  This action will permanently delete all deleted user_permissions!".yellow());
            println!("{}", "Use --confirm to proceed".yellow());
            return Ok(());
        }

        let command = EmptyTrashCommand {
            user_id,
        };

        let response = self.services.empty_user_permission_trash_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Trash emptied! {} user_permissions permanently deleted.", response.deleted_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    // Helper methods
    fn parse_user_permission_data(&self, data: Option<String>, file: Option<String>) -> Result<std::collections::HashMap<String, serde_json::Value>> {
        if let Some(file_path) = file {
            let content = std::fs::read_to_string(file_path)?;
            Ok(serde_json::from_str(&content)?)
        } else if let Some(json_data) = data {
            Ok(serde_json::from_str(&json_data)?)
        } else {
            // Return empty hash for interactive mode
            Ok(std::collections::HashMap::new())
        }
    }

    fn parse_bulk_user_permissions_data(&self, file: Option<String>) -> Result<Vec<std::collections::HashMap<String, serde_json::Value>>> {
        let content = if let Some(file_path) = file {
            std::fs::read_to_string(file_path)?
        } else {
            // Read from stdin
            use std::io::{self, Read};
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        };

        Ok(serde_json::from_str(&content)?)
    }

    fn display_user_permission(&self, user_permission: &UserPermissionDto) -> Result<()> {
        println!("{}", format!("📋 UserPermission Details:").cyan());
        println!("{}", format!("  ID: {}", user_permission.id).white());
        // TODO: Add more field display based on your user_permission structure
        println!("{}", format!("  Created: {:?}", user_permission.created_at).white());
        println!("{}", format!("  Updated: {:?}", user_permission.updated_at).white());
        Ok(())
    }

    fn display_user_permission_compact(&self, user_permission: &UserPermissionDto) -> Result<()> {
        println!("{}", format!("🔹 {} | {:?}", user_permission.id, user_permission.created_at).white());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_permission_commands_parsing() {
        // TODO: Add CLI command parsing tests
    }
}
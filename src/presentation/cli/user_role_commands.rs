// UserRole CLI Commands
// Command-line interface for UserRole CRUD operations

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

use crate::application::{
    commands::{
        CreateUserRoleCommand, UpdateUserRoleCommand, DeleteUserRoleCommand,
        BulkCreateUserRoleCommand, UpsertUserRoleCommand, RestoreUserRoleCommand,
        EmptyTrashCommand, UserRoleDto, UserRoleFilters,
    },
    queries::{
        GetUserRoleQuery, ListUserRoleQuery, ListDeletedUserRoleQuery,
    },
    user_role_services::UserRoleApplicationServices,
};

/// UserRole management commands
#[derive(Parser)]
#[command(name = "user_role")]
#[command(about = "Manage user_roles in the system")]
pub struct UserRoleCommands {
    #[command(subcommand)]
    pub action: UserRoleAction,
}

#[derive(Subcommand)]
pub enum UserRoleAction {
    /// Create a new user_role
    Create {
        /// UserRole data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// UserRole data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Get a user_role by ID
    Get {
        /// UserRole ID
        id: String,
    },
    /// List user_roles with optional filtering
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
    /// Update an existing user_role
    Update {
        /// UserRole ID
        id: String,
        /// UserRole data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// UserRole data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Delete a user_role (soft delete)
    Delete {
        /// UserRole ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Bulk create user_roles from file or stdin
    BulkCreate {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Upsert user_roles (update or insert)
    Upsert {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// List deleted user_roles (trash)
    ListTrash {
        /// Page number (default: 1)
        #[arg(short, long, default_value = "1")]
        page: usize,
        /// Page size (default: 20)
        #[arg(short, long, default_value = "20")]
        page_size: usize,
    },
    /// Restore a deleted user_role
    Restore {
        /// UserRole ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Empty trash (permanently delete all user_roles)
    EmptyTrash {
        /// Confirmation flag
        #[arg(long)]
        confirm: bool,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
}

pub struct UserRoleCliHandler {
    services: UserRoleApplicationServices,
}

impl UserRoleCliHandler {
    pub fn new(services: UserRoleApplicationServices) -> Self {
        Self { services }
    }

    pub async fn handle(&self, commands: UserRoleCommands) -> Result<()> {
        match commands.action {
            UserRoleAction::Create { data, file, user_id } => {
                self.create_user_role(data, file, user_id).await
            }
            UserRoleAction::Get { id } => {
                self.get_user_role(&id).await
            }
            UserRoleAction::List { page, page_size, sort_by, sort_direction, search, filters } => {
                self.list_user_roles(page, page_size, sort_by, sort_direction, search, filters).await
            }
            UserRoleAction::Update { id, data, file, user_id } => {
                self.update_user_role(&id, data, file, user_id).await
            }
            UserRoleAction::Delete { id, user_id } => {
                self.delete_user_role(&id, user_id).await
            }
            UserRoleAction::BulkCreate { file, user_id } => {
                self.bulk_create_user_roles(file, user_id).await
            }
            UserRoleAction::Upsert { file, user_id } => {
                self.upsert_user_roles(file, user_id).await
            }
            UserRoleAction::ListTrash { page, page_size } => {
                self.list_trash(page, page_size).await
            }
            UserRoleAction::Restore { id, user_id } => {
                self.restore_user_role(&id, user_id).await
            }
            UserRoleAction::EmptyTrash { confirm, user_id } => {
                self.empty_trash(confirm, user_id).await
            }
        }
    }

    async fn create_user_role(&self, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let user_role_data = self.parse_user_role_data(data, file)?;

        let command = CreateUserRoleCommand {
            // TODO: Map parsed data to command fields
            custom_fields: user_role_data,
            created_by: user_id,
        };

        let response = self.services.create_user_role_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ UserRole created successfully!".green());
            if let Some(user_role) = response.user_role {
                self.display_user_role(&user_role)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn get_user_role(&self, id: &str) -> Result<()> {
        let query = GetUserRoleQuery {
            id: id.to_string(),
        };

        let response = self.services.get_user_role_handler().handle(query).await?;

        if response.success {
            if let Some(user_role) = response.user_role {
                self.display_user_role(&user_role)?;
            } else {
                println!("{}", "UserRole not found".yellow());
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_user_roles(&self, page: usize, page_size: usize, sort_by: Option<String>, sort_direction: String, search: Option<String>, filters: Option<String>) -> Result<()> {
        let filters = if let Some(filters_json) = filters {
            Some(serde_json::from_str::<serde_json::Value>(&filters_json)?)
        } else {
            None
        };

        let filters = if let Some(filter_value) = filters {
            // TODO: Convert JSON filters to application filters
            Some(UserRoleFilters::new().with_search(search.unwrap_or_default()))
        } else if search.is_some() {
            Some(UserRoleFilters::new().with_search(search.unwrap_or_default()))
        } else {
            None
        };

        let query = ListUserRoleQuery {
            page,
            page_size,
            sort_by,
            sort_direction,
            filters,
        };

        let response = self.services.list_user_roles_handler().handle(query).await?;

        if response.success {
            println!("{}", format!("📄 Found {} user_roles (page {}/{}):",
                response.user_roles.len(), response.page, response.total_pages).cyan());

            for user_role in &response.user_roles {
                self.display_user_role_compact(user_role)?;
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

    async fn update_user_role(&self, id: &str, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let user_role_data = self.parse_user_role_data(data, file)?;

        let command = UpdateUserRoleCommand {
            id: id.to_string(),
            // TODO: Map parsed data to command fields
            custom_fields: user_role_data,
            updated_by: user_id,
        };

        let response = self.services.update_user_role_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ UserRole updated successfully!".green());
            if let Some(user_role) = response.user_role {
                self.display_user_role(&user_role)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn delete_user_role(&self, id: &str, user_id: String) -> Result<()> {
        let command = DeleteUserRoleCommand {
            id: id.to_string(),
            deleted_by: user_id,
        };

        let response = self.services.delete_user_role_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ UserRole deleted successfully!".green());
            println!("{}", "💡 Use 'restore' command to recover if needed".blue());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn bulk_create_user_roles(&self, file: Option<String>, user_id: String) -> Result<()> {
        let user_roles_data = self.parse_bulk_user_roles_data(file)?;

        let commands: Vec<CreateUserRoleCommand> = user_roles_data
            .into_iter()
            .map(|data| CreateUserRoleCommand {
                custom_fields: data,
                created_by: user_id.clone(),
            })
            .collect();

        let command = BulkCreateUserRoleCommand {
            user_roles: commands,
        };

        let response = self.services.bulk_create_user_roles_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ {} user_roles created successfully!", response.created_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn upsert_user_roles(&self, file: Option<String>, user_id: String) -> Result<()> {
        let user_roles_data = self.parse_bulk_user_roles_data(file)?;

        let command = UpsertUserRoleCommand {
            // TODO: Map parsed data to command fields
            custom_fields: user_roles_data,
            user_id,
        };

        let response = self.services.upsert_user_role_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Upsert completed! Created: {}, Updated: {}", response.created, response.updated).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_trash(&self, page: usize, page_size: usize) -> Result<()> {
        // This would use ListDeletedUserRoleQuery
        println!("{}", "🗑️  Deleted user_roles (trash):".yellow());
        println!("{}", "Feature not yet implemented - TODO: Add ListDeletedUserRoleQuery handler".yellow());
        Ok(())
    }

    async fn restore_user_role(&self, id: &str, user_id: String) -> Result<()> {
        let command = RestoreUserRoleCommand {
            id: id.to_string(),
            restored_by: user_id,
        };

        let response = self.services.restore_user_role_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ UserRole restored successfully!".green());
            if let Some(user_role) = response.user_role {
                self.display_user_role(&user_role)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn empty_trash(&self, confirm: bool, user_id: String) -> Result<()> {
        if !confirm {
            println!("{}", "⚠️  This action will permanently delete all deleted user_roles!".yellow());
            println!("{}", "Use --confirm to proceed".yellow());
            return Ok(());
        }

        let command = EmptyTrashCommand {
            user_id,
        };

        let response = self.services.empty_user_role_trash_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Trash emptied! {} user_roles permanently deleted.", response.deleted_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    // Helper methods
    fn parse_user_role_data(&self, data: Option<String>, file: Option<String>) -> Result<std::collections::HashMap<String, serde_json::Value>> {
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

    fn parse_bulk_user_roles_data(&self, file: Option<String>) -> Result<Vec<std::collections::HashMap<String, serde_json::Value>>> {
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

    fn display_user_role(&self, user_role: &UserRoleDto) -> Result<()> {
        println!("{}", format!("📋 UserRole Details:").cyan());
        println!("{}", format!("  ID: {}", user_role.id).white());
        // TODO: Add more field display based on your user_role structure
        println!("{}", format!("  Created: {:?}", user_role.created_at).white());
        println!("{}", format!("  Updated: {:?}", user_role.updated_at).white());
        Ok(())
    }

    fn display_user_role_compact(&self, user_role: &UserRoleDto) -> Result<()> {
        println!("{}", format!("🔹 {} | {:?}", user_role.id, user_role.created_at).white());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_role_commands_parsing() {
        // TODO: Add CLI command parsing tests
    }
}
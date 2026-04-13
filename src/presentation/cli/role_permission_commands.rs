// RolePermission CLI Commands
// Command-line interface for RolePermission CRUD operations

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

use crate::application::{
    commands::{
        CreateRolePermissionCommand, UpdateRolePermissionCommand, DeleteRolePermissionCommand,
        BulkCreateRolePermissionCommand, UpsertRolePermissionCommand, RestoreRolePermissionCommand,
        EmptyTrashCommand, RolePermissionDto, RolePermissionFilters,
    },
    queries::{
        GetRolePermissionQuery, ListRolePermissionQuery, ListDeletedRolePermissionQuery,
    },
    role_permission_services::RolePermissionApplicationServices,
};

/// RolePermission management commands
#[derive(Parser)]
#[command(name = "role_permission")]
#[command(about = "Manage role_permissions in the system")]
pub struct RolePermissionCommands {
    #[command(subcommand)]
    pub action: RolePermissionAction,
}

#[derive(Subcommand)]
pub enum RolePermissionAction {
    /// Create a new role_permission
    Create {
        /// RolePermission data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// RolePermission data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Get a role_permission by ID
    Get {
        /// RolePermission ID
        id: String,
    },
    /// List role_permissions with optional filtering
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
    /// Update an existing role_permission
    Update {
        /// RolePermission ID
        id: String,
        /// RolePermission data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// RolePermission data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Delete a role_permission (soft delete)
    Delete {
        /// RolePermission ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Bulk create role_permissions from file or stdin
    BulkCreate {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Upsert role_permissions (update or insert)
    Upsert {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// List deleted role_permissions (trash)
    ListTrash {
        /// Page number (default: 1)
        #[arg(short, long, default_value = "1")]
        page: usize,
        /// Page size (default: 20)
        #[arg(short, long, default_value = "20")]
        page_size: usize,
    },
    /// Restore a deleted role_permission
    Restore {
        /// RolePermission ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Empty trash (permanently delete all role_permissions)
    EmptyTrash {
        /// Confirmation flag
        #[arg(long)]
        confirm: bool,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
}

pub struct RolePermissionCliHandler {
    services: RolePermissionApplicationServices,
}

impl RolePermissionCliHandler {
    pub fn new(services: RolePermissionApplicationServices) -> Self {
        Self { services }
    }

    pub async fn handle(&self, commands: RolePermissionCommands) -> Result<()> {
        match commands.action {
            RolePermissionAction::Create { data, file, user_id } => {
                self.create_role_permission(data, file, user_id).await
            }
            RolePermissionAction::Get { id } => {
                self.get_role_permission(&id).await
            }
            RolePermissionAction::List { page, page_size, sort_by, sort_direction, search, filters } => {
                self.list_role_permissions(page, page_size, sort_by, sort_direction, search, filters).await
            }
            RolePermissionAction::Update { id, data, file, user_id } => {
                self.update_role_permission(&id, data, file, user_id).await
            }
            RolePermissionAction::Delete { id, user_id } => {
                self.delete_role_permission(&id, user_id).await
            }
            RolePermissionAction::BulkCreate { file, user_id } => {
                self.bulk_create_role_permissions(file, user_id).await
            }
            RolePermissionAction::Upsert { file, user_id } => {
                self.upsert_role_permissions(file, user_id).await
            }
            RolePermissionAction::ListTrash { page, page_size } => {
                self.list_trash(page, page_size).await
            }
            RolePermissionAction::Restore { id, user_id } => {
                self.restore_role_permission(&id, user_id).await
            }
            RolePermissionAction::EmptyTrash { confirm, user_id } => {
                self.empty_trash(confirm, user_id).await
            }
        }
    }

    async fn create_role_permission(&self, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let role_permission_data = self.parse_role_permission_data(data, file)?;

        let command = CreateRolePermissionCommand {
            // TODO: Map parsed data to command fields
            custom_fields: role_permission_data,
            created_by: user_id,
        };

        let response = self.services.create_role_permission_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ RolePermission created successfully!".green());
            if let Some(role_permission) = response.role_permission {
                self.display_role_permission(&role_permission)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn get_role_permission(&self, id: &str) -> Result<()> {
        let query = GetRolePermissionQuery {
            id: id.to_string(),
        };

        let response = self.services.get_role_permission_handler().handle(query).await?;

        if response.success {
            if let Some(role_permission) = response.role_permission {
                self.display_role_permission(&role_permission)?;
            } else {
                println!("{}", "RolePermission not found".yellow());
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_role_permissions(&self, page: usize, page_size: usize, sort_by: Option<String>, sort_direction: String, search: Option<String>, filters: Option<String>) -> Result<()> {
        let filters = if let Some(filters_json) = filters {
            Some(serde_json::from_str::<serde_json::Value>(&filters_json)?)
        } else {
            None
        };

        let filters = if let Some(filter_value) = filters {
            // TODO: Convert JSON filters to application filters
            Some(RolePermissionFilters::new().with_search(search.unwrap_or_default()))
        } else if search.is_some() {
            Some(RolePermissionFilters::new().with_search(search.unwrap_or_default()))
        } else {
            None
        };

        let query = ListRolePermissionQuery {
            page,
            page_size,
            sort_by,
            sort_direction,
            filters,
        };

        let response = self.services.list_role_permissions_handler().handle(query).await?;

        if response.success {
            println!("{}", format!("📄 Found {} role_permissions (page {}/{}):",
                response.role_permissions.len(), response.page, response.total_pages).cyan());

            for role_permission in &response.role_permissions {
                self.display_role_permission_compact(role_permission)?;
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

    async fn update_role_permission(&self, id: &str, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let role_permission_data = self.parse_role_permission_data(data, file)?;

        let command = UpdateRolePermissionCommand {
            id: id.to_string(),
            // TODO: Map parsed data to command fields
            custom_fields: role_permission_data,
            updated_by: user_id,
        };

        let response = self.services.update_role_permission_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ RolePermission updated successfully!".green());
            if let Some(role_permission) = response.role_permission {
                self.display_role_permission(&role_permission)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn delete_role_permission(&self, id: &str, user_id: String) -> Result<()> {
        let command = DeleteRolePermissionCommand {
            id: id.to_string(),
            deleted_by: user_id,
        };

        let response = self.services.delete_role_permission_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ RolePermission deleted successfully!".green());
            println!("{}", "💡 Use 'restore' command to recover if needed".blue());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn bulk_create_role_permissions(&self, file: Option<String>, user_id: String) -> Result<()> {
        let role_permissions_data = self.parse_bulk_role_permissions_data(file)?;

        let commands: Vec<CreateRolePermissionCommand> = role_permissions_data
            .into_iter()
            .map(|data| CreateRolePermissionCommand {
                custom_fields: data,
                created_by: user_id.clone(),
            })
            .collect();

        let command = BulkCreateRolePermissionCommand {
            role_permissions: commands,
        };

        let response = self.services.bulk_create_role_permissions_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ {} role_permissions created successfully!", response.created_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn upsert_role_permissions(&self, file: Option<String>, user_id: String) -> Result<()> {
        let role_permissions_data = self.parse_bulk_role_permissions_data(file)?;

        let command = UpsertRolePermissionCommand {
            // TODO: Map parsed data to command fields
            custom_fields: role_permissions_data,
            user_id,
        };

        let response = self.services.upsert_role_permission_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Upsert completed! Created: {}, Updated: {}", response.created, response.updated).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_trash(&self, page: usize, page_size: usize) -> Result<()> {
        // This would use ListDeletedRolePermissionQuery
        println!("{}", "🗑️  Deleted role_permissions (trash):".yellow());
        println!("{}", "Feature not yet implemented - TODO: Add ListDeletedRolePermissionQuery handler".yellow());
        Ok(())
    }

    async fn restore_role_permission(&self, id: &str, user_id: String) -> Result<()> {
        let command = RestoreRolePermissionCommand {
            id: id.to_string(),
            restored_by: user_id,
        };

        let response = self.services.restore_role_permission_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ RolePermission restored successfully!".green());
            if let Some(role_permission) = response.role_permission {
                self.display_role_permission(&role_permission)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn empty_trash(&self, confirm: bool, user_id: String) -> Result<()> {
        if !confirm {
            println!("{}", "⚠️  This action will permanently delete all deleted role_permissions!".yellow());
            println!("{}", "Use --confirm to proceed".yellow());
            return Ok(());
        }

        let command = EmptyTrashCommand {
            user_id,
        };

        let response = self.services.empty_role_permission_trash_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Trash emptied! {} role_permissions permanently deleted.", response.deleted_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    // Helper methods
    fn parse_role_permission_data(&self, data: Option<String>, file: Option<String>) -> Result<std::collections::HashMap<String, serde_json::Value>> {
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

    fn parse_bulk_role_permissions_data(&self, file: Option<String>) -> Result<Vec<std::collections::HashMap<String, serde_json::Value>>> {
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

    fn display_role_permission(&self, role_permission: &RolePermissionDto) -> Result<()> {
        println!("{}", format!("📋 RolePermission Details:").cyan());
        println!("{}", format!("  ID: {}", role_permission.id).white());
        // TODO: Add more field display based on your role_permission structure
        println!("{}", format!("  Created: {:?}", role_permission.created_at).white());
        println!("{}", format!("  Updated: {:?}", role_permission.updated_at).white());
        Ok(())
    }

    fn display_role_permission_compact(&self, role_permission: &RolePermissionDto) -> Result<()> {
        println!("{}", format!("🔹 {} | {:?}", role_permission.id, role_permission.created_at).white());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_permission_commands_parsing() {
        // TODO: Add CLI command parsing tests
    }
}
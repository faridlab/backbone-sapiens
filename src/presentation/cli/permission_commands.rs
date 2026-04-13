// Permission CLI Commands
// Command-line interface for Permission CRUD operations

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

use crate::application::{
    commands::{
        CreatePermissionCommand, UpdatePermissionCommand, DeletePermissionCommand,
        BulkCreatePermissionCommand, UpsertPermissionCommand, RestorePermissionCommand,
        EmptyTrashCommand, PermissionDto, PermissionFilters,
    },
    queries::{
        GetPermissionQuery, ListPermissionQuery, ListDeletedPermissionQuery,
    },
    permission_services::PermissionApplicationServices,
};

/// Permission management commands
#[derive(Parser)]
#[command(name = "permission")]
#[command(about = "Manage permissions in the system")]
pub struct PermissionCommands {
    #[command(subcommand)]
    pub action: PermissionAction,
}

#[derive(Subcommand)]
pub enum PermissionAction {
    /// Create a new permission
    Create {
        /// Permission data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// Permission data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Get a permission by ID
    Get {
        /// Permission ID
        id: String,
    },
    /// List permissions with optional filtering
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
    /// Update an existing permission
    Update {
        /// Permission ID
        id: String,
        /// Permission data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// Permission data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Delete a permission (soft delete)
    Delete {
        /// Permission ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Bulk create permissions from file or stdin
    BulkCreate {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Upsert permissions (update or insert)
    Upsert {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// List deleted permissions (trash)
    ListTrash {
        /// Page number (default: 1)
        #[arg(short, long, default_value = "1")]
        page: usize,
        /// Page size (default: 20)
        #[arg(short, long, default_value = "20")]
        page_size: usize,
    },
    /// Restore a deleted permission
    Restore {
        /// Permission ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Empty trash (permanently delete all permissions)
    EmptyTrash {
        /// Confirmation flag
        #[arg(long)]
        confirm: bool,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
}

pub struct PermissionCliHandler {
    services: PermissionApplicationServices,
}

impl PermissionCliHandler {
    pub fn new(services: PermissionApplicationServices) -> Self {
        Self { services }
    }

    pub async fn handle(&self, commands: PermissionCommands) -> Result<()> {
        match commands.action {
            PermissionAction::Create { data, file, user_id } => {
                self.create_permission(data, file, user_id).await
            }
            PermissionAction::Get { id } => {
                self.get_permission(&id).await
            }
            PermissionAction::List { page, page_size, sort_by, sort_direction, search, filters } => {
                self.list_permissions(page, page_size, sort_by, sort_direction, search, filters).await
            }
            PermissionAction::Update { id, data, file, user_id } => {
                self.update_permission(&id, data, file, user_id).await
            }
            PermissionAction::Delete { id, user_id } => {
                self.delete_permission(&id, user_id).await
            }
            PermissionAction::BulkCreate { file, user_id } => {
                self.bulk_create_permissions(file, user_id).await
            }
            PermissionAction::Upsert { file, user_id } => {
                self.upsert_permissions(file, user_id).await
            }
            PermissionAction::ListTrash { page, page_size } => {
                self.list_trash(page, page_size).await
            }
            PermissionAction::Restore { id, user_id } => {
                self.restore_permission(&id, user_id).await
            }
            PermissionAction::EmptyTrash { confirm, user_id } => {
                self.empty_trash(confirm, user_id).await
            }
        }
    }

    async fn create_permission(&self, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let permission_data = self.parse_permission_data(data, file)?;

        let command = CreatePermissionCommand {
            // TODO: Map parsed data to command fields
            custom_fields: permission_data,
            created_by: user_id,
        };

        let response = self.services.create_permission_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ Permission created successfully!".green());
            if let Some(permission) = response.permission {
                self.display_permission(&permission)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn get_permission(&self, id: &str) -> Result<()> {
        let query = GetPermissionQuery {
            id: id.to_string(),
        };

        let response = self.services.get_permission_handler().handle(query).await?;

        if response.success {
            if let Some(permission) = response.permission {
                self.display_permission(&permission)?;
            } else {
                println!("{}", "Permission not found".yellow());
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_permissions(&self, page: usize, page_size: usize, sort_by: Option<String>, sort_direction: String, search: Option<String>, filters: Option<String>) -> Result<()> {
        let filters = if let Some(filters_json) = filters {
            Some(serde_json::from_str::<serde_json::Value>(&filters_json)?)
        } else {
            None
        };

        let filters = if let Some(filter_value) = filters {
            // TODO: Convert JSON filters to application filters
            Some(PermissionFilters::new().with_search(search.unwrap_or_default()))
        } else if search.is_some() {
            Some(PermissionFilters::new().with_search(search.unwrap_or_default()))
        } else {
            None
        };

        let query = ListPermissionQuery {
            page,
            page_size,
            sort_by,
            sort_direction,
            filters,
        };

        let response = self.services.list_permissions_handler().handle(query).await?;

        if response.success {
            println!("{}", format!("📄 Found {} permissions (page {}/{}):",
                response.permissions.len(), response.page, response.total_pages).cyan());

            for permission in &response.permissions {
                self.display_permission_compact(permission)?;
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

    async fn update_permission(&self, id: &str, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let permission_data = self.parse_permission_data(data, file)?;

        let command = UpdatePermissionCommand {
            id: id.to_string(),
            // TODO: Map parsed data to command fields
            custom_fields: permission_data,
            updated_by: user_id,
        };

        let response = self.services.update_permission_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ Permission updated successfully!".green());
            if let Some(permission) = response.permission {
                self.display_permission(&permission)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn delete_permission(&self, id: &str, user_id: String) -> Result<()> {
        let command = DeletePermissionCommand {
            id: id.to_string(),
            deleted_by: user_id,
        };

        let response = self.services.delete_permission_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ Permission deleted successfully!".green());
            println!("{}", "💡 Use 'restore' command to recover if needed".blue());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn bulk_create_permissions(&self, file: Option<String>, user_id: String) -> Result<()> {
        let permissions_data = self.parse_bulk_permissions_data(file)?;

        let commands: Vec<CreatePermissionCommand> = permissions_data
            .into_iter()
            .map(|data| CreatePermissionCommand {
                custom_fields: data,
                created_by: user_id.clone(),
            })
            .collect();

        let command = BulkCreatePermissionCommand {
            permissions: commands,
        };

        let response = self.services.bulk_create_permissions_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ {} permissions created successfully!", response.created_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn upsert_permissions(&self, file: Option<String>, user_id: String) -> Result<()> {
        let permissions_data = self.parse_bulk_permissions_data(file)?;

        let command = UpsertPermissionCommand {
            // TODO: Map parsed data to command fields
            custom_fields: permissions_data,
            user_id,
        };

        let response = self.services.upsert_permission_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Upsert completed! Created: {}, Updated: {}", response.created, response.updated).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_trash(&self, page: usize, page_size: usize) -> Result<()> {
        // This would use ListDeletedPermissionQuery
        println!("{}", "🗑️  Deleted permissions (trash):".yellow());
        println!("{}", "Feature not yet implemented - TODO: Add ListDeletedPermissionQuery handler".yellow());
        Ok(())
    }

    async fn restore_permission(&self, id: &str, user_id: String) -> Result<()> {
        let command = RestorePermissionCommand {
            id: id.to_string(),
            restored_by: user_id,
        };

        let response = self.services.restore_permission_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ Permission restored successfully!".green());
            if let Some(permission) = response.permission {
                self.display_permission(&permission)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn empty_trash(&self, confirm: bool, user_id: String) -> Result<()> {
        if !confirm {
            println!("{}", "⚠️  This action will permanently delete all deleted permissions!".yellow());
            println!("{}", "Use --confirm to proceed".yellow());
            return Ok(());
        }

        let command = EmptyTrashCommand {
            user_id,
        };

        let response = self.services.empty_permission_trash_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Trash emptied! {} permissions permanently deleted.", response.deleted_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    // Helper methods
    fn parse_permission_data(&self, data: Option<String>, file: Option<String>) -> Result<std::collections::HashMap<String, serde_json::Value>> {
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

    fn parse_bulk_permissions_data(&self, file: Option<String>) -> Result<Vec<std::collections::HashMap<String, serde_json::Value>>> {
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

    fn display_permission(&self, permission: &PermissionDto) -> Result<()> {
        println!("{}", format!("📋 Permission Details:").cyan());
        println!("{}", format!("  ID: {}", permission.id).white());
        // TODO: Add more field display based on your permission structure
        println!("{}", format!("  Created: {:?}", permission.created_at).white());
        println!("{}", format!("  Updated: {:?}", permission.updated_at).white());
        Ok(())
    }

    fn display_permission_compact(&self, permission: &PermissionDto) -> Result<()> {
        println!("{}", format!("🔹 {} | {:?}", permission.id, permission.created_at).white());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_commands_parsing() {
        // TODO: Add CLI command parsing tests
    }
}
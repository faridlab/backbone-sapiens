// Role CLI Commands
// Command-line interface for Role CRUD operations

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

use crate::application::{
    commands::{
        CreateRoleCommand, UpdateRoleCommand, DeleteRoleCommand,
        BulkCreateRoleCommand, UpsertRoleCommand, RestoreRoleCommand,
        EmptyTrashCommand, RoleDto, RoleFilters,
    },
    queries::{
        GetRoleQuery, ListRoleQuery, ListDeletedRoleQuery,
    },
    role_services::RoleApplicationServices,
};

/// Role management commands
#[derive(Parser)]
#[command(name = "role")]
#[command(about = "Manage roles in the system")]
pub struct RoleCommands {
    #[command(subcommand)]
    pub action: RoleAction,
}

#[derive(Subcommand)]
pub enum RoleAction {
    /// Create a new role
    Create {
        /// Role data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// Role data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Get a role by ID
    Get {
        /// Role ID
        id: String,
    },
    /// List roles with optional filtering
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
    /// Update an existing role
    Update {
        /// Role ID
        id: String,
        /// Role data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// Role data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Delete a role (soft delete)
    Delete {
        /// Role ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Bulk create roles from file or stdin
    BulkCreate {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Upsert roles (update or insert)
    Upsert {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// List deleted roles (trash)
    ListTrash {
        /// Page number (default: 1)
        #[arg(short, long, default_value = "1")]
        page: usize,
        /// Page size (default: 20)
        #[arg(short, long, default_value = "20")]
        page_size: usize,
    },
    /// Restore a deleted role
    Restore {
        /// Role ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Empty trash (permanently delete all roles)
    EmptyTrash {
        /// Confirmation flag
        #[arg(long)]
        confirm: bool,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
}

pub struct RoleCliHandler {
    services: RoleApplicationServices,
}

impl RoleCliHandler {
    pub fn new(services: RoleApplicationServices) -> Self {
        Self { services }
    }

    pub async fn handle(&self, commands: RoleCommands) -> Result<()> {
        match commands.action {
            RoleAction::Create { data, file, user_id } => {
                self.create_role(data, file, user_id).await
            }
            RoleAction::Get { id } => {
                self.get_role(&id).await
            }
            RoleAction::List { page, page_size, sort_by, sort_direction, search, filters } => {
                self.list_roles(page, page_size, sort_by, sort_direction, search, filters).await
            }
            RoleAction::Update { id, data, file, user_id } => {
                self.update_role(&id, data, file, user_id).await
            }
            RoleAction::Delete { id, user_id } => {
                self.delete_role(&id, user_id).await
            }
            RoleAction::BulkCreate { file, user_id } => {
                self.bulk_create_roles(file, user_id).await
            }
            RoleAction::Upsert { file, user_id } => {
                self.upsert_roles(file, user_id).await
            }
            RoleAction::ListTrash { page, page_size } => {
                self.list_trash(page, page_size).await
            }
            RoleAction::Restore { id, user_id } => {
                self.restore_role(&id, user_id).await
            }
            RoleAction::EmptyTrash { confirm, user_id } => {
                self.empty_trash(confirm, user_id).await
            }
        }
    }

    async fn create_role(&self, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let role_data = self.parse_role_data(data, file)?;

        let command = CreateRoleCommand {
            // TODO: Map parsed data to command fields
            custom_fields: role_data,
            created_by: user_id,
        };

        let response = self.services.create_role_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ Role created successfully!".green());
            if let Some(role) = response.role {
                self.display_role(&role)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn get_role(&self, id: &str) -> Result<()> {
        let query = GetRoleQuery {
            id: id.to_string(),
        };

        let response = self.services.get_role_handler().handle(query).await?;

        if response.success {
            if let Some(role) = response.role {
                self.display_role(&role)?;
            } else {
                println!("{}", "Role not found".yellow());
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_roles(&self, page: usize, page_size: usize, sort_by: Option<String>, sort_direction: String, search: Option<String>, filters: Option<String>) -> Result<()> {
        let filters = if let Some(filters_json) = filters {
            Some(serde_json::from_str::<serde_json::Value>(&filters_json)?)
        } else {
            None
        };

        let filters = if let Some(filter_value) = filters {
            // TODO: Convert JSON filters to application filters
            Some(RoleFilters::new().with_search(search.unwrap_or_default()))
        } else if search.is_some() {
            Some(RoleFilters::new().with_search(search.unwrap_or_default()))
        } else {
            None
        };

        let query = ListRoleQuery {
            page,
            page_size,
            sort_by,
            sort_direction,
            filters,
        };

        let response = self.services.list_roles_handler().handle(query).await?;

        if response.success {
            println!("{}", format!("📄 Found {} roles (page {}/{}):",
                response.roles.len(), response.page, response.total_pages).cyan());

            for role in &response.roles {
                self.display_role_compact(role)?;
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

    async fn update_role(&self, id: &str, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let role_data = self.parse_role_data(data, file)?;

        let command = UpdateRoleCommand {
            id: id.to_string(),
            // TODO: Map parsed data to command fields
            custom_fields: role_data,
            updated_by: user_id,
        };

        let response = self.services.update_role_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ Role updated successfully!".green());
            if let Some(role) = response.role {
                self.display_role(&role)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn delete_role(&self, id: &str, user_id: String) -> Result<()> {
        let command = DeleteRoleCommand {
            id: id.to_string(),
            deleted_by: user_id,
        };

        let response = self.services.delete_role_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ Role deleted successfully!".green());
            println!("{}", "💡 Use 'restore' command to recover if needed".blue());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn bulk_create_roles(&self, file: Option<String>, user_id: String) -> Result<()> {
        let roles_data = self.parse_bulk_roles_data(file)?;

        let commands: Vec<CreateRoleCommand> = roles_data
            .into_iter()
            .map(|data| CreateRoleCommand {
                custom_fields: data,
                created_by: user_id.clone(),
            })
            .collect();

        let command = BulkCreateRoleCommand {
            roles: commands,
        };

        let response = self.services.bulk_create_roles_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ {} roles created successfully!", response.created_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn upsert_roles(&self, file: Option<String>, user_id: String) -> Result<()> {
        let roles_data = self.parse_bulk_roles_data(file)?;

        let command = UpsertRoleCommand {
            // TODO: Map parsed data to command fields
            custom_fields: roles_data,
            user_id,
        };

        let response = self.services.upsert_role_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Upsert completed! Created: {}, Updated: {}", response.created, response.updated).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_trash(&self, page: usize, page_size: usize) -> Result<()> {
        // This would use ListDeletedRoleQuery
        println!("{}", "🗑️  Deleted roles (trash):".yellow());
        println!("{}", "Feature not yet implemented - TODO: Add ListDeletedRoleQuery handler".yellow());
        Ok(())
    }

    async fn restore_role(&self, id: &str, user_id: String) -> Result<()> {
        let command = RestoreRoleCommand {
            id: id.to_string(),
            restored_by: user_id,
        };

        let response = self.services.restore_role_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ Role restored successfully!".green());
            if let Some(role) = response.role {
                self.display_role(&role)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn empty_trash(&self, confirm: bool, user_id: String) -> Result<()> {
        if !confirm {
            println!("{}", "⚠️  This action will permanently delete all deleted roles!".yellow());
            println!("{}", "Use --confirm to proceed".yellow());
            return Ok(());
        }

        let command = EmptyTrashCommand {
            user_id,
        };

        let response = self.services.empty_role_trash_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Trash emptied! {} roles permanently deleted.", response.deleted_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    // Helper methods
    fn parse_role_data(&self, data: Option<String>, file: Option<String>) -> Result<std::collections::HashMap<String, serde_json::Value>> {
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

    fn parse_bulk_roles_data(&self, file: Option<String>) -> Result<Vec<std::collections::HashMap<String, serde_json::Value>>> {
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

    fn display_role(&self, role: &RoleDto) -> Result<()> {
        println!("{}", format!("📋 Role Details:").cyan());
        println!("{}", format!("  ID: {}", role.id).white());
        // TODO: Add more field display based on your role structure
        println!("{}", format!("  Created: {:?}", role.created_at).white());
        println!("{}", format!("  Updated: {:?}", role.updated_at).white());
        Ok(())
    }

    fn display_role_compact(&self, role: &RoleDto) -> Result<()> {
        println!("{}", format!("🔹 {} | {:?}", role.id, role.created_at).white());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_commands_parsing() {
        // TODO: Add CLI command parsing tests
    }
}
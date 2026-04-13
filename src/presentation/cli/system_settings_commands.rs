// SystemSettings CLI Commands
// Command-line interface for SystemSettings CRUD operations

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

use crate::application::{
    commands::{
        CreateSystemSettingsCommand, UpdateSystemSettingsCommand, DeleteSystemSettingsCommand,
        BulkCreateSystemSettingsCommand, UpsertSystemSettingsCommand, RestoreSystemSettingsCommand,
        EmptyTrashCommand, SystemSettingsDto, SystemSettingsFilters,
    },
    queries::{
        GetSystemSettingsQuery, ListSystemSettingsQuery, ListDeletedSystemSettingsQuery,
    },
    system_settings_services::SystemSettingsApplicationServices,
};

/// SystemSettings management commands
#[derive(Parser)]
#[command(name = "system_settings")]
#[command(about = "Manage system_settingses in the system")]
pub struct SystemSettingsCommands {
    #[command(subcommand)]
    pub action: SystemSettingsAction,
}

#[derive(Subcommand)]
pub enum SystemSettingsAction {
    /// Create a new system_settings
    Create {
        /// SystemSettings data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// SystemSettings data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Get a system_settings by ID
    Get {
        /// SystemSettings ID
        id: String,
    },
    /// List system_settingses with optional filtering
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
    /// Update an existing system_settings
    Update {
        /// SystemSettings ID
        id: String,
        /// SystemSettings data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// SystemSettings data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Delete a system_settings (soft delete)
    Delete {
        /// SystemSettings ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Bulk create system_settingses from file or stdin
    BulkCreate {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Upsert system_settingses (update or insert)
    Upsert {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// List deleted system_settingses (trash)
    ListTrash {
        /// Page number (default: 1)
        #[arg(short, long, default_value = "1")]
        page: usize,
        /// Page size (default: 20)
        #[arg(short, long, default_value = "20")]
        page_size: usize,
    },
    /// Restore a deleted system_settings
    Restore {
        /// SystemSettings ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Empty trash (permanently delete all system_settingses)
    EmptyTrash {
        /// Confirmation flag
        #[arg(long)]
        confirm: bool,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
}

pub struct SystemSettingsCliHandler {
    services: SystemSettingsApplicationServices,
}

impl SystemSettingsCliHandler {
    pub fn new(services: SystemSettingsApplicationServices) -> Self {
        Self { services }
    }

    pub async fn handle(&self, commands: SystemSettingsCommands) -> Result<()> {
        match commands.action {
            SystemSettingsAction::Create { data, file, user_id } => {
                self.create_system_settings(data, file, user_id).await
            }
            SystemSettingsAction::Get { id } => {
                self.get_system_settings(&id).await
            }
            SystemSettingsAction::List { page, page_size, sort_by, sort_direction, search, filters } => {
                self.list_system_settingses(page, page_size, sort_by, sort_direction, search, filters).await
            }
            SystemSettingsAction::Update { id, data, file, user_id } => {
                self.update_system_settings(&id, data, file, user_id).await
            }
            SystemSettingsAction::Delete { id, user_id } => {
                self.delete_system_settings(&id, user_id).await
            }
            SystemSettingsAction::BulkCreate { file, user_id } => {
                self.bulk_create_system_settingses(file, user_id).await
            }
            SystemSettingsAction::Upsert { file, user_id } => {
                self.upsert_system_settingses(file, user_id).await
            }
            SystemSettingsAction::ListTrash { page, page_size } => {
                self.list_trash(page, page_size).await
            }
            SystemSettingsAction::Restore { id, user_id } => {
                self.restore_system_settings(&id, user_id).await
            }
            SystemSettingsAction::EmptyTrash { confirm, user_id } => {
                self.empty_trash(confirm, user_id).await
            }
        }
    }

    async fn create_system_settings(&self, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let system_settings_data = self.parse_system_settings_data(data, file)?;

        let command = CreateSystemSettingsCommand {
            // TODO: Map parsed data to command fields
            custom_fields: system_settings_data,
            created_by: user_id,
        };

        let response = self.services.create_system_settings_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ SystemSettings created successfully!".green());
            if let Some(system_settings) = response.system_settings {
                self.display_system_settings(&system_settings)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn get_system_settings(&self, id: &str) -> Result<()> {
        let query = GetSystemSettingsQuery {
            id: id.to_string(),
        };

        let response = self.services.get_system_settings_handler().handle(query).await?;

        if response.success {
            if let Some(system_settings) = response.system_settings {
                self.display_system_settings(&system_settings)?;
            } else {
                println!("{}", "SystemSettings not found".yellow());
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_system_settingses(&self, page: usize, page_size: usize, sort_by: Option<String>, sort_direction: String, search: Option<String>, filters: Option<String>) -> Result<()> {
        let filters = if let Some(filters_json) = filters {
            Some(serde_json::from_str::<serde_json::Value>(&filters_json)?)
        } else {
            None
        };

        let filters = if let Some(filter_value) = filters {
            // TODO: Convert JSON filters to application filters
            Some(SystemSettingsFilters::new().with_search(search.unwrap_or_default()))
        } else if search.is_some() {
            Some(SystemSettingsFilters::new().with_search(search.unwrap_or_default()))
        } else {
            None
        };

        let query = ListSystemSettingsQuery {
            page,
            page_size,
            sort_by,
            sort_direction,
            filters,
        };

        let response = self.services.list_system_settingses_handler().handle(query).await?;

        if response.success {
            println!("{}", format!("📄 Found {} system_settingses (page {}/{}):",
                response.system_settingses.len(), response.page, response.total_pages).cyan());

            for system_settings in &response.system_settingses {
                self.display_system_settings_compact(system_settings)?;
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

    async fn update_system_settings(&self, id: &str, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let system_settings_data = self.parse_system_settings_data(data, file)?;

        let command = UpdateSystemSettingsCommand {
            id: id.to_string(),
            // TODO: Map parsed data to command fields
            custom_fields: system_settings_data,
            updated_by: user_id,
        };

        let response = self.services.update_system_settings_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ SystemSettings updated successfully!".green());
            if let Some(system_settings) = response.system_settings {
                self.display_system_settings(&system_settings)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn delete_system_settings(&self, id: &str, user_id: String) -> Result<()> {
        let command = DeleteSystemSettingsCommand {
            id: id.to_string(),
            deleted_by: user_id,
        };

        let response = self.services.delete_system_settings_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ SystemSettings deleted successfully!".green());
            println!("{}", "💡 Use 'restore' command to recover if needed".blue());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn bulk_create_system_settingses(&self, file: Option<String>, user_id: String) -> Result<()> {
        let system_settingses_data = self.parse_bulk_system_settingses_data(file)?;

        let commands: Vec<CreateSystemSettingsCommand> = system_settingses_data
            .into_iter()
            .map(|data| CreateSystemSettingsCommand {
                custom_fields: data,
                created_by: user_id.clone(),
            })
            .collect();

        let command = BulkCreateSystemSettingsCommand {
            system_settingses: commands,
        };

        let response = self.services.bulk_create_system_settingses_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ {} system_settingses created successfully!", response.created_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn upsert_system_settingses(&self, file: Option<String>, user_id: String) -> Result<()> {
        let system_settingses_data = self.parse_bulk_system_settingses_data(file)?;

        let command = UpsertSystemSettingsCommand {
            // TODO: Map parsed data to command fields
            custom_fields: system_settingses_data,
            user_id,
        };

        let response = self.services.upsert_system_settings_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Upsert completed! Created: {}, Updated: {}", response.created, response.updated).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_trash(&self, page: usize, page_size: usize) -> Result<()> {
        let query = ListDeletedSystemSettingsQuery {
            page,
            page_size,
        };

        let response = self.services.list_deleted_system_settingses_handler().handle(query).await?;

        println!("{}", "🗑️  Deleted system_settingses (trash):".yellow());

        if response.success {
            if response.system_settingses.is_empty() {
                println!("{}", "  No deleted items in trash".green());
            } else {
                println!("{}", format!("  Found {} deleted items (page {}/{}):",
                    response.system_settingses.len(), response.page, response.total_pages).cyan());

                for system_settings in &response.system_settingses {
                    self.display_system_settings_compact(system_settings)?;
                }
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn restore_system_settings(&self, id: &str, user_id: String) -> Result<()> {
        let command = RestoreSystemSettingsCommand {
            id: id.to_string(),
            restored_by: user_id,
        };

        let response = self.services.restore_system_settings_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ SystemSettings restored successfully!".green());
            if let Some(system_settings) = response.system_settings {
                self.display_system_settings(&system_settings)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn empty_trash(&self, confirm: bool, user_id: String) -> Result<()> {
        if !confirm {
            println!("{}", "⚠️  This action will permanently delete all deleted system_settingses!".yellow());
            println!("{}", "Use --confirm to proceed".yellow());
            return Ok(());
        }

        let command = EmptyTrashCommand {
            user_id,
        };

        let response = self.services.empty_system_settings_trash_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Trash emptied! {} system_settingses permanently deleted.", response.deleted_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    // Helper methods
    fn parse_system_settings_data(&self, data: Option<String>, file: Option<String>) -> Result<std::collections::HashMap<String, serde_json::Value>> {
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

    fn parse_bulk_system_settingses_data(&self, file: Option<String>) -> Result<Vec<std::collections::HashMap<String, serde_json::Value>>> {
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

    fn display_system_settings(&self, system_settings: &SystemSettingsDto) -> Result<()> {
        println!("{}", format!("📋 SystemSettings Details:").cyan());
        println!("{}", format!("  ID: {}", system_settings.id).white());
        // TODO: Add more field display based on your system_settings structure
        println!("{}", format!("  Created: {:?}", system_settings.created_at).white());
        println!("{}", format!("  Updated: {:?}", system_settings.updated_at).white());
        Ok(())
    }

    fn display_system_settings_compact(&self, system_settings: &SystemSettingsDto) -> Result<()> {
        println!("{}", format!("🔹 {} | {:?}", system_settings.id, system_settings.created_at).white());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_settings_commands_parsing() {
        // TODO: Add CLI command parsing tests
    }
}
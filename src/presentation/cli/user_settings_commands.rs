// UserSettings CLI Commands
// Command-line interface for UserSettings CRUD operations

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

use crate::application::{
    commands::{
        CreateUserSettingsCommand, UpdateUserSettingsCommand, DeleteUserSettingsCommand,
        BulkCreateUserSettingsCommand, UpsertUserSettingsCommand, RestoreUserSettingsCommand,
        EmptyTrashCommand, UserSettingsDto, UserSettingsFilters,
    },
    queries::{
        GetUserSettingsQuery, ListUserSettingsQuery, ListDeletedUserSettingsQuery,
    },
    user_settings_services::UserSettingsApplicationServices,
};

/// UserSettings management commands
#[derive(Parser)]
#[command(name = "user_settings")]
#[command(about = "Manage user_settingses in the system")]
pub struct UserSettingsCommands {
    #[command(subcommand)]
    pub action: UserSettingsAction,
}

#[derive(Subcommand)]
pub enum UserSettingsAction {
    /// Create a new user_settings
    Create {
        /// UserSettings data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// UserSettings data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Get a user_settings by ID
    Get {
        /// UserSettings ID
        id: String,
    },
    /// List user_settingses with optional filtering
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
    /// Update an existing user_settings
    Update {
        /// UserSettings ID
        id: String,
        /// UserSettings data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// UserSettings data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Delete a user_settings (soft delete)
    Delete {
        /// UserSettings ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Bulk create user_settingses from file or stdin
    BulkCreate {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Upsert user_settingses (update or insert)
    Upsert {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// List deleted user_settingses (trash)
    ListTrash {
        /// Page number (default: 1)
        #[arg(short, long, default_value = "1")]
        page: usize,
        /// Page size (default: 20)
        #[arg(short, long, default_value = "20")]
        page_size: usize,
    },
    /// Restore a deleted user_settings
    Restore {
        /// UserSettings ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Empty trash (permanently delete all user_settingses)
    EmptyTrash {
        /// Confirmation flag
        #[arg(long)]
        confirm: bool,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
}

pub struct UserSettingsCliHandler {
    services: UserSettingsApplicationServices,
}

impl UserSettingsCliHandler {
    pub fn new(services: UserSettingsApplicationServices) -> Self {
        Self { services }
    }

    pub async fn handle(&self, commands: UserSettingsCommands) -> Result<()> {
        match commands.action {
            UserSettingsAction::Create { data, file, user_id } => {
                self.create_user_settings(data, file, user_id).await
            }
            UserSettingsAction::Get { id } => {
                self.get_user_settings(&id).await
            }
            UserSettingsAction::List { page, page_size, sort_by, sort_direction, search, filters } => {
                self.list_user_settingses(page, page_size, sort_by, sort_direction, search, filters).await
            }
            UserSettingsAction::Update { id, data, file, user_id } => {
                self.update_user_settings(&id, data, file, user_id).await
            }
            UserSettingsAction::Delete { id, user_id } => {
                self.delete_user_settings(&id, user_id).await
            }
            UserSettingsAction::BulkCreate { file, user_id } => {
                self.bulk_create_user_settingses(file, user_id).await
            }
            UserSettingsAction::Upsert { file, user_id } => {
                self.upsert_user_settingses(file, user_id).await
            }
            UserSettingsAction::ListTrash { page, page_size } => {
                self.list_trash(page, page_size).await
            }
            UserSettingsAction::Restore { id, user_id } => {
                self.restore_user_settings(&id, user_id).await
            }
            UserSettingsAction::EmptyTrash { confirm, user_id } => {
                self.empty_trash(confirm, user_id).await
            }
        }
    }

    async fn create_user_settings(&self, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let user_settings_data = self.parse_user_settings_data(data, file)?;

        let command = CreateUserSettingsCommand {
            // TODO: Map parsed data to command fields
            custom_fields: user_settings_data,
            created_by: user_id,
        };

        let response = self.services.create_user_settings_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ UserSettings created successfully!".green());
            if let Some(user_settings) = response.user_settings {
                self.display_user_settings(&user_settings)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn get_user_settings(&self, id: &str) -> Result<()> {
        let query = GetUserSettingsQuery {
            id: id.to_string(),
        };

        let response = self.services.get_user_settings_handler().handle(query).await?;

        if response.success {
            if let Some(user_settings) = response.user_settings {
                self.display_user_settings(&user_settings)?;
            } else {
                println!("{}", "UserSettings not found".yellow());
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_user_settingses(&self, page: usize, page_size: usize, sort_by: Option<String>, sort_direction: String, search: Option<String>, filters: Option<String>) -> Result<()> {
        let filters = if let Some(filters_json) = filters {
            Some(serde_json::from_str::<serde_json::Value>(&filters_json)?)
        } else {
            None
        };

        let filters = if let Some(filter_value) = filters {
            // TODO: Convert JSON filters to application filters
            Some(UserSettingsFilters::new().with_search(search.unwrap_or_default()))
        } else if search.is_some() {
            Some(UserSettingsFilters::new().with_search(search.unwrap_or_default()))
        } else {
            None
        };

        let query = ListUserSettingsQuery {
            page,
            page_size,
            sort_by,
            sort_direction,
            filters,
        };

        let response = self.services.list_user_settingses_handler().handle(query).await?;

        if response.success {
            println!("{}", format!("📄 Found {} user_settingses (page {}/{}):",
                response.user_settingses.len(), response.page, response.total_pages).cyan());

            for user_settings in &response.user_settingses {
                self.display_user_settings_compact(user_settings)?;
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

    async fn update_user_settings(&self, id: &str, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let user_settings_data = self.parse_user_settings_data(data, file)?;

        let command = UpdateUserSettingsCommand {
            id: id.to_string(),
            // TODO: Map parsed data to command fields
            custom_fields: user_settings_data,
            updated_by: user_id,
        };

        let response = self.services.update_user_settings_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ UserSettings updated successfully!".green());
            if let Some(user_settings) = response.user_settings {
                self.display_user_settings(&user_settings)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn delete_user_settings(&self, id: &str, user_id: String) -> Result<()> {
        let command = DeleteUserSettingsCommand {
            id: id.to_string(),
            deleted_by: user_id,
        };

        let response = self.services.delete_user_settings_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ UserSettings deleted successfully!".green());
            println!("{}", "💡 Use 'restore' command to recover if needed".blue());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn bulk_create_user_settingses(&self, file: Option<String>, user_id: String) -> Result<()> {
        let user_settingses_data = self.parse_bulk_user_settingses_data(file)?;

        let commands: Vec<CreateUserSettingsCommand> = user_settingses_data
            .into_iter()
            .map(|data| CreateUserSettingsCommand {
                custom_fields: data,
                created_by: user_id.clone(),
            })
            .collect();

        let command = BulkCreateUserSettingsCommand {
            user_settingses: commands,
        };

        let response = self.services.bulk_create_user_settingses_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ {} user_settingses created successfully!", response.created_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn upsert_user_settingses(&self, file: Option<String>, user_id: String) -> Result<()> {
        let user_settingses_data = self.parse_bulk_user_settingses_data(file)?;

        let command = UpsertUserSettingsCommand {
            // TODO: Map parsed data to command fields
            custom_fields: user_settingses_data,
            user_id,
        };

        let response = self.services.upsert_user_settings_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Upsert completed! Created: {}, Updated: {}", response.created, response.updated).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_trash(&self, page: usize, page_size: usize) -> Result<()> {
        // This would use ListDeletedUserSettingsQuery
        println!("{}", "🗑️  Deleted user_settingses (trash):".yellow());
        println!("{}", "Feature not yet implemented - TODO: Add ListDeletedUserSettingsQuery handler".yellow());
        Ok(())
    }

    async fn restore_user_settings(&self, id: &str, user_id: String) -> Result<()> {
        let command = RestoreUserSettingsCommand {
            id: id.to_string(),
            restored_by: user_id,
        };

        let response = self.services.restore_user_settings_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ UserSettings restored successfully!".green());
            if let Some(user_settings) = response.user_settings {
                self.display_user_settings(&user_settings)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn empty_trash(&self, confirm: bool, user_id: String) -> Result<()> {
        if !confirm {
            println!("{}", "⚠️  This action will permanently delete all deleted user_settingses!".yellow());
            println!("{}", "Use --confirm to proceed".yellow());
            return Ok(());
        }

        let command = EmptyTrashCommand {
            user_id,
        };

        let response = self.services.empty_user_settings_trash_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Trash emptied! {} user_settingses permanently deleted.", response.deleted_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    // Helper methods
    fn parse_user_settings_data(&self, data: Option<String>, file: Option<String>) -> Result<std::collections::HashMap<String, serde_json::Value>> {
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

    fn parse_bulk_user_settingses_data(&self, file: Option<String>) -> Result<Vec<std::collections::HashMap<String, serde_json::Value>>> {
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

    fn display_user_settings(&self, user_settings: &UserSettingsDto) -> Result<()> {
        println!("{}", format!("📋 UserSettings Details:").cyan());
        println!("{}", format!("  ID: {}", user_settings.id).white());
        // TODO: Add more field display based on your user_settings structure
        println!("{}", format!("  Created: {:?}", user_settings.created_at).white());
        println!("{}", format!("  Updated: {:?}", user_settings.updated_at).white());
        Ok(())
    }

    fn display_user_settings_compact(&self, user_settings: &UserSettingsDto) -> Result<()> {
        println!("{}", format!("🔹 {} | {:?}", user_settings.id, user_settings.created_at).white());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_settings_commands_parsing() {
        // TODO: Add CLI command parsing tests
    }
}
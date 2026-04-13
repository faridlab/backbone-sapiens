// PasswordResetToken CLI Commands
// Command-line interface for PasswordResetToken CRUD operations

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::application::{
    commands::{
        CreatePasswordResetTokenCommand, UpdatePasswordResetTokenCommand, DeletePasswordResetTokenCommand,
        BulkCreatePasswordResetTokenCommand, UpsertPasswordResetTokenCommand, RestorePasswordResetTokenCommand,
        EmptyTrashCommand, PasswordResetTokenDto, PasswordResetTokenFilters,
    },
    queries::{
        GetPasswordResetTokenQuery, ListPasswordResetTokenQuery, ListDeletedPasswordResetTokenQuery,
    },
    password_reset_token_services::PasswordResetTokenApplicationServices,
};

/// PasswordResetToken management commands
#[derive(Parser)]
#[command(name = "password_reset_token")]
#[command(about = "Manage password_reset_tokens in the system")]
pub struct PasswordResetTokenCommands {
    #[command(subcommand)]
    pub action: PasswordResetTokenAction,
}

#[derive(Subcommand)]
pub enum PasswordResetTokenAction {
    /// Create a new password_reset_token
    Create {
        /// PasswordResetToken data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// PasswordResetToken data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Get a password_reset_token by ID
    Get {
        /// PasswordResetToken ID
        id: String,
    },
    /// List password_reset_tokens with optional filtering
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
    /// Update an existing password_reset_token
    Update {
        /// PasswordResetToken ID
        id: String,
        /// PasswordResetToken data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// PasswordResetToken data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Delete a password_reset_token (soft delete)
    Delete {
        /// PasswordResetToken ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Bulk create password_reset_tokens from file or stdin
    BulkCreate {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Upsert password_reset_tokens (update or insert)
    Upsert {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// List deleted password_reset_tokens (trash)
    ListTrash {
        /// Page number (default: 1)
        #[arg(short, long, default_value = "1")]
        page: usize,
        /// Page size (default: 20)
        #[arg(short, long, default_value = "20")]
        page_size: usize,
    },
    /// Restore a deleted password_reset_token
    Restore {
        /// PasswordResetToken ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Empty trash (permanently delete all password_reset_tokens)
    EmptyTrash {
        /// Confirmation flag
        #[arg(long)]
        confirm: bool,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
}

pub struct PasswordResetTokenCliHandler {
    services: PasswordResetTokenApplicationServices,
}

impl PasswordResetTokenCliHandler {
    pub fn new(services: PasswordResetTokenApplicationServices) -> Self {
        Self { services }
    }

    pub async fn handle(&self, commands: PasswordResetTokenCommands) -> Result<()> {
        match commands.action {
            PasswordResetTokenAction::Create { data, file, user_id } => {
                self.create_password_reset_token(data, file, user_id).await
            }
            PasswordResetTokenAction::Get { id } => {
                self.get_password_reset_token(&id).await
            }
            PasswordResetTokenAction::List { page, page_size, sort_by, sort_direction, search, filters } => {
                self.list_password_reset_tokens(page, page_size, sort_by, sort_direction, search, filters).await
            }
            PasswordResetTokenAction::Update { id, data, file, user_id } => {
                self.update_password_reset_token(&id, data, file, user_id).await
            }
            PasswordResetTokenAction::Delete { id, user_id } => {
                self.delete_password_reset_token(&id, user_id).await
            }
            PasswordResetTokenAction::BulkCreate { file, user_id } => {
                self.bulk_create_password_reset_tokens(file, user_id).await
            }
            PasswordResetTokenAction::Upsert { file, user_id } => {
                self.upsert_password_reset_tokens(file, user_id).await
            }
            PasswordResetTokenAction::ListTrash { page, page_size } => {
                self.list_trash(page, page_size).await
            }
            PasswordResetTokenAction::Restore { id, user_id } => {
                self.restore_password_reset_token(&id, user_id).await
            }
            PasswordResetTokenAction::EmptyTrash { confirm, user_id } => {
                self.empty_trash(confirm, user_id).await
            }
        }
    }

    async fn create_password_reset_token(&self, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let password_reset_token_data = self.parse_password_reset_token_data(data, file)?;

        let command = CreatePasswordResetTokenCommand {
            user_id: password_reset_token_data.get("user_id")
                .and_then(|v| Uuid::parse_str(v).ok())
                .unwrap_or_else(|| Uuid::new_v4()),
            token_hash: password_reset_token_data.get("token_hash")
                .and_then(|v| v.as_str())
                .unwrap_or("default_token_hash").to_string(),
            expires_at: password_reset_token_data.get("expires_at")
                .and_then(|v| v.as_str())
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .unwrap_or_else(|| Utc::now() + chrono::Duration::hours(24)),
            is_used: password_reset_token_data.get("is_used")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            used_at: password_reset_token_data.get("used_at")
                .and_then(|v| v.as_str())
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok()),
            ip_address: password_reset_token_data.get("ip_address")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            metadata: password_reset_token_data.get("metadata")
                .cloned()
                .unwrap_or(serde_json::Value::Object(Default::default()).into()),
        };

        let response = self.services.create_password_reset_token_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ PasswordResetToken created successfully!".green());
            if let Some(password_reset_token) = response.password_reset_token {
                self.display_password_reset_token(&password_reset_token)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn get_password_reset_token(&self, id: &str) -> Result<()> {
        let query = GetPasswordResetTokenQuery {
            id: id.to_string(),
        };

        let response = self.services.get_password_reset_token_handler().handle(query).await?;

        if response.success {
            if let Some(password_reset_token) = response.password_reset_token {
                self.display_password_reset_token(&password_reset_token)?;
            } else {
                println!("{}", "PasswordResetToken not found".yellow());
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_password_reset_tokens(&self, page: usize, page_size: usize, sort_by: Option<String>, sort_direction: String, search: Option<String>, filters: Option<String>) -> Result<()> {
        let filters = if let Some(filters_json) = filters {
            Some(serde_json::from_str::<serde_json::Value>(&filters_json)?)
        } else {
            None
        };

        let filters = if let Some(filter_value) = filters {
            // Convert JSON filters to application filters
            let mut app_filters = PasswordResetTokenFilters::new();
            if let Some(search_term) = search {
                app_filters = app_filters.with_search(search_term);
            }
            
            // Apply additional filters from JSON
            if let Some(obj) = filter_value.as_object() {
                for (key, value) in obj {
                    match key.as_str() {
                        Some("user_id") => {
                            if let Some(user_id_str) = value.as_str() {
                                if let Ok(user_id) = Uuid::parse_str(user_id_str) {
                                    app_filters = app_filters.with_user_id(user_id);
                                }
                            }
                        }
                        Some("is_used") => {
                            if let Some(is_used) = value.as_bool() {
                                app_filters = app_filters.with_is_used(is_used);
                            }
                        }
                        Some("expires_before") => {
                            if let Some(date_str) = value.as_str() {
                                if let Ok(date) = DateTime::parse_from_rfc3339(date_str) {
                                    app_filters = app_filters.with_expires_before(date);
                                }
                            }
                        }
                        _ => {} // Ignore unknown filter keys
                    }
                }
            }
            Some(app_filters)
        } else if search.is_some() {
            Some(PasswordResetTokenFilters::new().with_search(search.unwrap_or_default()))
        } else {
            None
        };

        let query = ListPasswordResetTokenQuery {
            page,
            page_size,
            sort_by,
            sort_direction,
            filters,
        };

        let response = self.services.list_password_reset_tokens_handler().handle(query).await?;

        if response.success {
            println!("{}", format!("📄 Found {} password_reset_tokens (page {}/{}):",
                response.password_reset_tokens.len(), response.page, response.total_pages).cyan());

            for password_reset_token in &response.password_reset_tokens {
                self.display_password_reset_token_compact(password_reset_token)?;
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

    async fn update_password_reset_token(&self, id: &str, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let password_reset_token_data = self.parse_password_reset_token_data(data, file)?;

        let command = UpdatePasswordResetTokenCommand {
            id: id.to_string(),
            // TODO: Map parsed data to command fields
            custom_fields: password_reset_token_data,
            updated_by: user_id,
        };

        let response = self.services.update_password_reset_token_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ PasswordResetToken updated successfully!".green());
            if let Some(password_reset_token) = response.password_reset_token {
                self.display_password_reset_token(&password_reset_token)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn delete_password_reset_token(&self, id: &str, user_id: String) -> Result<()> {
        let command = DeletePasswordResetTokenCommand {
            id: id.to_string(),
            deleted_by: user_id,
        };

        let response = self.services.delete_password_reset_token_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ PasswordResetToken deleted successfully!".green());
            println!("{}", "💡 Use 'restore' command to recover if needed".blue());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn bulk_create_password_reset_tokens(&self, file: Option<String>, user_id: String) -> Result<()> {
        let password_reset_tokens_data = self.parse_bulk_password_reset_tokens_data(file)?;

        let commands: Vec<CreatePasswordResetTokenCommand> = password_reset_tokens_data
            .into_iter()
            .map(|data| CreatePasswordResetTokenCommand {
                custom_fields: data,
                created_by: user_id.clone(),
            })
            .collect();

        let command = BulkCreatePasswordResetTokenCommand {
            password_reset_tokens: commands,
        };

        let response = self.services.bulk_create_password_reset_tokens_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ {} password_reset_tokens created successfully!", response.created_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn upsert_password_reset_tokens(&self, file: Option<String>, user_id: String) -> Result<()> {
        let password_reset_tokens_data = self.parse_bulk_password_reset_tokens_data(file)?;

        let command = UpsertPasswordResetTokenCommand {
            // TODO: Map parsed data to command fields
            custom_fields: password_reset_tokens_data,
            user_id,
        };

        let response = self.services.upsert_password_reset_token_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Upsert completed! Created: {}, Updated: {}", response.created, response.updated).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_trash(&self, page: usize, page_size: usize) -> Result<()> {
        // This would use ListDeletedPasswordResetTokenQuery
        println!("{}", "🗑️  Deleted password_reset_tokens (trash):".yellow());
        println!("{}", "Feature not yet implemented - TODO: Add ListDeletedPasswordResetTokenQuery handler".yellow());
        Ok(())
    }

    async fn restore_password_reset_token(&self, id: &str, user_id: String) -> Result<()> {
        let command = RestorePasswordResetTokenCommand {
            id: id.to_string(),
            restored_by: user_id,
        };

        let response = self.services.restore_password_reset_token_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ PasswordResetToken restored successfully!".green());
            if let Some(password_reset_token) = response.password_reset_token {
                self.display_password_reset_token(&password_reset_token)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn empty_trash(&self, confirm: bool, user_id: String) -> Result<()> {
        if !confirm {
            println!("{}", "⚠️  This action will permanently delete all deleted password_reset_tokens!".yellow());
            println!("{}", "Use --confirm to proceed".yellow());
            return Ok(());
        }

        let command = EmptyTrashCommand {
            user_id,
        };

        let response = self.services.empty_password_reset_token_trash_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Trash emptied! {} password_reset_tokens permanently deleted.", response.deleted_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    // Helper methods
    fn parse_password_reset_token_data(&self, data: Option<String>, file: Option<String>) -> Result<std::collections::HashMap<String, serde_json::Value>> {
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

    fn parse_bulk_password_reset_tokens_data(&self, file: Option<String>) -> Result<Vec<std::collections::HashMap<String, serde_json::Value>>> {
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

    fn display_password_reset_token(&self, password_reset_token: &PasswordResetTokenDto) -> Result<()> {
        println!("{}", format!("📋 PasswordResetToken Details:").cyan());
        println!("{}", format!("  ID: {}", password_reset_token.id).white());
        // TODO: Add more field display based on your password_reset_token structure
        println!("{}", format!("  Created: {:?}", password_reset_token.created_at).white());
        println!("{}", format!("  Updated: {:?}", password_reset_token.updated_at).white());
        Ok(())
    }

    fn display_password_reset_token_compact(&self, password_reset_token: &PasswordResetTokenDto) -> Result<()> {
        println!("{}", format!("🔹 {} | {:?}", password_reset_token.id, password_reset_token.created_at).white());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_reset_token_commands_parsing() {
        // TODO: Add CLI command parsing tests
    }
}
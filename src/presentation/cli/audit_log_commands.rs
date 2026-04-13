// AuditLog CLI Commands
// Command-line interface for AuditLog CRUD operations

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

use crate::application::{
    commands::{
        CreateAuditLogCommand, UpdateAuditLogCommand, DeleteAuditLogCommand,
        BulkCreateAuditLogCommand, UpsertAuditLogCommand, RestoreAuditLogCommand,
        EmptyTrashCommand, AuditLogDto, AuditLogFilters,
    },
    queries::{
        GetAuditLogQuery, ListAuditLogQuery, ListDeletedAuditLogQuery,
    },
    audit_log_services::AuditLogApplicationServices,
};

/// AuditLog management commands
#[derive(Parser)]
#[command(name = "audit_log")]
#[command(about = "Manage audit_logs in the system")]
pub struct AuditLogCommands {
    #[command(subcommand)]
    pub action: AuditLogAction,
}

#[derive(Subcommand)]
pub enum AuditLogAction {
    /// Create a new audit_log
    Create {
        /// AuditLog data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// AuditLog data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Get a audit_log by ID
    Get {
        /// AuditLog ID
        id: String,
    },
    /// List audit_logs with optional filtering
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
    /// Update an existing audit_log
    Update {
        /// AuditLog ID
        id: String,
        /// AuditLog data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// AuditLog data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Delete a audit_log (soft delete)
    Delete {
        /// AuditLog ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Bulk create audit_logs from file or stdin
    BulkCreate {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Upsert audit_logs (update or insert)
    Upsert {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// List deleted audit_logs (trash)
    ListTrash {
        /// Page number (default: 1)
        #[arg(short, long, default_value = "1")]
        page: usize,
        /// Page size (default: 20)
        #[arg(short, long, default_value = "20")]
        page_size: usize,
    },
    /// Restore a deleted audit_log
    Restore {
        /// AuditLog ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Empty trash (permanently delete all audit_logs)
    EmptyTrash {
        /// Confirmation flag
        #[arg(long)]
        confirm: bool,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
}

pub struct AuditLogCliHandler {
    services: AuditLogApplicationServices,
}

impl AuditLogCliHandler {
    pub fn new(services: AuditLogApplicationServices) -> Self {
        Self { services }
    }

    pub async fn handle(&self, commands: AuditLogCommands) -> Result<()> {
        match commands.action {
            AuditLogAction::Create { data, file, user_id } => {
                self.create_audit_log(data, file, user_id).await
            }
            AuditLogAction::Get { id } => {
                self.get_audit_log(&id).await
            }
            AuditLogAction::List { page, page_size, sort_by, sort_direction, search, filters } => {
                self.list_audit_logs(page, page_size, sort_by, sort_direction, search, filters).await
            }
            AuditLogAction::Update { id, data, file, user_id } => {
                self.update_audit_log(&id, data, file, user_id).await
            }
            AuditLogAction::Delete { id, user_id } => {
                self.delete_audit_log(&id, user_id).await
            }
            AuditLogAction::BulkCreate { file, user_id } => {
                self.bulk_create_audit_logs(file, user_id).await
            }
            AuditLogAction::Upsert { file, user_id } => {
                self.upsert_audit_logs(file, user_id).await
            }
            AuditLogAction::ListTrash { page, page_size } => {
                self.list_trash(page, page_size).await
            }
            AuditLogAction::Restore { id, user_id } => {
                self.restore_audit_log(&id, user_id).await
            }
            AuditLogAction::EmptyTrash { confirm, user_id } => {
                self.empty_trash(confirm, user_id).await
            }
        }
    }

    async fn create_audit_log(&self, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let audit_log_data = self.parse_audit_log_data(data, file)?;

        let command = CreateAuditLogCommand {
            // TODO: Map parsed data to command fields
            custom_fields: audit_log_data,
            created_by: user_id,
        };

        let response = self.services.create_audit_log_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ AuditLog created successfully!".green());
            if let Some(audit_log) = response.audit_log {
                self.display_audit_log(&audit_log)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn get_audit_log(&self, id: &str) -> Result<()> {
        let query = GetAuditLogQuery {
            id: id.to_string(),
        };

        let response = self.services.get_audit_log_handler().handle(query).await?;

        if response.success {
            if let Some(audit_log) = response.audit_log {
                self.display_audit_log(&audit_log)?;
            } else {
                println!("{}", "AuditLog not found".yellow());
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_audit_logs(&self, page: usize, page_size: usize, sort_by: Option<String>, sort_direction: String, search: Option<String>, filters: Option<String>) -> Result<()> {
        let filters = if let Some(filters_json) = filters {
            Some(serde_json::from_str::<serde_json::Value>(&filters_json)?)
        } else {
            None
        };

        let filters = if let Some(filter_value) = filters {
            // TODO: Convert JSON filters to application filters
            Some(AuditLogFilters::new().with_search(search.unwrap_or_default()))
        } else if search.is_some() {
            Some(AuditLogFilters::new().with_search(search.unwrap_or_default()))
        } else {
            None
        };

        let query = ListAuditLogQuery {
            page,
            page_size,
            sort_by,
            sort_direction,
            filters,
        };

        let response = self.services.list_audit_logs_handler().handle(query).await?;

        if response.success {
            println!("{}", format!("📄 Found {} audit_logs (page {}/{}):",
                response.audit_logs.len(), response.page, response.total_pages).cyan());

            for audit_log in &response.audit_logs {
                self.display_audit_log_compact(audit_log)?;
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

    async fn update_audit_log(&self, id: &str, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let audit_log_data = self.parse_audit_log_data(data, file)?;

        let command = UpdateAuditLogCommand {
            id: id.to_string(),
            // TODO: Map parsed data to command fields
            custom_fields: audit_log_data,
            updated_by: user_id,
        };

        let response = self.services.update_audit_log_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ AuditLog updated successfully!".green());
            if let Some(audit_log) = response.audit_log {
                self.display_audit_log(&audit_log)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn delete_audit_log(&self, id: &str, user_id: String) -> Result<()> {
        let command = DeleteAuditLogCommand {
            id: id.to_string(),
            deleted_by: user_id,
        };

        let response = self.services.delete_audit_log_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ AuditLog deleted successfully!".green());
            println!("{}", "💡 Use 'restore' command to recover if needed".blue());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn bulk_create_audit_logs(&self, file: Option<String>, user_id: String) -> Result<()> {
        let audit_logs_data = self.parse_bulk_audit_logs_data(file)?;

        let commands: Vec<CreateAuditLogCommand> = audit_logs_data
            .into_iter()
            .map(|data| CreateAuditLogCommand {
                custom_fields: data,
                created_by: user_id.clone(),
            })
            .collect();

        let command = BulkCreateAuditLogCommand {
            audit_logs: commands,
        };

        let response = self.services.bulk_create_audit_logs_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ {} audit_logs created successfully!", response.created_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn upsert_audit_logs(&self, file: Option<String>, user_id: String) -> Result<()> {
        let audit_logs_data = self.parse_bulk_audit_logs_data(file)?;

        let command = UpsertAuditLogCommand {
            // TODO: Map parsed data to command fields
            custom_fields: audit_logs_data,
            user_id,
        };

        let response = self.services.upsert_audit_log_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Upsert completed! Created: {}, Updated: {}", response.created, response.updated).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_trash(&self, page: usize, page_size: usize) -> Result<()> {
        // This would use ListDeletedAuditLogQuery
        println!("{}", "🗑️  Deleted audit_logs (trash):".yellow());
        println!("{}", "Feature not yet implemented - TODO: Add ListDeletedAuditLogQuery handler".yellow());
        Ok(())
    }

    async fn restore_audit_log(&self, id: &str, user_id: String) -> Result<()> {
        let command = RestoreAuditLogCommand {
            id: id.to_string(),
            restored_by: user_id,
        };

        let response = self.services.restore_audit_log_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ AuditLog restored successfully!".green());
            if let Some(audit_log) = response.audit_log {
                self.display_audit_log(&audit_log)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn empty_trash(&self, confirm: bool, user_id: String) -> Result<()> {
        if !confirm {
            println!("{}", "⚠️  This action will permanently delete all deleted audit_logs!".yellow());
            println!("{}", "Use --confirm to proceed".yellow());
            return Ok(());
        }

        let command = EmptyTrashCommand {
            user_id,
        };

        let response = self.services.empty_audit_log_trash_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Trash emptied! {} audit_logs permanently deleted.", response.deleted_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    // Helper methods
    fn parse_audit_log_data(&self, data: Option<String>, file: Option<String>) -> Result<std::collections::HashMap<String, serde_json::Value>> {
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

    fn parse_bulk_audit_logs_data(&self, file: Option<String>) -> Result<Vec<std::collections::HashMap<String, serde_json::Value>>> {
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

    fn display_audit_log(&self, audit_log: &AuditLogDto) -> Result<()> {
        println!("{}", format!("📋 AuditLog Details:").cyan());
        println!("{}", format!("  ID: {}", audit_log.id).white());
        // TODO: Add more field display based on your audit_log structure
        println!("{}", format!("  Created: {:?}", audit_log.created_at).white());
        println!("{}", format!("  Updated: {:?}", audit_log.updated_at).white());
        Ok(())
    }

    fn display_audit_log_compact(&self, audit_log: &AuditLogDto) -> Result<()> {
        println!("{}", format!("🔹 {} | {:?}", audit_log.id, audit_log.created_at).white());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_log_commands_parsing() {
        // TODO: Add CLI command parsing tests
    }
}
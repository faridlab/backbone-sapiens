// Session CLI Commands
// Command-line interface for Session CRUD operations

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

use crate::application::{
    commands::{
        CreateSessionCommand, UpdateSessionCommand, DeleteSessionCommand,
        BulkCreateSessionCommand, UpsertSessionCommand, RestoreSessionCommand,
        EmptyTrashCommand, SessionDto, SessionFilters,
    },
    queries::{
        GetSessionQuery, ListSessionQuery, ListDeletedSessionQuery,
    },
    session_services::SessionApplicationServices,
};

/// Session management commands
#[derive(Parser)]
#[command(name = "session")]
#[command(about = "Manage sessions in the system")]
pub struct SessionCommands {
    #[command(subcommand)]
    pub action: SessionAction,
}

#[derive(Subcommand)]
pub enum SessionAction {
    /// Create a new session
    Create {
        /// Session data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// Session data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Get a session by ID
    Get {
        /// Session ID
        id: String,
    },
    /// List sessions with optional filtering
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
    /// Update an existing session
    Update {
        /// Session ID
        id: String,
        /// Session data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// Session data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Delete a session (soft delete)
    Delete {
        /// Session ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Bulk create sessions from file or stdin
    BulkCreate {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Upsert sessions (update or insert)
    Upsert {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// List deleted sessions (trash)
    ListTrash {
        /// Page number (default: 1)
        #[arg(short, long, default_value = "1")]
        page: usize,
        /// Page size (default: 20)
        #[arg(short, long, default_value = "20")]
        page_size: usize,
    },
    /// Restore a deleted session
    Restore {
        /// Session ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Empty trash (permanently delete all sessions)
    EmptyTrash {
        /// Confirmation flag
        #[arg(long)]
        confirm: bool,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
}

pub struct SessionCliHandler {
    services: SessionApplicationServices,
}

impl SessionCliHandler {
    pub fn new(services: SessionApplicationServices) -> Self {
        Self { services }
    }

    pub async fn handle(&self, commands: SessionCommands) -> Result<()> {
        match commands.action {
            SessionAction::Create { data, file, user_id } => {
                self.create_session(data, file, user_id).await
            }
            SessionAction::Get { id } => {
                self.get_session(&id).await
            }
            SessionAction::List { page, page_size, sort_by, sort_direction, search, filters } => {
                self.list_sessions(page, page_size, sort_by, sort_direction, search, filters).await
            }
            SessionAction::Update { id, data, file, user_id } => {
                self.update_session(&id, data, file, user_id).await
            }
            SessionAction::Delete { id, user_id } => {
                self.delete_session(&id, user_id).await
            }
            SessionAction::BulkCreate { file, user_id } => {
                self.bulk_create_sessions(file, user_id).await
            }
            SessionAction::Upsert { file, user_id } => {
                self.upsert_sessions(file, user_id).await
            }
            SessionAction::ListTrash { page, page_size } => {
                self.list_trash(page, page_size).await
            }
            SessionAction::Restore { id, user_id } => {
                self.restore_session(&id, user_id).await
            }
            SessionAction::EmptyTrash { confirm, user_id } => {
                self.empty_trash(confirm, user_id).await
            }
        }
    }

    async fn create_session(&self, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let session_data = self.parse_session_data(data, file)?;

        let command = CreateSessionCommand {
            // TODO: Map parsed data to command fields
            custom_fields: session_data,
            created_by: user_id,
        };

        let response = self.services.create_session_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ Session created successfully!".green());
            if let Some(session) = response.session {
                self.display_session(&session)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn get_session(&self, id: &str) -> Result<()> {
        let query = GetSessionQuery {
            id: id.to_string(),
        };

        let response = self.services.get_session_handler().handle(query).await?;

        if response.success {
            if let Some(session) = response.session {
                self.display_session(&session)?;
            } else {
                println!("{}", "Session not found".yellow());
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_sessions(&self, page: usize, page_size: usize, sort_by: Option<String>, sort_direction: String, search: Option<String>, filters: Option<String>) -> Result<()> {
        let filters = if let Some(filters_json) = filters {
            Some(serde_json::from_str::<serde_json::Value>(&filters_json)?)
        } else {
            None
        };

        let filters = if let Some(filter_value) = filters {
            // TODO: Convert JSON filters to application filters
            Some(SessionFilters::new().with_search(search.unwrap_or_default()))
        } else if search.is_some() {
            Some(SessionFilters::new().with_search(search.unwrap_or_default()))
        } else {
            None
        };

        let query = ListSessionQuery {
            page,
            page_size,
            sort_by,
            sort_direction,
            filters,
        };

        let response = self.services.list_sessions_handler().handle(query).await?;

        if response.success {
            println!("{}", format!("📄 Found {} sessions (page {}/{}):",
                response.sessions.len(), response.page, response.total_pages).cyan());

            for session in &response.sessions {
                self.display_session_compact(session)?;
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

    async fn update_session(&self, id: &str, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let session_data = self.parse_session_data(data, file)?;

        let command = UpdateSessionCommand {
            id: id.to_string(),
            // TODO: Map parsed data to command fields
            custom_fields: session_data,
            updated_by: user_id,
        };

        let response = self.services.update_session_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ Session updated successfully!".green());
            if let Some(session) = response.session {
                self.display_session(&session)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn delete_session(&self, id: &str, user_id: String) -> Result<()> {
        let command = DeleteSessionCommand {
            id: id.to_string(),
            deleted_by: user_id,
        };

        let response = self.services.delete_session_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ Session deleted successfully!".green());
            println!("{}", "💡 Use 'restore' command to recover if needed".blue());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn bulk_create_sessions(&self, file: Option<String>, user_id: String) -> Result<()> {
        let sessions_data = self.parse_bulk_sessions_data(file)?;

        let commands: Vec<CreateSessionCommand> = sessions_data
            .into_iter()
            .map(|data| CreateSessionCommand {
                custom_fields: data,
                created_by: user_id.clone(),
            })
            .collect();

        let command = BulkCreateSessionCommand {
            sessions: commands,
        };

        let response = self.services.bulk_create_sessions_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ {} sessions created successfully!", response.created_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn upsert_sessions(&self, file: Option<String>, user_id: String) -> Result<()> {
        let sessions_data = self.parse_bulk_sessions_data(file)?;

        let command = UpsertSessionCommand {
            // TODO: Map parsed data to command fields
            custom_fields: sessions_data,
            user_id,
        };

        let response = self.services.upsert_session_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Upsert completed! Created: {}, Updated: {}", response.created, response.updated).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_trash(&self, page: usize, page_size: usize) -> Result<()> {
        println!("{}", "🗑️  Deleted sessions (trash):".yellow());

        let query = ListDeletedSessionQuery::new()
            .with_pagination(page, page_size)
            .with_sort("deleted_at".to_string(), "desc".to_string());

        // TODO: Add actual ListDeletedSessionQuery handler when implemented
        // let response = self.services.list_deleted_session_handler().handle(query).await?;

        // Temporary implementation - show placeholder message
        println!("📄 Page {} ({} per page)", page, page_size);
        println!("🔍 Sorted by: deleted_at (descending)");
        println!();
        println!("{}", "⚠️  ListDeletedSessionQuery handler not yet implemented".yellow());
        println!("💡 This will show deleted/soft-deleted sessions");
        println!("💡 Sessions can be restored using the 'restore' command");
        println!("💡 Use 'empty-trash' to permanently delete all sessions");
        println!();

        Ok(())
    }

    async fn restore_session(&self, id: &str, user_id: String) -> Result<()> {
        let command = RestoreSessionCommand {
            id: id.to_string(),
            restored_by: user_id,
        };

        let response = self.services.restore_session_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ Session restored successfully!".green());
            if let Some(session) = response.session {
                self.display_session(&session)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn empty_trash(&self, confirm: bool, user_id: String) -> Result<()> {
        if !confirm {
            println!("{}", "⚠️  This action will permanently delete all deleted sessions!".yellow());
            println!("{}", "Use --confirm to proceed".yellow());
            return Ok(());
        }

        let command = EmptyTrashCommand {
            user_id,
        };

        let response = self.services.empty_session_trash_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Trash emptied! {} sessions permanently deleted.", response.deleted_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    // Helper methods
    fn parse_session_data(&self, data: Option<String>, file: Option<String>) -> Result<std::collections::HashMap<String, serde_json::Value>> {
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

    fn parse_bulk_sessions_data(&self, file: Option<String>) -> Result<Vec<std::collections::HashMap<String, serde_json::Value>>> {
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

    fn display_session(&self, session: &SessionDto) -> Result<()> {
        println!("{}", "📋 Session Details:".cyan());
        println!("{}", format!("  ID: {}", session.id).white());

        // Display session status with color coding
        let status_indicator = if session.is_active { "🟢" } else { "🔴" };
        let status_text = if session.is_active { "Active" } else { "Inactive" };
        println!("{}", format!("  Status: {} {}", status_indicator, status_text).white());

        // Display timestamps
        if let Some(created_at) = &session.created_at {
            println!("{}", format!("  Created: {}", created_at).white());
        }
        if let Some(updated_at) = &session.updated_at {
            println!("{}", format!("  Updated: {}", updated_at).white());
        }

        // Display session expiry
        if let Some(expires_at) = &session.expires_at {
            println!("{}", format!("  Expires: {}", expires_at).white());

            // Calculate time until expiry
            let now = chrono::Utc::now();
            let time_until_expiry = expires_at.signed_duration_since(now);
            if time_until_expiry.num_seconds() > 0 {
                println!("{}", format!("  Time until expiry: {} minutes", time_until_expiry.num_minutes()).green());
            } else {
                println!("{}", "  Status: EXPIRED".red().bold());
            }
        }

        // Display last activity
        if let Some(last_activity) = &session.last_activity {
            println!("{}", format!("  Last activity: {}", last_activity).white());

            // Check if recently active
            let time_since_activity = now.signed_duration_since(*last_activity);
            let minutes_since_activity = time_since_activity.num_minutes();
            if minutes_since_activity <= 30 {
                println!("{}", format!("  Recently active: {} minutes ago", minutes_since_activity).green());
            } else {
                println!("{}", format!("  Inactive for: {} minutes", minutes_since_activity).yellow());
            }
        }

        // Display session metadata if available
        if !session.custom_fields.is_empty() {
            println!("{}", "  Metadata:".white());
            for (key, value) in &session.custom_fields {
                println!("{}", format!("    {}: {}", key, value).dimmed());
            }
        }

        Ok(())
    }

    fn display_session_compact(&self, session: &SessionDto) -> Result<()> {
        println!("{}", format!("🔹 {} | {:?}", session.id, session.created_at).white());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_commands_parsing() {
        // TODO: Add CLI command parsing tests
    }
}
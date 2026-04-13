// MfaDevice CLI Commands
// Command-line interface for MfaDevice CRUD operations

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

use crate::application::{
    commands::{
        CreateMfaDeviceCommand, UpdateMfaDeviceCommand, DeleteMfaDeviceCommand,
        BulkCreateMfaDeviceCommand, UpsertMfaDeviceCommand, RestoreMfaDeviceCommand,
        EmptyTrashCommand, MfaDeviceDto, MfaDeviceFilters,
    },
    queries::{
        GetMfaDeviceQuery, ListMfaDeviceQuery, ListDeletedMfaDeviceQuery,
    },
    mfa_device_services::MfaDeviceApplicationServices,
};

/// MfaDevice management commands
#[derive(Parser)]
#[command(name = "mfa_device")]
#[command(about = "Manage mfa_devices in the system")]
pub struct MfaDeviceCommands {
    #[command(subcommand)]
    pub action: MfaDeviceAction,
}

#[derive(Subcommand)]
pub enum MfaDeviceAction {
    /// Create a new mfa_device
    Create {
        /// MfaDevice data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// MfaDevice data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Get a mfa_device by ID
    Get {
        /// MfaDevice ID
        id: String,
    },
    /// List mfa_devices with optional filtering
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
    /// Update an existing mfa_device
    Update {
        /// MfaDevice ID
        id: String,
        /// MfaDevice data in JSON format
        #[arg(short, long)]
        data: Option<String>,
        /// MfaDevice data file path
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Delete a mfa_device (soft delete)
    Delete {
        /// MfaDevice ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Bulk create mfa_devices from file or stdin
    BulkCreate {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Upsert mfa_devices (update or insert)
    Upsert {
        /// Input file path (JSON array)
        #[arg(short, long)]
        file: Option<String>,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// List deleted mfa_devices (trash)
    ListTrash {
        /// Page number (default: 1)
        #[arg(short, long, default_value = "1")]
        page: usize,
        /// Page size (default: 20)
        #[arg(short, long, default_value = "20")]
        page_size: usize,
    },
    /// Restore a deleted mfa_device
    Restore {
        /// MfaDevice ID
        id: String,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
    /// Empty trash (permanently delete all mfa_devices)
    EmptyTrash {
        /// Confirmation flag
        #[arg(long)]
        confirm: bool,
        /// User ID performing the action
        #[arg(long, default_value = "cli-user")]
        user_id: String,
    },
}

pub struct MfaDeviceCliHandler {
    services: MfaDeviceApplicationServices,
}

impl MfaDeviceCliHandler {
    pub fn new(services: MfaDeviceApplicationServices) -> Self {
        Self { services }
    }

    pub async fn handle(&self, commands: MfaDeviceCommands) -> Result<()> {
        match commands.action {
            MfaDeviceAction::Create { data, file, user_id } => {
                self.create_mfa_device(data, file, user_id).await
            }
            MfaDeviceAction::Get { id } => {
                self.get_mfa_device(&id).await
            }
            MfaDeviceAction::List { page, page_size, sort_by, sort_direction, search, filters } => {
                self.list_mfa_devices(page, page_size, sort_by, sort_direction, search, filters).await
            }
            MfaDeviceAction::Update { id, data, file, user_id } => {
                self.update_mfa_device(&id, data, file, user_id).await
            }
            MfaDeviceAction::Delete { id, user_id } => {
                self.delete_mfa_device(&id, user_id).await
            }
            MfaDeviceAction::BulkCreate { file, user_id } => {
                self.bulk_create_mfa_devices(file, user_id).await
            }
            MfaDeviceAction::Upsert { file, user_id } => {
                self.upsert_mfa_devices(file, user_id).await
            }
            MfaDeviceAction::ListTrash { page, page_size } => {
                self.list_trash(page, page_size).await
            }
            MfaDeviceAction::Restore { id, user_id } => {
                self.restore_mfa_device(&id, user_id).await
            }
            MfaDeviceAction::EmptyTrash { confirm, user_id } => {
                self.empty_trash(confirm, user_id).await
            }
        }
    }

    async fn create_mfa_device(&self, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let mfa_device_data = self.parse_mfa_device_data(data, file)?;

        let command = CreateMfaDeviceCommand {
            // TODO: Map parsed data to command fields
            custom_fields: mfa_device_data,
            created_by: user_id,
        };

        let response = self.services.create_mfa_device_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ MfaDevice created successfully!".green());
            if let Some(mfa_device) = response.mfa_device {
                self.display_mfa_device(&mfa_device)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn get_mfa_device(&self, id: &str) -> Result<()> {
        let query = GetMfaDeviceQuery {
            id: id.to_string(),
        };

        let response = self.services.get_mfa_device_handler().handle(query).await?;

        if response.success {
            if let Some(mfa_device) = response.mfa_device {
                self.display_mfa_device(&mfa_device)?;
            } else {
                println!("{}", "MfaDevice not found".yellow());
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_mfa_devices(&self, page: usize, page_size: usize, sort_by: Option<String>, sort_direction: String, search: Option<String>, filters: Option<String>) -> Result<()> {
        let filters = if let Some(filters_json) = filters {
            Some(serde_json::from_str::<serde_json::Value>(&filters_json)?)
        } else {
            None
        };

        let filters = if let Some(filter_value) = filters {
            // TODO: Convert JSON filters to application filters
            Some(MfaDeviceFilters::new().with_search(search.unwrap_or_default()))
        } else if search.is_some() {
            Some(MfaDeviceFilters::new().with_search(search.unwrap_or_default()))
        } else {
            None
        };

        let query = ListMfaDeviceQuery {
            page,
            page_size,
            sort_by,
            sort_direction,
            filters,
        };

        let response = self.services.list_mfa_devices_handler().handle(query).await?;

        if response.success {
            println!("{}", format!("📄 Found {} mfa_devices (page {}/{}):",
                response.mfa_devices.len(), response.page, response.total_pages).cyan());

            for mfa_device in &response.mfa_devices {
                self.display_mfa_device_compact(mfa_device)?;
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

    async fn update_mfa_device(&self, id: &str, data: Option<String>, file: Option<String>, user_id: String) -> Result<()> {
        let mfa_device_data = self.parse_mfa_device_data(data, file)?;

        let command = UpdateMfaDeviceCommand {
            id: id.to_string(),
            // TODO: Map parsed data to command fields
            custom_fields: mfa_device_data,
            updated_by: user_id,
        };

        let response = self.services.update_mfa_device_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ MfaDevice updated successfully!".green());
            if let Some(mfa_device) = response.mfa_device {
                self.display_mfa_device(&mfa_device)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn delete_mfa_device(&self, id: &str, user_id: String) -> Result<()> {
        let command = DeleteMfaDeviceCommand {
            id: id.to_string(),
            deleted_by: user_id,
        };

        let response = self.services.delete_mfa_device_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ MfaDevice deleted successfully!".green());
            println!("{}", "💡 Use 'restore' command to recover if needed".blue());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn bulk_create_mfa_devices(&self, file: Option<String>, user_id: String) -> Result<()> {
        let mfa_devices_data = self.parse_bulk_mfa_devices_data(file)?;

        let commands: Vec<CreateMfaDeviceCommand> = mfa_devices_data
            .into_iter()
            .map(|data| CreateMfaDeviceCommand {
                custom_fields: data,
                created_by: user_id.clone(),
            })
            .collect();

        let command = BulkCreateMfaDeviceCommand {
            mfa_devices: commands,
        };

        let response = self.services.bulk_create_mfa_devices_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ {} mfa_devices created successfully!", response.created_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn upsert_mfa_devices(&self, file: Option<String>, user_id: String) -> Result<()> {
        let mfa_devices_data = self.parse_bulk_mfa_devices_data(file)?;

        let command = UpsertMfaDeviceCommand {
            // TODO: Map parsed data to command fields
            custom_fields: mfa_devices_data,
            user_id,
        };

        let response = self.services.upsert_mfa_device_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Upsert completed! Created: {}, Updated: {}", response.created, response.updated).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn list_trash(&self, page: usize, page_size: usize) -> Result<()> {
        // This would use ListDeletedMfaDeviceQuery
        println!("{}", "🗑️  Deleted mfa_devices (trash):".yellow());
        println!("{}", "Feature not yet implemented - TODO: Add ListDeletedMfaDeviceQuery handler".yellow());
        Ok(())
    }

    async fn restore_mfa_device(&self, id: &str, user_id: String) -> Result<()> {
        let command = RestoreMfaDeviceCommand {
            id: id.to_string(),
            restored_by: user_id,
        };

        let response = self.services.restore_mfa_device_handler().handle(command).await?;

        if response.success {
            println!("{}", "✅ MfaDevice restored successfully!".green());
            if let Some(mfa_device) = response.mfa_device {
                self.display_mfa_device(&mfa_device)?;
            }
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    async fn empty_trash(&self, confirm: bool, user_id: String) -> Result<()> {
        if !confirm {
            println!("{}", "⚠️  This action will permanently delete all deleted mfa_devices!".yellow());
            println!("{}", "Use --confirm to proceed".yellow());
            return Ok(());
        }

        let command = EmptyTrashCommand {
            user_id,
        };

        let response = self.services.empty_mfa_device_trash_handler().handle(command).await?;

        if response.success {
            println!("{}", format!("✅ Trash emptied! {} mfa_devices permanently deleted.", response.deleted_count).green());
        } else {
            eprintln!("{} {}", "❌".red(), response.message.red());
        }

        Ok(())
    }

    // Helper methods
    fn parse_mfa_device_data(&self, data: Option<String>, file: Option<String>) -> Result<std::collections::HashMap<String, serde_json::Value>> {
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

    fn parse_bulk_mfa_devices_data(&self, file: Option<String>) -> Result<Vec<std::collections::HashMap<String, serde_json::Value>>> {
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

    fn display_mfa_device(&self, mfa_device: &MfaDeviceDto) -> Result<()> {
        println!("{}", format!("📋 MfaDevice Details:").cyan());
        println!("{}", format!("  ID: {}", mfa_device.id).white());
        // TODO: Add more field display based on your mfa_device structure
        println!("{}", format!("  Created: {:?}", mfa_device.created_at).white());
        println!("{}", format!("  Updated: {:?}", mfa_device.updated_at).white());
        Ok(())
    }

    fn display_mfa_device_compact(&self, mfa_device: &MfaDeviceDto) -> Result<()> {
        println!("{}", format!("🔹 {} | {:?}", mfa_device.id, mfa_device.created_at).white());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mfa_device_commands_parsing() {
        // TODO: Add CLI command parsing tests
    }
}
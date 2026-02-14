//! # TodoRust - Todoist CLI Client
//!
//! A modern CLI client for Todoist built with Rust.

pub mod cli;
pub mod config;
pub mod error;
pub mod formatter;
pub mod models;
pub mod sync;

pub use formatter::{Formattable, OutputFormat};
pub use models::Project;
pub use sync::{SyncFilter, SyncLabel, SyncProject, SyncSection, SyncTask, TodoistSyncClient};

use crate::cli::{handle_error, AddCommands, Cli, Commands, CompleteCommands, ConfigCommands, DeleteCommands, EditCommands, GetCommands, MoveCommands, ReopenCommands, ReorderCommands};
use clap::Parser;

#[tokio::main]
async fn main() -> crate::error::Result<()> {
    let cli = Cli::parse();

    // Handle init command separately (doesn't require config)
    if let Commands::Init(init_cmd) = &cli.command {
        let token = if let Some(t) = &init_cmd.api_token {
            t.clone()
        } else {
            println!("No API token provided.");
            println!("Please get your API token from: https://todoist.com/app/settings/integrations");
            println!("Enter your API token:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).map_err(|e| crate::error::TodoError::Io(e))?;
            input.trim().to_string()
        };

        if token.is_empty() {
            handle_error(crate::error::TodoError::InvalidInput("API token cannot be empty".to_string()));
        }

        if let Err(e) = crate::config::init_config(&token) {
            handle_error(e);
        }
        println!("Configuration initialized successfully!");
        return Ok(());
    }

    // Load config for other commands
    let config = match crate::config::load_config() {
        Ok(c) => c,
        Err(e) => {
            handle_error(e);
        }
    };

    // Mask API token for display: show "****" when token < 4
    let masked_token = if config.api_token.len() < 4 {
        "****".to_string()
    } else {
        format!("{}****", &config.api_token[..config.api_token.len() - 4])
    };

    // Create sync client
    let client = crate::sync::TodoistSyncClient::new(config.api_token);

    // Determine output format (command-specific override or global)
    let format = match &cli.command {
        Commands::Get(GetCommands::Tasks { format, .. }) => format.clone().unwrap_or(cli.format),
        Commands::Get(GetCommands::Projects { format, .. }) => format.clone().unwrap_or(cli.format),
        Commands::Get(GetCommands::Task { format, .. }) => format.clone().unwrap_or(cli.format),
        Commands::Get(GetCommands::Sections { format, .. }) => format.clone().unwrap_or(cli.format),
        _ => cli.format,
    };

    // Execute command
    match &cli.command {
        // Config commands
        Commands::Config(ConfigCommands::Get) => {
            println!("API token: {}", masked_token);
        }
        Commands::Config(ConfigCommands::Set) => {
            eprintln!("Error: Use 'init' command to set API token");
            std::process::exit(1);
        }

        // Get commands
        Commands::Get(GetCommands::Tasks { filter, .. }) => {
            cli::handlers::get_tasks(&client, filter.as_deref(), &format).await?;
        }
        Commands::Get(GetCommands::Projects { .. }) => {
            cli::handlers::get_projects(&client, &format).await?;
        }
        Commands::Get(GetCommands::Task { task_id, .. }) => {
            cli::handlers::get_task(&client, task_id, &format).await?;
        }
        Commands::Get(GetCommands::Sections { project_id, .. }) => {
            cli::handlers::get_sections(&client, project_id.as_deref(), &format).await?;
        }
        Commands::Get(GetCommands::Filters { .. }) => {
            cli::handlers::get_filters(&client, &format).await?;
        }
        Commands::Get(GetCommands::Labels { .. }) => {
            cli::handlers::get_labels(&client, &format).await?;
        }

        // Add commands
        Commands::Add(AddCommands::Task {
            title,
            content,
            description,
            project_id,
            due_date,
            priority,
            labels,
            ..
        }) => {
            cli::handlers::add_task(
                &client,
                title.clone(),
                content.clone(),
                description.clone(),
                project_id.clone(),
                due_date.clone(),
                *priority,
                labels.clone(),
            )
            .await?;
        }
        Commands::Add(AddCommands::Section { name, project_id }) => {
            cli::handlers::add_section(&client, name.clone(), project_id.clone()).await?;
        }
        Commands::Add(AddCommands::Project { name, color, favorite }) => {
            cli::handlers::add_project(&client, name.clone(), color.clone(), *favorite).await?;
        }
        Commands::Add(AddCommands::Label { name, color }) => {
            cli::handlers::add_label(&client, name.clone(), color.clone()).await?;
        }
        Commands::Add(AddCommands::Filter { name, query, color }) => {
            cli::handlers::add_filter(&client, name.clone(), query.clone(), color.clone()).await?;
        }

        // Edit commands
        Commands::Edit(EditCommands::Task {
            task_id,
            title,
            content,
            project_id,
            due_date,
            priority,
            labels,
        }) => {
            cli::handlers::edit_task(
                &client,
                task_id.clone(),
                title.clone(),
                content.clone(),
                project_id.clone(),
                due_date.clone(),
                *priority,
                labels.clone(),
            )
            .await?;
        }
        Commands::Edit(EditCommands::Project { project_id, name }) => {
            cli::handlers::edit_project(&client, project_id.clone(), name.clone()).await?;
        }
        Commands::Edit(EditCommands::Section { section_id, name }) => {
            cli::handlers::edit_section(&client, section_id.clone(), name.clone()).await?;
        }
        Commands::Edit(EditCommands::Label { label_id, name, color }) => {
            cli::handlers::edit_label(&client, label_id.clone(), name.clone(), color.clone()).await?;
        }
        Commands::Edit(EditCommands::Filter { filter_id, name, query, color }) => {
            cli::handlers::edit_filter(&client, filter_id.clone(), name.clone(), query.clone(), color.clone()).await?;
        }

        // Complete/Reopen
        Commands::Complete(CompleteCommands::Task { task_id }) => {
            cli::handlers::complete_task(&client, task_id.clone()).await?;
        }
        Commands::Reopen(ReopenCommands::Task { task_id }) => {
            cli::handlers::reopen_task(&client, task_id.clone()).await?;
        }

        // Delete commands
        Commands::Delete(DeleteCommands::Task { task_id }) => {
            cli::handlers::delete_task(&client, task_id.clone()).await?;
        }
        Commands::Delete(DeleteCommands::Project { project_id }) => {
            cli::handlers::delete_project(&client, project_id.clone()).await?;
        }
        Commands::Delete(DeleteCommands::Section { section_id }) => {
            cli::handlers::delete_section(&client, section_id.clone()).await?;
        }

        // Move commands
        Commands::Move(MoveCommands::Task { task_id, project_id, section_id }) => {
            cli::handlers::move_task(&client, task_id.clone(), project_id.clone(), section_id.clone()).await?;
        }

        // Reorder commands
        Commands::Reorder(ReorderCommands::Sections { section_ids }) => {
            cli::handlers::reorder_sections(&client, section_ids.clone()).await?;
        }

        // Completion command
        Commands::Completion { shell } => {
            cli::handlers::generate_completions(*shell);
        }

        // Init was handled above
        Commands::Init(_) => unreachable!(),
    }

    Ok(())
}

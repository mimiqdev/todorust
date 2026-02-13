//! # TodoRust - Todoist CLI Client
//!
//! A modern CLI client for Todoist built with Rust.
//!
//! ## Features
//!
//! - **Sync API**: Efficient batch operations using Todoist's Sync API
//! - **Multiple Output Formats**: JSON, Checklist, and Structured views
//! - **Task Management**: Create, complete, reopen, and organize tasks
//!
//! ## Usage
//!
//! ```bash
//! # Initialize configuration
//! todorust init --api-token YOUR_TOKEN
//!
//! # List tasks
//! todorust tasks
//! todorust tasks --filter "today"
//!
//! # Create a task
//! todorust create --title "Buy milk"
//!
//! # Complete a task
//! todorust complete --task-id 12345
//! ```
//!
//! ## Modules
//!
//! - [`sync`]: Todoist Sync API client for efficient batch operations
//! - [`api`]: Legacy REST API client (deprecated, use [`sync`] instead)
//! - [`formatter`]: Output formatting utilities

// pub mod api;  // DEPRECATED: Legacy REST API client
pub mod config;
pub mod error;
pub mod formatter;
pub mod models;
pub mod sync;

pub use formatter::{Formattable, OutputFormat};
pub use models::Project;
pub use sync::{SyncFilter, SyncLabel, SyncProject, SyncSection, SyncTask, TodoistSyncClient};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todorust")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, short, global = true, default_value = "json")]
    format: OutputFormat,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize configuration
    Init(InitCommand),

    /// Configuration management
    #[command(subcommand)]
    Config(ConfigCommands),

    /// Get resources
    #[command(subcommand)]
    Get(GetCommands),

    /// Add resources
    #[command(subcommand)]
    Add(AddCommands),

    /// Edit resources
    #[command(subcommand)]
    Edit(EditCommands),

    /// Complete a task
    #[command(subcommand)]
    Complete(CompleteCommands),

    /// Reopen a task
    #[command(subcommand)]
    Reopen(ReopenCommands),

    /// Delete resources
    #[command(subcommand)]
    Delete(DeleteCommands),

    /// Move resources
    #[command(subcommand)]
    Move(MoveCommands),

    /// Reorder resources
    #[command(subcommand)]
    Reorder(ReorderCommands),
}

#[derive(Parser)]
struct InitCommand {
    #[arg(long = "api-token")]
    api_token: String,
}

#[derive(Clone, Subcommand)]
enum ConfigCommands {
    /// Get configuration
    Get,
    /// Set configuration
    Set,
}

#[derive(Clone, Subcommand)]
enum GetCommands {
    /// Get tasks with optional filter
    Tasks {
        #[arg(long)]
        filter: Option<String>,
        #[arg(long, short)]
        format: Option<OutputFormat>,
    },
    /// Get all projects
    Projects {
        #[arg(long, short)]
        format: Option<OutputFormat>,
    },
    /// Get a specific task
    Task {
        #[arg(long)]
        task_id: String,
        #[arg(long, short)]
        format: Option<OutputFormat>,
    },
    /// Get all sections (optionally filtered by project)
    Sections {
        #[arg(long)]
        project_id: Option<String>,
        #[arg(long, short)]
        format: Option<OutputFormat>,
    },
    /// Get all filters
    Filters {
        #[arg(long, short)]
        format: Option<OutputFormat>,
    },
    /// Get all labels
    Labels {
        #[arg(long, short)]
        format: Option<OutputFormat>,
    },
}

#[derive(Clone, Subcommand)]
enum AddCommands {
    /// Create a new task
    Task {
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        content: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        project_id: Option<String>,
        #[arg(long)]
        due_date: Option<String>,
        #[arg(long)]
        priority: Option<u8>,
        #[arg(long)]
        labels: Option<String>,
        #[arg(long, short)]
        format: Option<OutputFormat>,
    },
    /// Create a new section
    Section {
        #[arg(long)]
        name: String,
        #[arg(long)]
        project_id: String,
    },
    /// Create a new project
    Project {
        #[arg(long)]
        name: String,
        #[arg(long)]
        color: Option<String>,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        favorite: bool,
    },
}

#[derive(Clone, Subcommand)]
enum EditCommands {
    /// Edit a task
    Task {
        #[arg(long)]
        task_id: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        content: Option<String>,
        #[arg(long)]
        project_id: Option<String>,
        #[arg(long)]
        due_date: Option<String>,
        #[arg(long)]
        priority: Option<u8>,
        #[arg(long)]
        labels: Option<String>,
    },
    /// Edit a project
    Project {
        #[arg(long)]
        project_id: String,
        #[arg(long)]
        name: Option<String>,
    },
    /// Edit a section
    Section {
        #[arg(long)]
        section_id: String,
        #[arg(long)]
        name: Option<String>,
    },
}

#[derive(Clone, Subcommand)]
enum CompleteCommands {
    /// Complete a task
    Task {
        #[arg(long)]
        task_id: String,
    },
}

#[derive(Clone, Subcommand)]
enum ReopenCommands {
    /// Reopen a task
    Task {
        #[arg(long)]
        task_id: String,
    },
}

#[derive(Clone, Subcommand)]
enum DeleteCommands {
    /// Delete a task
    Task {
        #[arg(long)]
        task_id: String,
    },
    /// Delete a project
    Project {
        #[arg(long)]
        project_id: String,
    },
    /// Delete a section
    Section {
        #[arg(long)]
        section_id: String,
    },
}

/// Move commands - for moving tasks between projects/sections
#[derive(Clone, Subcommand)]
enum MoveCommands {
    /// Move a task to a different project or section
    Task {
        #[arg(long)]
        task_id: String,
        #[arg(long)]
        project_id: String,
        #[arg(long)]
        section_id: Option<String>,
    },
}

/// Reorder commands - for reordering sections
#[derive(Clone, Subcommand)]
enum ReorderCommands {
    /// Reorder sections within a project
    Sections {
        /// Section IDs in desired order (comma-separated)
        #[arg(long)]
        section_ids: String,
    },
}

fn validate_priority(priority: u8) -> bool {
    (1..=4).contains(&priority)
}

#[allow(dead_code)]
fn handle_error(error: crate::error::TodoError) -> ! {
    match &error {
        crate::error::TodoError::ConfigNotFound => {
            eprintln!("Error: Configuration not found.");
            eprintln!("Run: todorust init --api-token YOUR_TOKEN");
        }
        crate::error::TodoError::Http(status) => {
            eprintln!("Error: HTTP {}", status);
        }
        crate::error::TodoError::Api(msg) => {
            eprintln!("API Error: {}", msg);
        }
        crate::error::TodoError::Request(e) => {
            eprintln!("Request Error: {}", e);
        }
        crate::error::TodoError::Config(msg) => {
            eprintln!("Config Error: {}", msg);
        }
        crate::error::TodoError::InvalidInput(msg) => {
            eprintln!("Invalid Input: {}", msg);
        }
        crate::error::TodoError::Serialize(msg) => {
            eprintln!("Serialize Error: {}", msg);
        }
    }
    std::process::exit(1);
}

#[tokio::main]
async fn main() -> crate::error::Result<()> {
    let cli = Cli::parse();

    // Handle init command separately (doesn't require config)
    if let Commands::Init(init_cmd) = &cli.command {
        if let Err(e) = crate::config::init_config(&init_cmd.api_token) {
            handle_error(e);
        }
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
            get_tasks(&client, filter.as_deref(), &format).await?;
        }
        Commands::Get(GetCommands::Projects { .. }) => {
            get_projects(&client, &format).await?;
        }
        Commands::Get(GetCommands::Task { task_id, .. }) => {
            get_task(&client, task_id, &format).await?;
        }
        Commands::Get(GetCommands::Sections { project_id, .. }) => {
            get_sections(&client, project_id.as_deref(), &format).await?;
        }
        Commands::Get(GetCommands::Filters { .. }) => {
            get_filters(&client, &format).await?;
        }
        Commands::Get(GetCommands::Labels { .. }) => {
            get_labels(&client, &format).await?;
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
            let task_content = title
                .as_ref()
                .or(content.as_ref())
                .ok_or_else(|| {
                    crate::error::TodoError::InvalidInput(
                        "Task title or content required".to_string(),
                    )
                })?
                .clone();

            // Validate priority - return error if invalid
            let validated_priority = if let Some(p) = *priority {
                if !validate_priority(p) {
                    return Err(crate::error::TodoError::InvalidInput(format!(
                        "Invalid priority {}. Priority must be between 1 and 4.",
                        p
                    )));
                }
                Some(p)
            } else {
                None
            };

            let labels_vec: Option<Vec<&str>> = labels
                .as_ref()
                .map(|l| l.split(',').map(|s| s.trim()).collect());

            let task_id = client
                .add_task(
                    &task_content,
                    description.as_deref(),
                    project_id.as_deref(),
                    None,
                    due_date.as_deref(),
                    validated_priority,
                    labels_vec,
                )
                .await?;

            println!("Task created with ID: {}", task_id);
        }

        // Add section command
        Commands::Add(AddCommands::Section { name, project_id }) => {
            let section_id = client.add_section(name, project_id).await?;
            println!("Section created with ID: {}", section_id);
        }

        // Add project command
        Commands::Add(AddCommands::Project {
            name,
            color,
            favorite,
        }) => {
            let fav = *favorite;
            let project_id = client
                .add_project(name, color.as_deref(), Some(fav))
                .await?;
            println!("Project created with ID: {}", project_id);
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
            let task_content = title.as_ref().or(content.as_ref()).map(|s| s.as_str());
            let labels_vec: Option<Vec<&str>> = labels
                .as_ref()
                .map(|l| l.split(',').map(|s| s.trim()).collect());

            // Validate priority - return error if invalid
            if let Some(p) = *priority {
                if !validate_priority(p) {
                    handle_error(crate::error::TodoError::InvalidInput(format!(
                        "Invalid priority {}. Priority must be between 1 and 4.",
                        p
                    )));
                }
            }

            // If project_id is provided, move the task to the new project first
            if let Some(ref new_project_id) = project_id {
                let builder =
                    crate::sync::CommandBuilder::new().item_move(task_id, new_project_id, None);
                client.execute(builder).await?;
            }

            // Update task fields after move
            client
                .update_task(
                    task_id,
                    task_content,
                    None,
                    *priority,
                    due_date.as_deref(),
                    labels_vec,
                )
                .await?;

            println!("Task {} updated", task_id);
        }
        Commands::Edit(EditCommands::Project { project_id, name }) => {
            // Implement project update using project_update command
            if name.is_none() {
                eprintln!("Error: No fields to update. Provide at least --name.");
                std::process::exit(1);
            }
            let builder = crate::sync::CommandBuilder::new().project_update(
                project_id,
                name.as_deref(),
                None,
                None,
            );
            client.execute(builder).await?;
            println!("Project {} updated", project_id);
        }
        Commands::Edit(EditCommands::Section { section_id, name }) => {
            if name.is_none() {
                eprintln!("Error: No fields to update. Provide at least --name.");
                std::process::exit(1);
            }
            client
                .update_section(section_id, name.as_deref().unwrap())
                .await?;
            println!("Section {} updated", section_id);
        }

        // Complete commands
        Commands::Complete(CompleteCommands::Task { task_id }) => {
            client.complete_task(task_id).await?;
            println!("Task {} completed", task_id);
        }

        // Reopen commands
        Commands::Reopen(ReopenCommands::Task { task_id }) => {
            // Use item_reopen to reopen a task
            let builder = crate::sync::CommandBuilder::new().item_reopen(task_id);
            client.execute(builder).await?;
            println!("Task {} reopened", task_id);
        }

        // Delete commands
        Commands::Delete(DeleteCommands::Task { task_id }) => {
            client.delete_task(task_id).await?;
            println!("Task {} deleted", task_id);
        }
        Commands::Delete(DeleteCommands::Project { project_id }) => {
            let builder = crate::sync::CommandBuilder::new().project_delete(project_id);
            client.execute(builder).await?;
            println!("Project {} deleted", project_id);
        }
        Commands::Delete(DeleteCommands::Section { section_id }) => {
            client.delete_section(section_id).await?;
            println!("Section {} deleted", section_id);
        }

        // Move commands
        Commands::Move(MoveCommands::Task {
            task_id,
            project_id,
            section_id,
        }) => {
            let builder = crate::sync::CommandBuilder::new().item_move(
                task_id,
                project_id,
                section_id.as_deref(),
            );
            client.execute(builder).await?;
            println!("Task {} moved to project {}", task_id, project_id);
            if let Some(sid) = section_id {
                println!("Task {} moved to section {}", task_id, sid);
            }
        }

        // Reorder commands
        Commands::Reorder(ReorderCommands::Sections { section_ids }) => {
            let sections: Vec<&str> = section_ids.split(',').map(|s| s.trim()).collect();
            let sections_with_order: Vec<(&str, i64)> = sections
                .iter()
                .enumerate()
                .map(|(i, id)| (*id, i as i64))
                .collect();
            client.reorder_sections(&sections_with_order).await?;
            println!("Sections reordered: {}", section_ids);
        }

        // Init was handled above
        Commands::Init(_) => unreachable!(),
    }

    Ok(())
}

async fn get_tasks(
    client: &crate::sync::TodoistSyncClient,
    filter: Option<&str>,
    format: &crate::formatter::OutputFormat,
) -> crate::error::Result<()> {
    // Get all tasks and projects to resolve project names
    let tasks: Vec<crate::models::Task> = client.get_tasks().await?;
    let projects: Vec<crate::models::Project> = client.get_projects().await?;

    // Build project name lookup
    let project_map: std::collections::HashMap<&str, &str> = projects
        .iter()
        .map(|p| (p.id.as_str(), p.name.as_str()))
        .collect();

    // Convert to TaskOutput with project names
    let task_outputs: Vec<crate::models::TaskOutput> = tasks
        .into_iter()
        .map(|t| {
            let project_name = t
                .project_id
                .as_ref()
                .and_then(|pid| project_map.get(pid.as_str()))
                .map(|s| s.to_string());

            crate::models::TaskOutput {
                id: t.id,
                content: t.content,
                description: t.description,
                project_id: t.project_id,
                project_name,
                due_date: t.due.and_then(|d| d.date),
                is_completed: t.is_completed,
                created_at: t.created_at,
                order: t.order,
                priority: t.priority,
                labels: t.labels,
            }
        })
        .collect();

    // Apply filter if provided
    let filtered: Vec<crate::models::TaskOutput> = if let Some(f) = filter {
        // Simple filter implementation - check if content or project contains the filter string
        task_outputs
            .into_iter()
            .filter(|t| {
                t.content.to_lowercase().contains(&f.to_lowercase())
                    || t.project_name
                        .as_ref()
                        .map(|p| p.to_lowercase().contains(&f.to_lowercase()))
                        .unwrap_or(false)
            })
            .collect()
    } else {
        task_outputs
    };

    println!("{}", filtered.format(format));
    Ok(())
}

async fn get_projects(
    client: &crate::sync::TodoistSyncClient,
    format: &crate::formatter::OutputFormat,
) -> crate::error::Result<()> {
    let projects = client.get_projects().await?;
    println!("{}", projects.format(format));
    Ok(())
}

async fn get_sections(
    client: &crate::sync::TodoistSyncClient,
    project_id: Option<&str>,
    format: &crate::formatter::OutputFormat,
) -> crate::error::Result<()> {
    let mut sections = client.get_sections().await?;

    // Filter by project_id if provided
    if let Some(pid) = project_id {
        sections.retain(|s| s.project_id == pid);
    }

    println!("{}", sections.format(format));
    Ok(())
}

async fn get_filters(
    client: &crate::sync::TodoistSyncClient,
    format: &crate::formatter::OutputFormat,
) -> crate::error::Result<()> {
    let filters = client.get_filters().await?;
    println!("{}", filters.format(format));
    Ok(())
}

async fn get_labels(
    client: &crate::sync::TodoistSyncClient,
    format: &crate::formatter::OutputFormat,
) -> crate::error::Result<()> {
    let labels = client.get_labels().await?;
    println!("{}", labels.format(format));
    Ok(())
}

async fn get_task(
    client: &crate::sync::TodoistSyncClient,
    task_id: &str,
    format: &crate::formatter::OutputFormat,
) -> crate::error::Result<()> {
    let tasks: Vec<crate::models::Task> = client.get_tasks().await?;
    let projects: Vec<crate::models::Project> = client.get_projects().await?;

    let project_map: std::collections::HashMap<&str, &str> = projects
        .iter()
        .map(|p| (p.id.as_str(), p.name.as_str()))
        .collect();

    let task = tasks.into_iter().find(|t| t.id == task_id).ok_or_else(|| {
        crate::error::TodoError::InvalidInput(format!("Task {} not found", task_id))
    })?;

    let project_name = task
        .project_id
        .as_ref()
        .and_then(|pid| project_map.get(pid.as_str()))
        .map(|s| s.to_string());

    let task_output = crate::models::TaskOutput {
        id: task.id,
        content: task.content,
        description: task.description,
        project_id: task.project_id,
        project_name,
        due_date: task.due.and_then(|d| d.date),
        is_completed: task.is_completed,
        created_at: task.created_at,
        order: task.order,
        priority: task.priority,
        labels: task.labels,
    };

    println!("{}", vec![task_output].format(format));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing_get_tasks() {
        let args = vec!["todorust", "get", "tasks"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(matches!(
            cli.command,
            Commands::Get(GetCommands::Tasks { filter: None, .. })
        ));
    }

    #[test]
    fn test_cli_get_tasks_with_filter() {
        let args = vec!["todorust", "get", "tasks", "--filter", "project:Work"];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Get(GetCommands::Tasks { filter, .. }) = cli.command {
            assert_eq!(filter, Some("project:Work".to_string()));
        } else {
            panic!("Expected Get Tasks command");
        }
    }

    #[test]
    fn test_cli_add_task() {
        let args = vec![
            "todorust",
            "add",
            "task",
            "--title",
            "New task",
            "--description",
            "Details",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Add(AddCommands::Task { title, .. }) = cli.command {
            assert_eq!(title, Some("New task".to_string()));
        } else {
            panic!("Expected Add Task command");
        }
    }

    #[test]
    fn test_cli_complete_task() {
        let args = vec!["todorust", "complete", "task", "--task-id", "12345"];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Complete(CompleteCommands::Task { task_id }) = cli.command {
            assert_eq!(task_id, "12345");
        } else {
            panic!("Expected Complete Task command");
        }
    }

    #[test]
    fn test_cli_init() {
        let args = vec!["todorust", "init", "--api-token", "test_token"];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Init(cmd) = cli.command {
            assert_eq!(cmd.api_token, "test_token");
        } else {
            panic!("Expected Init command");
        }
    }

    #[test]
    fn test_priority_validation() {
        assert!(validate_priority(1));
        assert!(validate_priority(4));
        assert!(!validate_priority(0));
        assert!(!validate_priority(5));
    }

    #[test]
    fn test_cli_delete_task() {
        let args = vec!["todorust", "delete", "task", "--task-id", "67890"];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Delete(DeleteCommands::Task { task_id }) = cli.command {
            assert_eq!(task_id, "67890");
        } else {
            panic!("Expected Delete Task command");
        }
    }

    #[test]
    fn test_cli_delete_project() {
        let args = vec!["todorust", "delete", "project", "--project-id", "proj123"];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Delete(DeleteCommands::Project { project_id }) = cli.command {
            assert_eq!(project_id, "proj123");
        } else {
            panic!("Expected Delete Project command");
        }
    }

    #[test]
    fn test_cli_delete_section() {
        let args = vec!["todorust", "delete", "section", "--section-id", "sec456"];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Delete(DeleteCommands::Section { section_id }) = cli.command {
            assert_eq!(section_id, "sec456");
        } else {
            panic!("Expected Delete Section command");
        }
    }

    #[test]
    fn test_cli_reopen_task() {
        let args = vec!["todorust", "reopen", "task", "--task-id", "11122"];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Reopen(ReopenCommands::Task { task_id }) = cli.command {
            assert_eq!(task_id, "11122");
        } else {
            panic!("Expected Reopen Task command");
        }
    }

    #[test]
    fn test_cli_edit_task() {
        let args = vec![
            "todorust",
            "edit",
            "task",
            "--task-id",
            "33344",
            "--priority",
            "3",
            "--title",
            "Updated Task",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Edit(EditCommands::Task {
            task_id, priority, ..
        }) = cli.command
        {
            assert_eq!(task_id, "33344");
            assert_eq!(priority, Some(3));
        } else {
            panic!("Expected Edit Task command");
        }
    }

    #[test]
    fn test_cli_edit_project() {
        let args = vec![
            "todorust",
            "edit",
            "project",
            "--project-id",
            "proj555",
            "--name",
            "New Project Name",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Edit(EditCommands::Project { project_id, name }) = cli.command {
            assert_eq!(project_id, "proj555");
            assert_eq!(name, Some("New Project Name".to_string()));
        } else {
            panic!("Expected Edit Project command");
        }
    }

    #[test]
    fn test_edit_project() {
        // Use Command::Edit(EditCommands::Project{...}) to parse and print
        let args = vec![
            "todorust",
            "edit",
            "project",
            "--project-id",
            "test_project_123",
            "--name",
            "My Test Project",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Edit(EditCommands::Project { project_id, name }) = cli.command {
            println!("project_id: {}", project_id);
            println!("name: {:?}", name);
        } else {
            panic!("Expected Edit Project command");
        }
    }

    #[test]
    fn test_cli_add_task_full() {
        let args = vec![
            "todorust",
            "add",
            "task",
            "--title",
            "Full Task",
            "--content",
            "Task content here",
            "--description",
            "Detailed description",
            "--priority",
            "4",
            "--labels",
            "work,urgent",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Add(AddCommands::Task {
            title,
            content,
            description,
            priority,
            labels,
            ..
        }) = cli.command
        {
            assert_eq!(title, Some("Full Task".to_string()));
            assert_eq!(content, Some("Task content here".to_string()));
            assert_eq!(description, Some("Detailed description".to_string()));
            assert_eq!(priority, Some(4));
            assert_eq!(labels, Some("work,urgent".to_string()));
        } else {
            panic!("Expected Add Task command");
        }
    }

    // Tests for new P0 CLI commands: move task, get sections, add section

    #[test]
    fn test_cli_get_sections() {
        let args = vec!["todorust", "get", "sections"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(matches!(
            cli.command,
            Commands::Get(GetCommands::Sections {
                project_id: None,
                ..
            })
        ));
    }

    #[test]
    fn test_cli_get_sections_with_project_id() {
        let args = vec!["todorust", "get", "sections", "--project-id", "proj123"];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Get(GetCommands::Sections { project_id, .. }) = cli.command {
            assert_eq!(project_id, Some("proj123".to_string()));
        } else {
            panic!("Expected Get Sections command");
        }
    }

    #[test]
    fn test_cli_add_section() {
        let args = vec![
            "todorust",
            "add",
            "section",
            "--name",
            "New Section",
            "--project-id",
            "proj456",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Add(AddCommands::Section { name, project_id }) = cli.command {
            assert_eq!(name, "New Section");
            assert_eq!(project_id, "proj456");
        } else {
            panic!("Expected Add Section command");
        }
    }

    #[test]
    fn test_cli_move_task() {
        let args = vec![
            "todorust",
            "move",
            "task",
            "--task-id",
            "task789",
            "--project-id",
            "proj123",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Move(MoveCommands::Task {
            task_id,
            project_id,
            section_id,
        }) = cli.command
        {
            assert_eq!(task_id, "task789");
            assert_eq!(project_id, "proj123");
            assert_eq!(section_id, None);
        } else {
            panic!("Expected Move Task command");
        }
    }

    #[test]
    fn test_cli_move_task_with_section() {
        let args = vec![
            "todorust",
            "move",
            "task",
            "--task-id",
            "task456",
            "--project-id",
            "proj789",
            "--section-id",
            "sec123",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Move(MoveCommands::Task {
            task_id,
            project_id,
            section_id,
        }) = cli.command
        {
            assert_eq!(task_id, "task456");
            assert_eq!(project_id, "proj789");
            assert_eq!(section_id, Some("sec123".to_string()));
        } else {
            panic!("Expected Move Task command");
        }
    }

    #[test]
    fn test_cli_get_sections_format_option() {
        let args = vec!["todorust", "get", "sections", "--format", "checklist"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(matches!(
            cli.command,
            Commands::Get(GetCommands::Sections { .. })
        ));
    }

    // Tests for P1 CLI commands: edit section, add project, reorder sections, get filters, get labels

    #[test]
    fn test_cli_get_filters() {
        let args = vec!["todorust", "get", "filters"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(matches!(
            cli.command,
            Commands::Get(GetCommands::Filters { .. })
        ));
    }

    #[test]
    fn test_cli_get_labels() {
        let args = vec!["todorust", "get", "labels"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(matches!(
            cli.command,
            Commands::Get(GetCommands::Labels { .. })
        ));
    }

    #[test]
    fn test_cli_add_project() {
        let args = vec!["todorust", "add", "project", "--name", "New Project"];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Add(AddCommands::Project { name, .. }) = cli.command {
            assert_eq!(name, "New Project");
        } else {
            panic!("Expected Add Project command");
        }
    }

    #[test]
    fn test_cli_add_project_with_color() {
        let args = vec![
            "todorust",
            "add",
            "project",
            "--name",
            "My Project",
            "--color",
            "blue",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Add(AddCommands::Project { name, color, .. }) = cli.command {
            assert_eq!(name, "My Project");
            assert_eq!(color, Some("blue".to_string()));
        } else {
            panic!("Expected Add Project command");
        }
    }

    #[test]
    fn test_cli_add_project_with_favorite() {
        let args = vec![
            "todorust",
            "add",
            "project",
            "--name",
            "Favorite Project",
            "--favorite",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Add(AddCommands::Project { name, favorite, .. }) = cli.command {
            assert_eq!(name, "Favorite Project");
            assert!(favorite);
        } else {
            panic!("Expected Add Project command");
        }
    }

    #[test]
    fn test_cli_edit_section() {
        let args = vec![
            "todorust",
            "edit",
            "section",
            "--section-id",
            "sec123",
            "--name",
            "Updated Section",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Edit(EditCommands::Section { section_id, name }) = cli.command {
            assert_eq!(section_id, "sec123");
            assert_eq!(name, Some("Updated Section".to_string()));
        } else {
            panic!("Expected Edit Section command");
        }
    }

    #[test]
    fn test_cli_reorder_sections() {
        let args = vec![
            "todorust",
            "reorder",
            "sections",
            "--section-ids",
            "sec1,sec2,sec3",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Reorder(ReorderCommands::Sections { section_ids }) = cli.command {
            assert_eq!(section_ids, "sec1,sec2,sec3");
        } else {
            panic!("Expected Reorder Sections command");
        }
    }

    #[test]
    fn test_cli_get_filters_with_format() {
        let args = vec!["todorust", "get", "filters", "--format", "checklist"];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Get(GetCommands::Filters { format }) = cli.command {
            assert_eq!(format, Some(OutputFormat::Checklist));
        } else {
            panic!("Expected Get Filters command");
        }
    }

    #[test]
    fn test_cli_get_labels_with_format() {
        let args = vec!["todorust", "get", "labels", "--format", "json"];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Get(GetCommands::Labels { format }) = cli.command {
            assert_eq!(format, Some(OutputFormat::Json));
        } else {
            panic!("Expected Get Labels command");
        }
    }
}

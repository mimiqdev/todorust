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
#[allow(deprecated)]
use todorust::{
    api::TodoistClient,
    config::{init_config, load_config},
    error::TodoError,
    models::TaskOutput,
    sync::{self, CommandBuilder},
    Formattable, OutputFormat,
};

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
}

#[derive(Clone, Subcommand)]
enum MoveCommands {
    /// Move a task
    Task {
        #[arg(long)]
        task_id: String,
        #[arg(long)]
        project_id: String,
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

#[allow(dead_code)]
fn validate_priority(priority: u8) -> bool {
    (1..=4).contains(&priority)
}

#[allow(dead_code)]
fn handle_error(error: todorust::error::TodoError) {
    match &error {
        todorust::error::TodoError::ConfigNotFound => {
            eprintln!("Error: Configuration not found.");
            eprintln!("Run: todorust init --api-token YOUR_TOKEN");
        }
        todorust::error::TodoError::Http(status) => {
            eprintln!("Error: HTTP {}", status);
        }
        todorust::error::TodoError::Api(msg) => {
            eprintln!("API Error: {}", msg);
        }
        todorust::error::TodoError::Request(e) => {
            eprintln!("Request Error: {}", e);
        }
        todorust::error::TodoError::Config(msg) => {
            eprintln!("Config Error: {}", msg);
        }
        todorust::error::TodoError::InvalidInput(msg) => {
            eprintln!("Invalid Input: {}", msg);
        }
        todorust::error::TodoError::Serialize(msg) => {
            eprintln!("Serialize Error: {}", msg);
        }
    }
    std::process::exit(1);
}

// DEPRECATED: The main function using legacy REST API is commented out
// The sync API client (TodoistSyncClient) should be used instead for new implementations
/*
#[tokio::main]
async fn main() -> Result<(), TodoError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(cmd) => {
            init_config(&cmd.api_token)?;
            println!("Configuration initialized successfully!");
            return Ok(());
        }
        Commands::Config(cmd) => {
            match cmd {
                ConfigCommands::Get => {
                    let config = load_config()?;
                    println!("API Token: {}", config.api_token);
                }
                ConfigCommands::Set => {
                    eprintln!("Config set not implemented yet");
                }
            }
            return Ok(());
        }
        _ => {}
    }

    #[allow(deprecated)]
    let config = load_config()?;
    let client = TodoistClient::new(config.api_token.clone());
    let sync_client = sync::TodoistSyncClient::new(config.api_token);

    match cli.command {
        Commands::Get(cmd) => match cmd {
            GetCommands::Tasks { filter, format } => {
                let output_format = format.unwrap_or(cli.format);
                let tasks = client.get_tasks(filter.clone()).await?;
                let output = tasks.format(&output_format);
                println!("{}", output);
            }
            GetCommands::Projects { format } => {
                let output_format = format.unwrap_or(cli.format);
                let projects = client.get_projects().await?;
                let output = projects.format(&output_format);
                println!("{}", output);
            }
            GetCommands::Task { task_id, format } => {
                let output_format = format.unwrap_or(cli.format);
                let tasks = client.get_tasks(None).await?;
                let task = tasks
                    .iter()
                    .find(|t| t.id == task_id.parse::<u32>().unwrap_or_default().to_string())
                    .ok_or_else(|| TodoError::Api(format!("Task {} not found", task_id)))?;
                match output_format {
                    OutputFormat::Json => {
                        println!("{}", serde_json::to_string_pretty(task).unwrap_or_default());
                    }
                    OutputFormat::Checklist => {
                        let checkbox = if task.is_completed { "[x]" } else { "[ ]" };
                        if let Some(ref project) = task.project_name {
                            println!("- {} {} ({})", checkbox, task.content, project);
                        } else {
                            println!("- {} {}", checkbox, task.content);
                        }
                    }
                    OutputFormat::Structured => {
                        println!("Task:");
                        println!("  Content: {}", task.content);
                        if let Some(desc) = &task.description {
                            println!("  Description: {}", desc);
                        }
                        if let Some(ref project) = task.project_name {
                            println!("  Project: {}", project);
                        }
                        if let Some(ref due) = task.due_date {
                            println!("  Due: {}", due);
                        }
                        println!("  Priority: {}", task.priority);
                    }
                }
            }
        },
        Commands::Add(cmd) => match cmd {
            AddCommands::Task {
                title,
                content,
                description,
                project_id,
                due_date,
                priority,
                labels,
                format,
            } => {
                let output_format = format.unwrap_or(cli.format);
                let title_value = title.clone().filter(|value| !value.trim().is_empty());
                let content_value = content.clone().filter(|value| !value.trim().is_empty());
                if title_value.is_some() && content_value.is_some() {
                    eprintln!("Warning: both --title and --content provided; using --title.");
                }
                let content = title_value.or(content_value).ok_or_else(|| {
                    todorust::error::TodoError::InvalidInput("Task title/content cannot be empty".to_string())
                })?;

                if let Some(p) = priority {
                    if !validate_priority(p) {
                        return Err(todorust::error::TodoError::InvalidInput(
                            "Priority must be between 1 and 4".to_string(),
                        ));
                    }
                }

                let labels_vec = labels.clone().and_then(|l| {
                    if l.is_empty() {
                        None
                    } else {
                        Some(l.split(',').map(|s| s.trim().to_string()).collect())
                    }
                });

                let task = client
                    .create_task(
                        &content,
                        description.clone(),
                        project_id.clone(),
                        due_date.clone(),
                        priority,
                        labels_vec,
                    )
                    .await?;
                let output = vec![task].format(&output_format);
                println!("{}", output);
            }
        },
        Commands::Edit(cmd) => match cmd {
            EditCommands::Task {
                task_id,
                title,
                content,
                project_id: _,
                due_date,
                priority,
                labels,
            } => {
                let content_value = title.as_deref().or(content.as_deref());
                let labels_vec = labels
                    .as_ref()
                    .map(|l| l.split(',').map(|s| s.trim()).collect());
                sync_client
                    .update_task(
                        &task_id,
                        content_value,
                        None, // description not in edit command yet
                        priority,
                        due_date.as_deref(),
                        labels_vec,
                    )
                    .await?;
                println!("Task {} updated", task_id);
            }
            EditCommands::Project { project_id, name } => {
                let builder =
                    CommandBuilder::new().project_update(&project_id, name.as_deref(), None, None);
                sync_client.execute(builder).await?;
                println!("Project {} updated", project_id);
            }
        },
        Commands::Complete(cmd) => match cmd {
            CompleteCommands::Task { task_id } => {
                client.complete_task(&task_id).await?;
                println!("Task {} completed", task_id);
            }
        },
        Commands::Reopen(cmd) => match cmd {
            ReopenCommands::Task { task_id } => {
                client.reopen_task(&task_id).await?;
                println!("Task {} reopened", task_id);
            }
        },
        Commands::Delete(cmd) => match cmd {
            DeleteCommands::Task { task_id } => {
                sync_client.delete_task(&task_id).await?;
                println!("Task {} deleted", task_id);
            }
            DeleteCommands::Project { project_id } => {
                let builder = CommandBuilder::new().project_delete(&project_id);
                sync_client.execute(builder).await?;
                println!("Project {} deleted", project_id);
            }
            DeleteCommands::Section { section_id } => {
                sync_client.delete_section(&section_id).await?;
                println!("Section {} deleted", section_id);
            }
        },
        Commands::Init(_) => unreachable!(),
        Commands::Config(_) => unreachable!(),
    }

    Ok(())
}
*/

fn main() {
    eprintln!("Error: The legacy REST API CLI has been deprecated.");
    eprintln!("Please use the library functionality via the sync API instead.");
    eprintln!("The todorust binary is currently disabled pending migration to sync API.");
    std::process::exit(1);
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
}

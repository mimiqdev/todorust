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
        if let Commands::Edit(EditCommands::Task { task_id, priority, .. }) = cli.command {
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
}

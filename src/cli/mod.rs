use crate::formatter::OutputFormat;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todorust")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long, short, global = true, default_value = "json")]
    pub format: OutputFormat,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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

    /// Synchronize data (default incremental, --force for full sync)
    #[command(subcommand)]
    Sync(SyncCommands),

    /// Cache management
    #[command(subcommand)]
    Cache(CacheCommands),

    /// Execute multiple commands in a single batch request
    Batch {
        /// JSON array of commands
        commands: String,
    },

    /// Generate shell completion scripts
    Completion {
        /// Shell to generate completion for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

#[derive(Parser)]
pub struct InitCommand {
    #[arg(long = "api-token")]
    pub api_token: Option<String>,
}

#[derive(Clone, Subcommand)]
pub enum ConfigCommands {
    /// Get configuration
    Get,
    /// Set configuration
    Set,
}

#[derive(Clone, Subcommand)]
pub enum GetCommands {
    /// Get tasks with optional filter
    Tasks {
        #[arg(long)]
        filter: Option<String>,
        #[arg(long, short)]
        format: Option<OutputFormat>,
        /// Specific fields to include in JSON output (e.g., "id,content")
        #[arg(long)]
        fields: Option<String>,
        /// Limit the number of tasks returned
        #[arg(long)]
        limit: Option<usize>,
    },
    /// Get all projects
    Projects {
        #[arg(long, short)]
        format: Option<OutputFormat>,
        /// Specific fields to include in JSON output
        #[arg(long)]
        fields: Option<String>,
    },
    /// Get a specific task
    Task {
        #[arg(long)]
        task_id: String,
        #[arg(long, short)]
        format: Option<OutputFormat>,
        /// Specific fields to include in JSON output
        #[arg(long)]
        fields: Option<String>,
    },
    /// Get all sections (optionally filtered by project)
    Sections {
        #[arg(long)]
        project_id: Option<String>,
        #[arg(long, short)]
        format: Option<OutputFormat>,
        /// Specific fields to include in JSON output
        #[arg(long)]
        fields: Option<String>,
    },
    /// Get all filters
    Filters {
        #[arg(long, short)]
        format: Option<OutputFormat>,
        /// Specific fields to include in JSON output
        #[arg(long)]
        fields: Option<String>,
    },
    /// Get all labels
    Labels {
        #[arg(long, short)]
        format: Option<OutputFormat>,
        /// Specific fields to include in JSON output
        #[arg(long)]
        fields: Option<String>,
    },
}

#[derive(Clone, Subcommand)]
pub enum AddCommands {
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
    /// Create a new label
    Label {
        #[arg(long)]
        name: String,
        #[arg(long)]
        color: Option<String>,
    },
    /// Create a new filter
    Filter {
        #[arg(long)]
        name: String,
        #[arg(long)]
        query: String,
        #[arg(long)]
        color: Option<String>,
    },
}

#[derive(Clone, Subcommand)]
pub enum EditCommands {
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
    /// Edit a label
    Label {
        #[arg(long)]
        label_id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        color: Option<String>,
    },
    /// Edit a filter
    Filter {
        #[arg(long)]
        filter_id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        query: Option<String>,
        #[arg(long)]
        color: Option<String>,
    },
}

#[derive(Clone, Subcommand)]
pub enum CompleteCommands {
    /// Complete a task
    Task {
        #[arg(long)]
        task_id: String,
    },
}

#[derive(Clone, Subcommand)]
pub enum ReopenCommands {
    /// Reopen a task
    Task {
        #[arg(long)]
        task_id: String,
    },
}

#[derive(Clone, Subcommand)]
pub enum DeleteCommands {
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
pub enum MoveCommands {
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
pub enum ReorderCommands {
    /// Reorder sections within a project
    Sections {
        /// Section IDs in desired order (comma-separated)
        #[arg(long)]
        section_ids: String,
    },
}

/// Sync commands - for synchronizing data with Todoist
#[derive(Parser)]
pub enum SyncCommands {
    /// 同步数据 (默认增量，--force 全量)
    Sync {
        #[arg(long)]
        force: bool,
    },
}

/// Cache commands - for managing local cache
#[derive(Parser)]
pub enum CacheCommands {
    /// 显示缓存状态
    Status,
    /// 清除缓存
    Clear,
}

pub mod error;
pub mod handlers;

pub use error::handle_error;

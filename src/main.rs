use clap::{Parser, Subcommand};
use serde_json::to_string_pretty;
use todorust::{
    api::TodoistClient,
    config::{init_config, load_config},
    error::TodoError,
    Formattable,
    OutputFormat,
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
    /// Get custom filters
    Filters {
        #[arg(long, short)]
        format: Option<OutputFormat>,
    },
    /// Create a new task
    Create {
        #[arg(long)]
        content: Option<String>,
        #[arg(long)]
        title: Option<String>,
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
    /// Complete a task
    Complete {
        #[arg(long)]
        task_id: String,
    },
    /// Reopen a task
    Reopen {
        #[arg(long)]
        task_id: String,
    },
    /// Initialize configuration
    Init {
        #[arg(long)]
        api_token: String,
    },
}

fn validate_priority(priority: u8) -> bool {
    (1..=4).contains(&priority)
}

fn handle_error(error: TodoError) {
    match &error {
        TodoError::ConfigNotFound => {
            eprintln!("Error: Configuration not found.");
            eprintln!("Run: todorust init --api-token YOUR_TOKEN");
        }
        TodoError::Http(status, msg) => {
            eprintln!("Error: HTTP {} - {}", status, msg);
        }
        TodoError::Api(msg) => {
            eprintln!("API Error: {}", msg);
        }
        TodoError::Request(e) => {
            eprintln!("Request Error: {}", e);
        }
        TodoError::Config(msg) => {
            eprintln!("Config Error: {}", msg);
        }
        TodoError::InvalidInput(msg) => {
            eprintln!("Invalid Input: {}", msg);
        }
    }
    std::process::exit(1);
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Commands::Init { api_token } = cli.command {
        if let Err(e) = init_config(&api_token) {
            handle_error(e);
        }
        println!("Configuration initialized successfully!");
        return;
    }

    let result = async {
        let config = load_config()?;
        let client = TodoistClient::new(config.api_token);

        match cli.command {
            Commands::Tasks { filter, format } => {
                let output_format = format.unwrap_or(cli.format);
                let tasks = client.get_tasks(filter).await?;
                let output = tasks.format(&output_format);
                println!("{}", output);
            }
            Commands::Projects { format } => {
                let output_format = format.unwrap_or(cli.format);
                let projects = client.get_projects().await?;
                let output = projects.format(&output_format);
                println!("{}", output);
            }
            Commands::Filters { format } => {
                let output_format = format.unwrap_or(cli.format);
                let filters = client.get_filters().await?;
                let output = filters.format(&output_format);
                println!("{}", output);
            }
            Commands::Create {
                content,
                title,
                description,
                project_id,
                due_date,
                priority,
                labels,
                format,
            } => {
                let output_format = format.unwrap_or(cli.format);
                let content = title
                    .filter(|value| !value.trim().is_empty())
                    .or_else(|| content.filter(|value| !value.trim().is_empty()))
                    .ok_or_else(|| {
                        TodoError::InvalidInput("Task title/content cannot be empty".to_string())
                    })?;

                if let Some(p) = priority {
                    if !validate_priority(p) {
                        return Err(TodoError::InvalidInput(
                            "Priority must be between 1 and 4".to_string(),
                        ));
                    }
                }

                // Parse labels from comma-separated string
                let labels_vec = labels.and_then(|l| {
                    if l.is_empty() {
                        None
                    } else {
                        Some(l.split(',').map(|s| s.trim().to_string()).collect())
                    }
                });

                let task = client
                    .create_task(
                        &content,
                        description.filter(|value| !value.trim().is_empty()),
                        project_id,
                        due_date,
                        priority,
                        labels_vec,
                    )
                    .await?;
                // For single task, wrap in vec for formatting
                let output = vec![task].format(&output_format);
                println!("{}", output);
            }
            Commands::Complete { task_id } => {
                client.complete_task(&task_id).await?;
                println!("Task {} completed", task_id);
            }
            Commands::Reopen { task_id } => {
                client.reopen_task(&task_id).await?;
                println!("Task {} reopened", task_id);
            }
            Commands::Init { .. } => unreachable!(),
        }

        Ok::<(), TodoError>(())
    };

    if let Err(e) = result.await {
        handle_error(e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let args = vec!["todorust", "tasks"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(matches!(cli.command, Commands::Tasks { filter: None, .. }));
    }

    #[test]
    fn test_cli_with_filter() {
        let args = vec!["todorust", "tasks", "--filter", "project:Work"];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Tasks { filter, .. } = cli.command {
            assert_eq!(filter, Some("project:Work".to_string()));
        } else {
            panic!("Expected Tasks command");
        }
    }

    #[test]
    fn test_cli_with_format() {
        let args = vec!["todorust", "tasks", "--format", "checklist"];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Tasks { format, .. } = cli.command {
            assert_eq!(format, Some(OutputFormat::Checklist));
        } else {
            panic!("Expected Tasks command with format");
        }
    }

    #[test]
    fn test_global_format() {
        let args = vec!["todorust", "--format", "structured", "tasks"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.format, OutputFormat::Structured);
    }

    #[test]
    fn test_cli_create_with_title() {
        let args = vec![
            "todorust",
            "create",
            "--title",
            "New task",
            "--description",
            "Details",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Create {
            title,
            description,
            content,
            ..
        } = cli.command
        {
            assert_eq!(title, Some("New task".to_string()));
            assert_eq!(description, Some("Details".to_string()));
            assert_eq!(content, None);
        } else {
            panic!("Expected Create command with title");
        }
    }

    #[test]
    fn test_cli_create_with_content() {
        let args = vec!["todorust", "create", "--content", "Legacy task"];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Create {
            content, title, ..
        } = cli.command
        {
            assert_eq!(content, Some("Legacy task".to_string()));
            assert_eq!(title, None);
        } else {
            panic!("Expected Create command with content");
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

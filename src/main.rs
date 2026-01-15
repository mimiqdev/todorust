use clap::{Parser, Subcommand};
use todorust::{api::TodoistClient, config::{load_config, init_config}, error::TodoError};
use serde_json::to_string_pretty;

#[derive(Parser)]
#[command(name = "todorust")]
#[command(about = "CLI tool for Todoist API", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get tasks with optional filter
    Tasks {
        #[arg(long)]
        filter: Option<String>,
    },
    /// Get all projects
    Projects,
    /// Get custom filters
    Filters,
    /// Create a new task
    Create {
        #[arg(long)]
        content: String,
        #[arg(long)]
        project_id: Option<String>,
        #[arg(long)]
        due_date: Option<String>,
        #[arg(long)]
        priority: Option<u8>,
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
            Commands::Tasks { filter } => {
                let tasks = client.get_tasks(filter).await?;
                println!("{}", to_string_pretty(&tasks)?);
            }
            Commands::Projects => {
                let projects = client.get_projects().await?;
                println!("{}", to_string_pretty(&projects)?);
            }
            Commands::Filters => {
                let filters = client.get_filters().await?;
                println!("{}", to_string_pretty(&filters)?);
            }
            Commands::Create { content, project_id, due_date, priority } => {
                if content.trim().is_empty() {
                    return Err(TodoError::InvalidInput("Task content cannot be empty".to_string()));
                }

                if let Some(p) = priority {
                    if !validate_priority(p) {
                        return Err(TodoError::InvalidInput(
                            "Priority must be between 1 and 4".to_string()
                        ));
                    }
                }

                let task = client.create_task(&content, project_id, due_date, priority).await?;
                println!("{}", to_string_pretty(&task)?);
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
        assert!(matches!(cli.command, Commands::Tasks { filter: None }));
    }

    #[test]
    fn test_cli_with_filter() {
        let args = vec!["todorust", "tasks", "--filter", "project:Work"];
        let cli = Cli::try_parse_from(args).unwrap();
        if let Commands::Tasks { filter } = cli.command {
            assert_eq!(filter, Some("project:Work".to_string()));
        } else {
            panic!("Expected Tasks command");
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

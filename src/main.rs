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
}

#[tokio::main]
async fn main() -> Result<(), TodoError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { api_token } => {
            init_config(&api_token)?;
            println!("Configuration initialized successfully!");
        }
        _ => {
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
        }
    }

    Ok(())
}

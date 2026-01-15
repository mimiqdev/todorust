use clap::{Parser, Subcommand};

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
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Tasks { filter } => {
            println!("Get tasks with filter: {:?}", filter);
        }
        Commands::Projects => {
            println!("Get projects");
        }
        Commands::Filters => {
            println!("Get filters");
        }
        Commands::Create { content, project_id, due_date, priority } => {
            println!("Create task: {} {:?} {:?} {:?}", content, project_id, due_date, priority);
        }
        Commands::Complete { task_id } => {
            println!("Complete task: {}", task_id);
        }
        Commands::Reopen { task_id } => {
            println!("Reopen task: {}", task_id);
        }
    }
}

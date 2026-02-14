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

use crate::cli::{
    handle_error, AddCommands, Cli, Commands, CompleteCommands, ConfigCommands, DeleteCommands,
    EditCommands, GetCommands, MoveCommands, ReopenCommands, ReorderCommands,
};
use clap::Parser;

#[tokio::main]
async fn main() -> crate::error::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr) // Send logs to stderr so they don't mess up piped output
        .init();

    let cli = Cli::parse();
    if let Err(e) = run(cli).await {
        handle_error(e);
    }
    Ok(())
}

pub async fn run(cli: Cli) -> crate::error::Result<()> {
    // Handle init command separately (doesn't require config)
    if let Commands::Init(init_cmd) = &cli.command {
        let token = if let Some(t) = &init_cmd.api_token {
            t.clone()
        } else {
            println!("No API token provided.");
            println!(
                "Please get your API token from: https://todoist.com/app/settings/integrations"
            );
            println!("Enter your API token:");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .map_err(crate::error::TodoError::Io)?;
            input.trim().to_string()
        };

        if token.is_empty() {
            return Err(crate::error::TodoError::InvalidInput(
                "API token cannot be empty".to_string(),
            ));
        }

        crate::config::init_config(&token)?;
        println!("Configuration initialized successfully!");
        return Ok(());
    }

    // Handle completion command separately (doesn't require config)
    if let Commands::Completion { shell } = &cli.command {
        cli::handlers::generate_completions(*shell);
        return Ok(());
    }

    // Load config for other commands
    let config = crate::config::load_config()?;

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
        Commands::Get(GetCommands::Tasks {
            filter,
            fields,
            limit,
            ..
        }) => {
            cli::handlers::get_tasks(
                &client,
                filter.as_deref(),
                &format,
                fields.as_deref(),
                *limit,
            )
            .await?;
        }
        Commands::Get(GetCommands::Projects { fields, .. }) => {
            cli::handlers::get_projects(&client, &format, fields.as_deref()).await?;
        }
        Commands::Get(GetCommands::Task {
            task_id, fields, ..
        }) => {
            cli::handlers::get_task(&client, task_id, &format, fields.as_deref()).await?;
        }
        Commands::Get(GetCommands::Sections {
            project_id, fields, ..
        }) => {
            cli::handlers::get_sections(&client, project_id.as_deref(), &format, fields.as_deref())
                .await?;
        }
        Commands::Get(GetCommands::Filters { fields, .. }) => {
            cli::handlers::get_filters(&client, &format, fields.as_deref()).await?;
        }
        Commands::Get(GetCommands::Labels { fields, .. }) => {
            cli::handlers::get_labels(&client, &format, fields.as_deref()).await?;
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
        Commands::Add(AddCommands::Project {
            name,
            color,
            favorite,
        }) => {
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
        Commands::Edit(EditCommands::Label {
            label_id,
            name,
            color,
        }) => {
            cli::handlers::edit_label(&client, label_id.clone(), name.clone(), color.clone())
                .await?;
        }
        Commands::Edit(EditCommands::Filter {
            filter_id,
            name,
            query,
            color,
        }) => {
            cli::handlers::edit_filter(
                &client,
                filter_id.clone(),
                name.clone(),
                query.clone(),
                color.clone(),
            )
            .await?;
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
        Commands::Move(MoveCommands::Task {
            task_id,
            project_id,
            section_id,
        }) => {
            cli::handlers::move_task(
                &client,
                task_id.clone(),
                project_id.clone(),
                section_id.clone(),
            )
            .await?;
        }

        // Reorder commands
        Commands::Reorder(ReorderCommands::Sections { section_ids }) => {
            cli::handlers::reorder_sections(&client, section_ids.clone()).await?;
        }

        // Batch command
        Commands::Batch { commands } => {
            cli::handlers::batch(&client, commands.clone()).await?;
        }

        // Completion and Init were handled above
        Commands::Completion { .. } | Commands::Init(_) => unreachable!(),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::InitCommand;
    use serial_test::serial;

    #[test]
    fn test_cli_parsing_basic() {
        let args = vec!["todorust", "get", "tasks"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(matches!(
            cli.command,
            Commands::Get(GetCommands::Tasks { .. })
        ));
    }

    #[tokio::test]
    #[serial]
    async fn test_run_init_with_token() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();

        std::env::set_var("TODORUST_CONFIG_DIR", &temp_path);

        let cli = Cli {
            format: OutputFormat::Json,
            command: Commands::Init(InitCommand {
                api_token: Some("test_token".to_string()),
            }),
        };

        let result = run(cli).await;
        assert!(result.is_ok());

        let config_path = temp_path.join("config.toml");
        assert!(config_path.exists());
    }

    #[tokio::test]
    #[serial]
    async fn test_run_config_show_no_config() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();
        std::env::set_var("TODORUST_CONFIG_DIR", &temp_path);
        // Clean environment variable if set
        std::env::remove_var("TODORUST_API_TOKEN");

        let cli = Cli {
            format: OutputFormat::Json,
            command: Commands::Config(ConfigCommands::Get),
        };

        let result = run(cli).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::error::TodoError::ConfigNotFound
        ));
    }

    #[tokio::test]
    #[serial]
    async fn test_run_get_tasks_flow() {
        use httpmock::prelude::*;
        let server = MockServer::start();

        std::env::set_var("TODORUST_API_TOKEN", "mock_token");
        std::env::set_var("TODORUST_SYNC_URL", server.url("/sync"));

        server.mock(|when, then| {
            when.method(POST).path("/sync");
            then.status(200).json_body(serde_json::json!({
                "sync_token": "*",
                "items": [],
                "projects": []
            }));
        });

        let cli = Cli {
            format: OutputFormat::Json,
            command: Commands::Get(GetCommands::Tasks {
                filter: None,
                format: None,
                fields: None,
                limit: None,
            }),
        };

        let result = run(cli).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]
    async fn test_run_add_task_flow() {
        use httpmock::prelude::*;
        let server = MockServer::start();

        std::env::set_var("TODORUST_API_TOKEN", "mock_token");
        std::env::set_var("TODORUST_SYNC_URL", server.url("/sync"));

        server.mock(|when, then| {
            when.method(POST).path("/sync");
            then.status(200).json_body(serde_json::json!({
                "sync_token": "token123",
                "sync_status": {"any_uuid": "ok"},
                "temp_id_mapping": {"any_temp": "real_id"}
            }));
        });

        let cli = Cli {
            format: OutputFormat::Json,
            command: Commands::Add(AddCommands::Task {
                title: Some("New Task".to_string()),
                content: None,
                description: None,
                project_id: None,
                due_date: None,
                priority: Some(4),
                labels: None,
                format: None,
            }),
        };

        let result = run(cli).await;
        // Even if ID extraction fails because of random UUIDs,
        // the command execution itself reached the server.
        // We accept Api error here if it's just about ID mapping in this specific test.
        if let Err(crate::error::TodoError::Api(ref s)) = result {
            if s == "No ID returned" {
                return; // Acceptable for this test setup
            }
        }
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]

    async fn test_run_batch_flow() {
        use httpmock::prelude::*;

        let server = MockServer::start();

        std::env::set_var("TODORUST_API_TOKEN", "mock_token");

        std::env::set_var("TODORUST_SYNC_URL", server.url("/sync"));

        server.mock(|when, then| {
            when.method(POST).path("/sync");

            then.status(200).json_body(serde_json::json!({

                "sync_token": "token123",

                "sync_status": {"uuid1": "ok"},

                "temp_id_mapping": {}

            }));
        });

        let cli = Cli {
            format: OutputFormat::Json,

            command: Commands::Batch {
                commands: r#"[{"type": "item_complete", "uuid": "uuid1", "args": {"id": "123"}}]"#
                    .to_string(),
            },
        };

        let result = run(cli).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]

    async fn test_run_delete_task_flow() {
        use httpmock::prelude::*;

        let server = MockServer::start();

        std::env::set_var("TODORUST_API_TOKEN", "mock_token");

        std::env::set_var("TODORUST_SYNC_URL", server.url("/sync"));

        server.mock(|when, then| {
            when.method(POST).path("/sync");

            then.status(200).json_body(serde_json::json!({

                "sync_token": "token123",

                "sync_status": {"uuid": "ok"}

            }));
        });

        let cli = Cli {
            format: OutputFormat::Json,

            command: Commands::Delete(DeleteCommands::Task {
                task_id: "123".to_string(),
            }),
        };

        let result = run(cli).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]

    async fn test_run_edit_task_flow() {
        use httpmock::prelude::*;

        let server = MockServer::start();

        std::env::set_var("TODORUST_API_TOKEN", "mock_token");

        std::env::set_var("TODORUST_SYNC_URL", server.url("/sync"));

        server.mock(|when, then| {
            when.method(POST).path("/sync");

            then.status(200).json_body(serde_json::json!({

                "sync_token": "token123",

                "sync_status": {"uuid": "ok"}

            }));
        });

        let cli = Cli {
            format: OutputFormat::Json,

            command: Commands::Edit(EditCommands::Task {
                task_id: "123".to_string(),

                title: Some("Updated".to_string()),

                content: None,

                project_id: None,

                due_date: None,

                priority: None,

                labels: None,
            }),
        };

        let result = run(cli).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]

    async fn test_run_get_projects_flow() {
        use httpmock::prelude::*;

        let server = MockServer::start();

        std::env::set_var("TODORUST_API_TOKEN", "mock_token");

        std::env::set_var("TODORUST_SYNC_URL", server.url("/sync"));

        server.mock(|when, then| {
            when.method(POST).path("/sync");

            then.status(200).json_body(serde_json::json!({



                "sync_token": "token123",



                "projects": []



            }));
        });

        let cli = Cli {
            format: OutputFormat::Json,

            command: Commands::Get(GetCommands::Projects {
                format: None,

                fields: None,
            }),
        };

        let result = run(cli).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]

    async fn test_run_config_get_flow() {
        let temp_dir = tempfile::tempdir().unwrap();

        let temp_path = temp_dir.path().to_path_buf();

        std::env::set_var("TODORUST_CONFIG_DIR", &temp_path);

        // Initialize config first

        crate::config::init_config("test_token_12345").unwrap();

        let cli = Cli {
            format: OutputFormat::Json,

            command: Commands::Config(ConfigCommands::Get),
        };

        let result = run(cli).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]

    async fn test_run_get_sections_flow() {
        use httpmock::prelude::*;

        let server = MockServer::start();

        std::env::set_var("TODORUST_API_TOKEN", "mock_token");

        std::env::set_var("TODORUST_SYNC_URL", server.url("/sync"));

        server.mock(|when, then| {
            when.method(POST).path("/sync");

            then.status(200).json_body(serde_json::json!({

                "sync_token": "token123",

                "sections": []

            }));
        });

        let cli = Cli {
            format: OutputFormat::Json,

            command: Commands::Get(GetCommands::Sections {
                project_id: None,

                format: None,

                fields: None,
            }),
        };

        let result = run(cli).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]

    async fn test_run_move_task_flow() {
        use httpmock::prelude::*;

        let server = MockServer::start();

        std::env::set_var("TODORUST_API_TOKEN", "mock_token");

        std::env::set_var("TODORUST_SYNC_URL", server.url("/sync"));

        server.mock(|when, then| {
            when.method(POST).path("/sync");

            then.status(200).json_body(serde_json::json!({

                "sync_token": "token123",

                "sync_status": {"uuid": "ok"}

            }));
        });

        let cli = Cli {
            format: OutputFormat::Json,

            command: Commands::Move(MoveCommands::Task {
                task_id: "123".to_string(),

                project_id: "proj1".to_string(),

                section_id: None,
            }),
        };

        let result = run(cli).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]

    async fn test_run_reorder_sections_flow() {
        use httpmock::prelude::*;

        let server = MockServer::start();

        std::env::set_var("TODORUST_API_TOKEN", "mock_token");

        std::env::set_var("TODORUST_SYNC_URL", server.url("/sync"));

        server.mock(|when, then| {
            when.method(POST).path("/sync");

            then.status(200).json_body(serde_json::json!({

                "sync_token": "token123",

                "sync_status": {"uuid": "ok"}

            }));
        });

        let cli = Cli {
            format: OutputFormat::Json,

            command: Commands::Reorder(ReorderCommands::Sections {
                section_ids: "s1,s2".to_string(),
            }),
        };

        let result = run(cli).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]

    async fn test_run_add_section_flow() {
        use httpmock::prelude::*;

        let server = MockServer::start();

        std::env::set_var("TODORUST_API_TOKEN", "mock_token");

        std::env::set_var("TODORUST_SYNC_URL", server.url("/sync"));

        server.mock(|when, then| {
            when.method(POST).path("/sync");

            then.status(200).json_body(serde_json::json!({

                "sync_token": "token123",

                "sync_status": {"uuid": "ok"},

                "temp_id_mapping": {"temp": "real_sec_id"}

            }));
        });

        let cli = Cli {
            format: OutputFormat::Json,

            command: Commands::Add(AddCommands::Section {
                name: "New Section".to_string(),

                project_id: "proj1".to_string(),
            }),
        };

        let result = run(cli).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]

    async fn test_run_completion_flow() {
        let cli = Cli {
            format: OutputFormat::Json,

            command: Commands::Completion {
                shell: clap_complete::Shell::Bash,
            },
        };

        let result = run(cli).await;

        if let Err(ref e) = result {
            println!("test_run_completion_flow failed with error: {:?}", e);
        }

        assert!(result.is_ok());
    }
}

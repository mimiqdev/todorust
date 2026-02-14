use crate::error::Result;
use crate::formatter::{Formattable, OutputFormat};
use crate::sync::{Command, TodoistSyncClient};
use std::collections::HashMap;

pub async fn get_tasks(
    client: &TodoistSyncClient,
    filter: Option<&str>,
    format: &OutputFormat,
    fields: Option<&str>,
    limit: Option<usize>,
) -> Result<()> {
    // Get all tasks and projects to resolve project names
    let tasks = client.get_tasks().await?;
    let projects = client.get_projects().await?;

    // Build project name lookup
    let project_map: HashMap<&str, &str> = projects
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
    let mut filtered: Vec<crate::models::TaskOutput> = if let Some(f) = filter {
        let f_lower = f.to_lowercase();

        // Check for specific patterns like "p:1" or "priority:1"
        let priority_filter = if f_lower.starts_with("p:") {
            f_lower
                .strip_prefix("p:")
                .and_then(|s| s.parse::<u8>().ok())
        } else if f_lower.starts_with("priority:") {
            f_lower
                .strip_prefix("priority:")
                .and_then(|s| s.parse::<u8>().ok())
        } else {
            None
        };

        let status_filter = match f_lower.as_str() {
            "is:completed" | "completed" => Some(true),
            "is:active" | "active" | "incomplete" => Some(false),
            _ => None,
        };

        task_outputs
            .into_iter()
            .filter(|t| {
                if let Some(p) = priority_filter {
                    if t.priority == p {
                        return true;
                    }
                }

                if let Some(completed) = status_filter {
                    if t.is_completed == completed {
                        return true;
                    }
                }

                // Content or project name match
                t.content.to_lowercase().contains(&f_lower)
                    || t.project_name
                        .as_ref()
                        .map(|p| p.to_lowercase().contains(&f_lower))
                        .unwrap_or(false)
            })
            .collect()
    } else {
        task_outputs
    };

    // Apply limit if provided
    if let Some(l) = limit {
        filtered.truncate(l);
    }

    if format == &OutputFormat::Json && fields.is_some() {
        println!("{}", filtered.format_filtered(fields));
    } else {
        println!("{}", filtered.format(format));
    }
    Ok(())
}

pub async fn get_projects(
    client: &TodoistSyncClient,
    format: &OutputFormat,
    fields: Option<&str>,
) -> Result<()> {
    let projects = client.get_projects().await?;
    if format == &OutputFormat::Json && fields.is_some() {
        println!("{}", projects.format_filtered(fields));
    } else {
        println!("{}", projects.format(format));
    }
    Ok(())
}

pub async fn get_sections(
    client: &TodoistSyncClient,
    project_id: Option<&str>,
    format: &OutputFormat,
    fields: Option<&str>,
) -> Result<()> {
    let mut sections = client.get_sections().await?;

    // Filter by project_id if provided
    if let Some(pid) = project_id {
        sections.retain(|s| s.project_id == pid);
    }

    if format == &OutputFormat::Json && fields.is_some() {
        println!("{}", sections.format_filtered(fields));
    } else {
        println!("{}", sections.format(format));
    }
    Ok(())
}

pub async fn get_filters(
    client: &TodoistSyncClient,
    format: &OutputFormat,
    fields: Option<&str>,
) -> Result<()> {
    let filters = client.get_filters().await?;
    if format == &OutputFormat::Json && fields.is_some() {
        println!("{}", filters.format_filtered(fields));
    } else {
        println!("{}", filters.format(format));
    }
    Ok(())
}

pub async fn get_labels(
    client: &TodoistSyncClient,
    format: &OutputFormat,
    fields: Option<&str>,
) -> Result<()> {
    let labels = client.get_labels().await?;
    if format == &OutputFormat::Json && fields.is_some() {
        println!("{}", labels.format_filtered(fields));
    } else {
        println!("{}", labels.format(format));
    }
    Ok(())
}

pub async fn get_task(
    client: &TodoistSyncClient,
    task_id: &str,
    format: &OutputFormat,
    fields: Option<&str>,
) -> Result<()> {
    let tasks = client.get_tasks().await?;
    let projects = client.get_projects().await?;

    let project_map: HashMap<&str, &str> = projects
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

    let result = vec![task_output];
    if format == &OutputFormat::Json && fields.is_some() {
        println!("{}", result.format_filtered(fields));
    } else {
        println!("{}", result.format(format));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn add_task(
    client: &TodoistSyncClient,
    title: Option<String>,
    content: Option<String>,
    description: Option<String>,
    project_id: Option<String>,
    due_date: Option<String>,
    priority: Option<u8>,
    labels: Option<String>,
) -> Result<()> {
    let task_content = title
        .as_ref()
        .or(content.as_ref())
        .ok_or_else(|| {
            crate::error::TodoError::InvalidInput("Task title or content required".to_string())
        })?
        .clone();

    // Validate priority
    let validated_priority = if let Some(p) = priority {
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

    let response = serde_json::json!({
        "status": "success",
        "type": "task",
        "id": task_id,
        "content": task_content
    });
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

pub async fn add_section(
    client: &TodoistSyncClient,
    name: String,
    project_id: String,
) -> Result<()> {
    let section_id = client.add_section(&name, &project_id).await?;
    let response = serde_json::json!({
        "status": "success",
        "type": "section",
        "id": section_id,
        "name": name,
        "project_id": project_id
    });
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

pub async fn add_project(
    client: &TodoistSyncClient,
    name: String,
    color: Option<String>,
    favorite: bool,
) -> Result<()> {
    let project_id = client
        .add_project(&name, color.as_deref(), Some(favorite))
        .await?;
    let response = serde_json::json!({
        "status": "success",
        "type": "project",
        "id": project_id,
        "name": name
    });
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

pub async fn add_label(
    client: &TodoistSyncClient,
    name: String,
    color: Option<String>,
) -> Result<()> {
    let label_id = client.add_label(&name, color.as_deref()).await?;
    let response = serde_json::json!({
        "status": "success",
        "type": "label",
        "id": label_id,
        "name": name
    });
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

pub async fn add_filter(
    client: &TodoistSyncClient,
    name: String,
    query: String,
    color: Option<String>,
) -> Result<()> {
    let filter_id = client.add_filter(&name, &query, color.as_deref()).await?;
    let response = serde_json::json!({
        "status": "success",
        "type": "filter",
        "id": filter_id,
        "name": name,
        "query": query
    });
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn edit_task(
    client: &TodoistSyncClient,
    task_id: String,
    title: Option<String>,
    content: Option<String>,
    project_id: Option<String>,
    due_date: Option<String>,
    priority: Option<u8>,
    labels: Option<String>,
) -> Result<()> {
    let task_content = title.as_ref().or(content.as_ref()).map(|s| s.as_str());
    let labels_vec: Option<Vec<&str>> = labels
        .as_ref()
        .map(|l| l.split(',').map(|s| s.trim()).collect());

    // Validate priority
    if let Some(p) = priority {
        if !validate_priority(p) {
            return Err(crate::error::TodoError::InvalidInput(format!(
                "Invalid priority {}. Priority must be between 1 and 4.",
                p
            )));
        }
    }

    // If project_id is provided, move the task to the new project first
    if let Some(ref new_project_id) = project_id {
        let builder = crate::sync::CommandBuilder::new().item_move(&task_id, new_project_id, None);
        client.execute(builder).await?;
    }

    // Update task fields after move
    client
        .update_task(
            &task_id,
            task_content,
            None,
            priority,
            due_date.as_deref(),
            labels_vec,
        )
        .await?;

    let response = serde_json::json!({
        "status": "success",
        "action": "edit",
        "type": "task",
        "id": task_id
    });
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

pub async fn edit_project(
    client: &TodoistSyncClient,
    project_id: String,
    name: Option<String>,
) -> Result<()> {
    if name.is_none() {
        return Err(crate::error::TodoError::InvalidInput(
            "No fields to update. Provide at least --name.".to_string(),
        ));
    }
    let builder =
        crate::sync::CommandBuilder::new().project_update(&project_id, name.as_deref(), None, None);
    client.execute(builder).await?;
    let response = serde_json::json!({
        "status": "success",
        "action": "edit",
        "type": "project",
        "id": project_id
    });
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

pub async fn edit_section(
    client: &TodoistSyncClient,
    section_id: String,
    name: Option<String>,
) -> Result<()> {
    let new_name = name.ok_or_else(|| {
        crate::error::TodoError::InvalidInput(
            "No fields to update. Provide at least --name.".to_string(),
        )
    })?;
    client.update_section(&section_id, &new_name).await?;
    let response = serde_json::json!({
        "status": "success",
        "action": "edit",
        "type": "section",
        "id": section_id
    });
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

pub async fn edit_label(
    client: &TodoistSyncClient,
    label_id: String,
    name: Option<String>,
    color: Option<String>,
) -> Result<()> {
    if name.is_none() && color.is_none() {
        return Err(crate::error::TodoError::InvalidInput(
            "No fields to update. Provide at least --name or --color.".to_string(),
        ));
    }
    client
        .update_label(&label_id, name.as_deref(), color.as_deref())
        .await?;
    let response = serde_json::json!({
        "status": "success",
        "action": "edit",
        "type": "label",
        "id": label_id
    });
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

pub async fn edit_filter(
    client: &TodoistSyncClient,
    filter_id: String,
    name: Option<String>,
    query: Option<String>,
    color: Option<String>,
) -> Result<()> {
    if name.is_none() && query.is_none() && color.is_none() {
        return Err(crate::error::TodoError::InvalidInput(
            "Error: No fields to update. Provide at least --name, --query, or --color.".to_string(),
        ));
    }
    client
        .update_filter(
            &filter_id,
            name.as_deref(),
            query.as_deref(),
            color.as_deref(),
        )
        .await?;
    let response = serde_json::json!({
        "status": "success",
        "action": "edit",
        "type": "filter",
        "id": filter_id
    });
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

pub async fn complete_task(client: &TodoistSyncClient, task_id: String) -> Result<()> {
    client.complete_task(&task_id).await?;
    let response = serde_json::json!({
        "status": "success",
        "action": "complete",
        "id": task_id
    });
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

pub async fn reopen_task(client: &TodoistSyncClient, task_id: String) -> Result<()> {
    let builder = crate::sync::CommandBuilder::new().item_uncomplete(&task_id);
    client.execute(builder).await?;
    let response = serde_json::json!({
        "status": "success",
        "action": "reopen",
        "id": task_id
    });
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

pub async fn delete_task(client: &TodoistSyncClient, task_id: String) -> Result<()> {
    client.delete_task(&task_id).await?;
    let response = serde_json::json!({
        "status": "success",
        "action": "delete",
        "type": "task",
        "id": task_id
    });
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

pub async fn delete_project(client: &TodoistSyncClient, project_id: String) -> Result<()> {
    let builder = crate::sync::CommandBuilder::new().project_delete(&project_id);
    client.execute(builder).await?;
    let response = serde_json::json!({
        "status": "success",
        "action": "delete",
        "type": "project",
        "id": project_id
    });
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

pub async fn delete_section(client: &TodoistSyncClient, section_id: String) -> Result<()> {
    client.delete_section(&section_id).await?;
    let response = serde_json::json!({
        "status": "success",
        "action": "delete",
        "type": "section",
        "id": section_id
    });
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

pub async fn move_task(
    client: &TodoistSyncClient,
    task_id: String,
    project_id: String,
    section_id: Option<String>,
) -> Result<()> {
    let builder =
        crate::sync::CommandBuilder::new().item_move(&task_id, &project_id, section_id.as_deref());
    client.execute(builder).await?;
    let response = serde_json::json!({
        "status": "success",
        "action": "move",
        "id": task_id,
        "project_id": project_id,
        "section_id": section_id
    });
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

pub async fn reorder_sections(client: &TodoistSyncClient, section_ids: String) -> Result<()> {
    let sections: Vec<&str> = section_ids.split(',').map(|s| s.trim()).collect();
    let sections_with_order: Vec<(&str, i64)> = sections
        .iter()
        .enumerate()
        .map(|(i, id)| (*id, i as i64))
        .collect();
    client.reorder_sections(&sections_with_order).await?;
    let response = serde_json::json!({
        "status": "success",
        "action": "reorder",
        "type": "sections",
        "section_ids": sections
    });
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

pub async fn batch(client: &TodoistSyncClient, commands_json: String) -> Result<()> {
    let commands: Vec<Command> = serde_json::from_str(&commands_json)
        .map_err(|e| crate::error::TodoError::InvalidInput(format!("Invalid batch JSON: {}", e)))?;

    let response = client.execute_commands_with_status(&commands).await?;

    // For batch operations, we always output JSON to show the status and mappings
    println!("{}", serde_json::to_string_pretty(&response).unwrap());
    Ok(())
}

pub fn generate_completions(shell: clap_complete::Shell) {
    use clap::CommandFactory;
    let mut cmd = crate::cli::Cli::command();
    clap_complete::generate(shell, &mut cmd, "todorust", &mut std::io::stdout());
}

pub fn validate_priority(priority: u8) -> bool {
    (1..=4).contains(&priority)
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_get_tasks_handler() {
        let server = MockServer::start();
        let client = TodoistSyncClient::new_with_url("token".to_string(), server.url("/sync"));

        server.mock(|when, then| {
            when.method(POST).path("/sync");
            then.status(200).json_body(json!({
                "sync_token": "token123",
                "items": [
                    {
                        "id": "1",
                        "content": "Task 1",
                        "priority": 4,
                        "checked": false,
                        "added_at": "2024-01-01T00:00:00Z",
                        "child_order": 1
                    }
                ],
                "projects": []
            }));
        });

        let result = get_tasks(&client, None, &OutputFormat::Json, None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_projects_handler() {
        let server = MockServer::start();
        let client = TodoistSyncClient::new_with_url("token".to_string(), server.url("/sync"));

        server.mock(|when, then| {
            when.method(POST).path("/sync");
            then.status(200).json_body(json!({
                "sync_token": "token123",
                "projects": [
                    {
                        "id": "p1",
                        "name": "Project 1",
                        "color": "blue",
                        "child_order": 1,
                        "added_at": "2024-01-01T00:00:00Z"
                    }
                ]
            }));
        });

        let result = get_projects(&client, &OutputFormat::Json, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_task_handler() {
        let server = MockServer::start();
        let client = TodoistSyncClient::new_with_url("token".to_string(), server.url("/sync"));

        server.mock(|when, then| {
            when.method(POST).path("/sync");
            then.status(200).json_body(json!({
                "sync_token": "token123",
                "sync_status": {"uuid": "ok"},
                "temp_id_mapping": {"temp": "real_id"}
            }));
        });

        let result = add_task(
            &client,
            Some("New Task".to_string()),
            None,
            None,
            None,
            None,
            Some(4),
            None,
        )
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_batch_handler() {
        let server = MockServer::start();
        let client = TodoistSyncClient::new_with_url("token".to_string(), server.url("/sync"));

        server.mock(|when, then| {
            when.method(POST).path("/sync");
            then.status(200).json_body(json!({
                "sync_token": "token123",
                "sync_status": {"uuid1": "ok"},
                "temp_id_mapping": {"temp1": "real1"}
            }));
        });

        let commands_json = r#"[{"type": "item_add", "args": {"content": "Task 1"}}]"#;
        let result = batch(&client, commands_json.to_string()).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_priority() {
        assert!(validate_priority(1));
        assert!(validate_priority(4));
        assert!(!validate_priority(0));
        assert!(!validate_priority(5));
    }
}

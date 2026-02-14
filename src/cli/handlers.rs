use crate::error::Result;
use crate::formatter::{Formattable, OutputFormat};
use crate::sync::TodoistSyncClient;
use std::collections::HashMap;

pub async fn get_tasks(
    client: &TodoistSyncClient,
    filter: Option<&str>,
    format: &OutputFormat,
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

pub async fn get_projects(
    client: &TodoistSyncClient,
    format: &OutputFormat,
) -> Result<()> {
    let projects = client.get_projects().await?;
    println!("{}", projects.format(format));
    Ok(())
}

pub async fn get_sections(
    client: &TodoistSyncClient,
    project_id: Option<&str>,
    format: &OutputFormat,
) -> Result<()> {
    let mut sections = client.get_sections().await?;

    // Filter by project_id if provided
    if let Some(pid) = project_id {
        sections.retain(|s| s.project_id == pid);
    }

    println!("{}", sections.format(format));
    Ok(())
}

pub async fn get_filters(
    client: &TodoistSyncClient,
    format: &OutputFormat,
) -> Result<()> {
    let filters = client.get_filters().await?;
    println!("{}", filters.format(format));
    Ok(())
}

pub async fn get_labels(
    client: &TodoistSyncClient,
    format: &OutputFormat,
) -> Result<()> {
    let labels = client.get_labels().await?;
    println!("{}", labels.format(format));
    Ok(())
}

pub async fn get_task(
    client: &TodoistSyncClient,
    task_id: &str,
    format: &OutputFormat,
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

    println!("{}", vec![task_output].format(format));
    Ok(())
}

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

    // Validate priority - return error if invalid
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

    println!("Task created with ID: {}", task_id);
    Ok(())
}

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

    // Validate priority - return error if invalid
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

    println!("Task {} updated", task_id);
    Ok(())
}

pub async fn add_section(
    client: &TodoistSyncClient,
    name: String,
    project_id: String,
) -> Result<()> {
    let section_id = client.add_section(&name, &project_id).await?;
    println!("Section created with ID: {}", section_id);
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
    println!("Project created with ID: {}", project_id);
    Ok(())
}

pub async fn add_label(
    client: &TodoistSyncClient,
    name: String,
    color: Option<String>,
) -> Result<()> {
    let label_id = client.add_label(&name, color.as_deref()).await?;
    println!("Label created with ID: {}", label_id);
    Ok(())
}

pub async fn add_filter(
    client: &TodoistSyncClient,
    name: String,
    query: String,
    color: Option<String>,
) -> Result<()> {
    let filter_id = client.add_filter(&name, &query, color.as_deref()).await?;
    println!("Filter created with ID: {}", filter_id);
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
    let builder = crate::sync::CommandBuilder::new().project_update(
        &project_id,
        name.as_deref(),
        None,
        None,
    );
    client.execute(builder).await?;
    println!("Project {} updated", project_id);
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
    println!("Section {} updated", section_id);
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
    println!("Label {} updated", label_id);
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
    println!("Filter {} updated", filter_id);
    Ok(())
}

pub async fn complete_task(client: &TodoistSyncClient, task_id: String) -> Result<()> {
    client.complete_task(&task_id).await?;
    println!("Task {} completed", task_id);
    Ok(())
}

pub async fn reopen_task(client: &TodoistSyncClient, task_id: String) -> Result<()> {
    let builder = crate::sync::CommandBuilder::new().item_reopen(&task_id);
    client.execute(builder).await?;
    println!("Task {} reopened", task_id);
    Ok(())
}

pub async fn delete_task(client: &TodoistSyncClient, task_id: String) -> Result<()> {
    client.delete_task(&task_id).await?;
    println!("Task {} deleted", task_id);
    Ok(())
}

pub async fn delete_project(client: &TodoistSyncClient, project_id: String) -> Result<()> {
    let builder = crate::sync::CommandBuilder::new().project_delete(&project_id);
    client.execute(builder).await?;
    println!("Project {} deleted", project_id);
    Ok(())
}

pub async fn delete_section(client: &TodoistSyncClient, section_id: String) -> Result<()> {
    client.delete_section(&section_id).await?;
    println!("Section {} deleted", section_id);
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
    println!("Task {} moved to project {}", task_id, project_id);
    if let Some(sid) = section_id {
        println!("Task {} moved to section {}", task_id, sid);
    }
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
    println!("Sections reordered: {}", section_ids);
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

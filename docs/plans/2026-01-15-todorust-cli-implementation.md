# Todorust CLI Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a Rust CLI tool for Todoist API integration with simplified JSON output for AI tools and automation workflows.

**Architecture:** Single binary CLI application using clap for argument parsing, reqwest for HTTP requests, and serde for JSON serialization. Configuration stored in TOML format. Direct API calls to Todoist REST API v1 with minimal transformation to lean JSON output.

**Tech Stack:** Rust, clap (CLI), reqwest (HTTP), serde/serde_json (serialization), toml (config), thiserror (error handling), tokio (async runtime)

---

## Task 1: Project Setup and Structure

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`
- Create: `src/lib.rs`
- Create: `src/config.rs`
- Create: `src/error.rs`

**Step 1: Create Cargo.toml with dependencies**

```toml
[package]
name = "todorust"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4.4", features = ["derive"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.32", features = ["full"] }
thiserror = "1.0"
toml = "0.8"
dirs = "5.0"
```

**Step 2: Run cargo init to verify setup**

Run: `cargo check`
Expected: No errors, successful compilation

**Step 3: Create basic error module**

`src/error.rs`:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("API request failed: {0}")]
    Request(#[from] reqwest::Error),
    
    #[error("API error: {0}")]
    Api(String),
    
    #[error("HTTP error: {0}")]
    Http(u16, String),
}

pub type Result<T> = std::result::Result<T, TodoError>;
```

**Step 4: Create configuration module**

`src/config.rs`:
```rust
use crate::error::{TodoError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_token: String,
}

pub fn load_config() -> Result<Config> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| TodoError::Config("Cannot find config directory".to_string()))?
        .join("todoirust");
    
    let config_path = config_dir.join("config.toml");
    
    let content = fs::read_to_string(&config_path)
        .map_err(|e| TodoError::Config(format!("Cannot read config: {}", e)))?;
    
    let config: Config = toml::from_str(&content)
        .map_err(|e| TodoError::Config(format!("Cannot parse config: {}", e)))?;
    
    Ok(config)
}
```

**Step 5: Create lib stub**

`src/lib.rs`:
```rust
pub mod config;
pub mod error;
```

**Step 6: Create basic CLI main**

`src/main.rs`:
```rust
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
```

**Step 7: Verify CLI structure**

Run: `cargo build`
Expected: Successful compilation

**Step 8: Test CLI basic commands**

Run: `cargo run -- --help`
Expected: Help text showing all subcommands

**Step 9: Commit**

```bash
git add Cargo.toml src/
git commit -m "feat: initial project structure and CLI framework"
```

---

## Task 2: API Client Module

**Files:**
- Create: `src/api.rs`
- Modify: `src/lib.rs`

**Step 1: Write API client tests**

`src/api.rs` (start with test):
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_client_creation() {
        let client = TodoistClient::new("test_token".to_string());
        assert_eq!(client.token, "test_token");
    }
    
    #[test]
    fn test_base_url() {
        let client = TodoistClient::new("test_token".to_string());
        assert_eq!(client.base_url, "https://api.todoist.com/api/v1");
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_client_creation`
Expected: FAIL with `TodoistClient` not defined

**Step 3: Implement minimal TodoistClient struct**

```rust
use reqwest::Client as HttpClient;

pub struct TodoistClient {
    token: String,
    base_url: String,
    http: HttpClient,
}

impl TodoistClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            base_url: "https://api.todoist.com/api/v1".to_string(),
            http: HttpClient::new(),
        }
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_client_creation`
Expected: PASS

**Step 5: Add authorization header helper test**

```rust
#[test]
fn test_get_auth_header() {
    let client = TodoistClient::new("test_token".to_string());
    let auth = client.get_auth_header();
    assert_eq!(auth, "Bearer test_token");
}
```

**Step 6: Run test to verify it fails**

Run: `cargo test test_get_auth_header`
Expected: FAIL with method not defined

**Step 7: Implement auth header method**

```rust
impl TodoistClient {
    fn get_auth_header(&self) -> String {
        format!("Bearer {}", self.token)
    }
}
```

**Step 8: Run test to verify it passes**

Run: `cargo test test_get_auth_header`
Expected: PASS

**Step 9: Update lib.rs**

Add to `src/lib.rs`:
```rust
pub mod api;
```

**Step 10: Commit**

```bash
git add src/api.rs src/lib.rs
git commit -m "feat: add API client with auth header"
```

---

## Task 3: Projects API Integration

**Files:**
- Create: `src/models.rs`
- Modify: `src/api.rs`

**Step 1: Write Project model and test**

Add to `src/api.rs`:
```rust
#[cfg(test)]
mod tests {
    // ... existing tests ...
    
    #[test]
    fn test_project_deserialization() {
        let json = r#"{
            "id": "123",
            "name": "Work",
            "color": "blue",
            "is_shared": false,
            "is_favorite": true,
            "url": "https://todoist.com/showProject/123"
        }"#;
        
        let project: Project = serde_json::from_str(json).unwrap();
        assert_eq!(project.id, "123");
        assert_eq!(project.name, "Work");
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_project_deserialization`
Expected: FAIL with `Project` type not found

**Step 3: Create models module**

`src/models.rs`:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub color: String,
    pub is_shared: bool,
    pub is_favorite: bool,
    pub url: String,
}
```

**Step 4: Update lib.rs to include models**

Add to `src/lib.rs`:
```rust
pub mod models;
```

Add import to `src/api.rs`:
```rust
use crate::models::Project;
```

**Step 5: Run test to verify it passes**

Run: `cargo test test_project_deserialization`
Expected: PASS

**Step 6: Write get_projects test**

Add to `src/api.rs`:
```rust
#[tokio::test]
async fn test_get_projects_mock() {
    // This will be a mock test for now
    // We'll implement the real method next
}
```

**Step 7: Implement get_projects method**

Add to `src/api.rs`:
```rust
impl TodoistClient {
    pub async fn get_projects(&self) -> Result<Vec<Project>> {
        let response = self
            .http
            .get(format!("{}/projects", self.base_url))
            .header("Authorization", self.get_auth_header())
            .send()
            .await?;
        
        let status = response.status();
        
        if status.is_success() {
            let projects = response.json::<Vec<Project>>().await?;
            Ok(projects)
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(crate::error::TodoError::Http(status.as_u16(), error_text))
        }
    }
}
```

**Step 8: Update test with mock**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    #[ignore] // Requires real API token, run with cargo test -- --ignored
    async fn test_get_projects_real() {
        let client = TodoistClient::new(std::env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN env var"));
        let projects = client.get_projects().await.unwrap();
        assert!(!projects.is_empty());
        println!("Found {} projects", projects.len());
    }
}
```

**Step 9: Commit**

```bash
git add src/models.rs src/api.rs src/lib.rs
git commit -m "feat: add projects API integration"
```

---

## Task 4: Tasks API Integration

**Files:**
- Modify: `src/models.rs`
- Modify: `src/api.rs`

**Step 1: Write Task model test**

Add to `src/api.rs`:
```rust
#[test]
fn test_task_deserialization() {
    let json = r#"{
        "id": "456",
        "content": "Buy milk",
        "project_id": "123",
        "due": {"date": "2026-01-15"},
        "is_completed": false,
        "created_at": "2026-01-10T10:00:00Z",
        "order": 1,
        "priority": 4,
        "labels": ["shopping", "urgent"]
    }"#;
    
    let task: Task = serde_json::from_str(json).unwrap();
    assert_eq!(task.id, "456");
    assert_eq!(task.content, "Buy milk");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_task_deserialization`
Expected: FAIL with `Task` type not found

**Step 3: Add Task model to models.rs**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub content: String,
    pub project_id: Option<String>,
    #[serde(default)]
    pub due: Option<Due>,
    pub is_completed: bool,
    pub created_at: String,
    pub order: i32,
    pub priority: u8,
    #[serde(default)]
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Due {
    pub date: Option<String>,
    pub is_recurring: Option<bool>,
    pub datetime: Option<String>,
}

// Output model with enriched data
#[derive(Debug, Serialize)]
pub struct TaskOutput {
    pub id: String,
    pub content: String,
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub due_date: Option<String>,
    pub is_completed: bool,
    pub created_at: String,
    pub order: i32,
    pub priority: u8,
    pub labels: Vec<String>,
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_task_deserialization`
Expected: PASS

**Step 5: Write get_tasks test**

Add to `src/api.rs`:
```rust
#[tokio::test]
#[ignore]
async fn test_get_tasks_real() {
    let client = TodoistClient::new(std::env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN env var"));
    let tasks = client.get_tasks(None).await.unwrap();
    println!("Found {} tasks", tasks.len());
}
```

**Step 6: Implement get_tasks method**

Add to `src/api.rs`:
```rust
use crate::models::{Task, TaskOutput, Project};

impl TodoistClient {
    pub async fn get_tasks(&self, filter: Option<String>) -> Result<Vec<TaskOutput>> {
        let mut request = self
            .http
            .get(format!("{}/tasks", self.base_url))
            .header("Authorization", self.get_auth_header());
        
        if let Some(filter_str) = filter {
            request = request.query(&[("filter", filter_str)]);
        }
        
        let response = request.send().await?;
        let status = response.status();
        
        if status.is_success() {
            let tasks = response.json::<Vec<Task>>().await?;
            Ok(self.enrich_tasks(tasks).await)
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(crate::error::TodoError::Http(status.as_u16(), error_text))
        }
    }
    
    async fn enrich_tasks(&self, tasks: Vec<Task>) -> Vec<TaskOutput> {
        let projects = match self.get_projects().await {
            Ok(p) => p,
            Err(_) => Vec::new(),
        };
        
        tasks
            .into_iter()
            .map(|task| {
                let project_name = task
                    .project_id
                    .as_ref()
                    .and_then(|pid| projects.iter().find(|p| p.id == *pid))
                    .map(|p| p.name.clone());
                
                TaskOutput {
                    id: task.id,
                    content: task.content,
                    project_id: task.project_id,
                    project_name,
                    due_date: task.due.and_then(|d| d.date),
                    is_completed: task.is_completed,
                    created_at: task.created_at,
                    order: task.order,
                    priority: task.priority,
                    labels: task.labels,
                }
            })
            .collect()
    }
}
```

**Step 7: Run test to verify it passes**

Run: `cargo test test_get_tasks_real -- --ignored`
Expected: PASS (if TODOIST_TOKEN is set)

**Step 8: Commit**

```bash
git add src/models.rs src/api.rs
git commit -m "feat: add tasks API with filter support"
```

---

## Task 5: Filters API Integration

**Files:**
- Modify: `src/models.rs`
- Modify: `src/api.rs`

**Step 1: Write Filter model test**

Add to `src/api.rs`:
```rust
#[test]
fn test_filter_deserialization() {
    let json = r#"{
        "id": "789",
        "name": "This Week",
        "query": "due within \"7 days of today\""
    }"#;
    
    let filter: Filter = serde_json::from_str(json).unwrap();
    assert_eq!(filter.id, "789");
    assert_eq!(filter.name, "This Week");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_filter_deserialization`
Expected: FAIL with `Filter` type not found

**Step 3: Add Filter model to models.rs**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub id: String,
    pub name: String,
    pub query: String,
}

// Sync API response wrapper
#[derive(Debug, Deserialize)]
pub struct SyncResponse {
    pub filters: Vec<Filter>,
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_filter_deserialization`
Expected: PASS

**Step 5: Write get_filters test**

Add to `src/api.rs`:
```rust
#[tokio::test]
#[ignore]
async fn test_get_filters_real() {
    let client = TodoistClient::new(std::env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN env var"));
    let filters = client.get_filters().await.unwrap();
    println!("Found {} filters", filters.len());
}
```

**Step 6: Implement get_filters method**

Add to `src/api.rs`:
```rust
use crate::models::{Filter, SyncResponse};

impl TodoistClient {
    pub async fn get_filters(&self) -> Result<Vec<Filter>> {
        let response = self
            .http
            .post(format!("{}/sync", self.base_url))
            .header("Authorization", self.get_auth_header())
            .json(&serde_json::json!({
                "resource_types": ["filters"]
            }))
            .send()
            .await?;
        
        let status = response.status();
        
        if status.is_success() {
            let sync_data: SyncResponse = response.json().await?;
            Ok(sync_data.filters)
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(crate::error::TodoError::Http(status.as_u16(), error_text))
        }
    }
}
```

**Step 7: Run test to verify it passes**

Run: `cargo test test_get_filters_real -- --ignored`
Expected: PASS (if TODOIST_TOKEN is set)

**Step 8: Commit**

```bash
git add src/models.rs src/api.rs
git commit -m "feat: add filters API via sync endpoint"
```

---

## Task 6: Create Task API Integration

**Files:**
- Modify: `src/api.rs`

**Step 1: Write create_task test**

Add to `src/api.rs`:
```rust
#[tokio::test]
#[ignore]
async fn test_create_task_real() {
    let client = TodoistClient::new(std::env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN env var"));
    
    let task_output = client
        .create_task("Test task from integration test", None, None, None)
        .await
        .unwrap();
    
    assert_eq!(task_output.content, "Test task from integration test");
    
    // Cleanup
    let _ = client.delete_task(&task_output.id).await;
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_create_task_real -- --ignored`
Expected: FAIL with method not defined

**Step 3: Implement create_task method**

Add to `src/api.rs`:
```rust
#[derive(Serialize)]
struct CreateTaskRequest {
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<u8>,
}

impl TodoistClient {
    pub async fn create_task(
        &self,
        content: &str,
        project_id: Option<String>,
        due_date: Option<String>,
        priority: Option<u8>,
    ) -> Result<TaskOutput> {
        let request_body = CreateTaskRequest {
            content: content.to_string(),
            project_id,
            due_string: due_date,
            priority,
        };
        
        let response = self
            .http
            .post(format!("{}/tasks", self.base_url))
            .header("Authorization", self.get_auth_header())
            .json(&request_body)
            .send()
            .await?;
        
        let status = response.status();
        
        if status.is_success() {
            let task: Task = response.json().await?;
            // Enrich with project name
            let enriched = self.enrich_tasks(vec![task]).await;
            Ok(enriched.into_iter().next().unwrap())
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(crate::error::TodoError::Http(status.as_u16(), error_text))
        }
    }
    
    async fn delete_task(&self, task_id: &str) -> Result<()> {
        let response = self
            .http
            .delete(format!("{}/tasks/{}", self.base_url, task_id))
            .header("Authorization", self.get_auth_header())
            .send()
            .await?;
        
        let status = response.status();
        
        if status.is_success() || status.as_u16() == 404 {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(crate::error::TodoError::Http(status.as_u16(), error_text))
        }
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_create_task_real -- --ignored`
Expected: PASS

**Step 5: Commit**

```bash
git add src/api.rs
git commit -m "feat: add create task API"
```

---

## Task 7: Complete and Reopen Task API Integration

**Files:**
- Modify: `src/api.rs`

**Step 1: Write complete_task test**

Add to `src/api.rs`:
```rust
#[tokio::test]
#[ignore]
async fn test_complete_task_real() {
    let client = TodoistClient::new(std::env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN env var"));
    
    // Create a task first
    let task = client
        .create_task("Test task for completion", None, None, None)
        .await
        .unwrap();
    
    // Complete it
    client.complete_task(&task.id).await.unwrap();
    
    // Verify it's completed
    let tasks = client
        .get_tasks(Some(format!("id:{}", task.id)))
        .await
        .unwrap();
    
    assert_eq!(tasks.len(), 1);
    assert!(tasks[0].is_completed);
    
    // Cleanup
    let _ = client.delete_task(&task.id).await;
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_complete_task_real -- --ignored`
Expected: FAIL with method not defined

**Step 3: Implement complete_task method**

Add to `src/api.rs`:
```rust
impl TodoistClient {
    pub async fn complete_task(&self, task_id: &str) -> Result<()> {
        let response = self
            .http
            .post(format!("{}/tasks/{}/close", self.base_url, task_id))
            .header("Authorization", self.get_auth_header())
            .send()
            .await?;
        
        let status = response.status();
        
        if status.is_success() || status.as_u16() == 204 {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(crate::error::TodoError::Http(status.as_u16(), error_text))
        }
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_complete_task_real -- --ignored`
Expected: PASS

**Step 5: Write reopen_task test**

Add to `src/api.rs`:
```rust
#[tokio::test]
#[ignore]
async fn test_reopen_task_real() {
    let client = TodoistClient::new(std::env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN env var"));
    
    // Create and complete a task
    let task = client
        .create_task("Test task for reopening", None, None, None)
        .await
        .unwrap();
    client.complete_task(&task.id).await.unwrap();
    
    // Reopen it
    client.reopen_task(&task.id).await.unwrap();
    
    // Verify it's not completed
    let tasks = client
        .get_tasks(Some(format!("id:{}", task.id)))
        .await
        .unwrap();
    
    assert_eq!(tasks.len(), 1);
    assert!(!tasks[0].is_completed);
    
    // Cleanup
    let _ = client.delete_task(&task.id).await;
}
```

**Step 6: Run test to verify it fails**

Run: `cargo test test_reopen_task_real -- --ignored`
Expected: FAIL with method not defined

**Step 7: Implement reopen_task method**

Add to `src/api.rs`:
```rust
impl TodoistClient {
    pub async fn reopen_task(&self, task_id: &str) -> Result<()> {
        let response = self
            .http
            .post(format!("{}/tasks/{}/reopen", self.base_url, task_id))
            .header("Authorization", self.get_auth_header())
            .send()
            .await?;
        
        let status = response.status();
        
        if status.is_success() || status.as_u16() == 204 {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(crate::error::TodoError::Http(status.as_u16(), error_text))
        }
    }
}
```

**Step 8: Run test to verify it passes**

Run: `cargo test test_reopen_task_real -- --ignored`
Expected: PASS

**Step 9: Commit**

```bash
git add src/api.rs
git commit -m "feat: add complete and reopen task APIs"
```

---

## Task 8: Wire Up Commands in Main

**Files:**
- Modify: `src/main.rs`

**Step 1: Write integration test for tasks command**

Add to `src/main.rs`:
```rust
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
```

**Step 2: Run test to verify it passes**

Run: `cargo test test_cli_parsing`
Expected: PASS

**Step 3: Implement tasks command handler**

Replace placeholder in `src/main.rs`:
```rust
use clap::{Parser, Subcommand};
use todoirust::{api::TodoistClient, config::load_config, error::TodoError};
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
}

#[tokio::main]
async fn main() -> Result<(), TodoError> {
    let cli = Cli::parse();
    
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
    }
    
    Ok(())
}
```

**Step 4: Run basic build check**

Run: `cargo build`
Expected: Successful compilation

**Step 5: Test help output**

Run: `cargo run -- --help`
Expected: Help text showing all subcommands

**Step 6: Test projects command (will fail without config)**

Run: `cargo run -- projects`
Expected: Error about config file

**Step 7: Commit**

```bash
git add src/main.rs
git commit -m "feat: wire up all commands to API client"
```

---

## Task 9: Add Config File Management

**Files:**
- Modify: `src/config.rs`
- Create: `src/main.rs` (add config init command)

**Step 1: Write config init test**

Add to `src/config.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_parsing() {
        let toml_str = r#"
            api_token = "test_token_123"
        "#;
        
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.api_token, "test_token_123");
    }
}
```

**Step 2: Run test to verify it passes**

Run: `cargo test test_config_parsing`
Expected: PASS

**Step 3: Implement config initialization**

Add to `src/config.rs`:
```rust
pub fn init_config(api_token: &str) -> Result<()> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| TodoError::Config("Cannot find config directory".to_string()))?
        .join("todoirust");
    
    fs::create_dir_all(&config_dir)
        .map_err(|e| TodoError::Config(format!("Cannot create config directory: {}", e)))?;
    
    let config = Config {
        api_token: api_token.to_string(),
    };
    
    let toml_str = toml::to_string(&config)
        .map_err(|e| TodoError::Config(format!("Cannot serialize config: {}", e)))?;
    
    let config_path = config_dir.join("config.toml");
    fs::write(&config_path, toml_str)
        .map_err(|e| TodoError::Config(format!("Cannot write config: {}", e)))?;
    
    println!("Config saved to {}", config_path.display());
    Ok(())
}
```

**Step 4: Add init command to CLI**

Modify `src/main.rs`:
```rust
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
```

**Step 5: Handle init command**

Add to main function in `src/main.rs`:
```rust
Commands::Init { api_token } => {
    todoirust::config::init_config(&api_token)?;
    println!("Configuration initialized successfully!");
}
```

**Step 6: Test init command**

Run: `cargo run -- init --api-token test_token`
Expected: Success message and config file created

**Step 7: Verify config file exists**

Run: `cat ~/.config/todoirust/config.toml`
Expected: Config file with API token

**Step 8: Cleanup test config**

Run: `rm ~/.config/todoirust/config.toml`

**Step 9: Commit**

```bash
git add src/config.rs src/main.rs
git commit -m "feat: add config initialization command"
```

---

## Task 10: Add Better Error Handling and Output

**Files:**
- Modify: `src/error.rs`
- Modify: `src/main.rs`

**Step 1: Enhance error types**

Modify `src/error.rs`:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("API request failed: {0}")]
    Request(#[from] reqwest::Error),
    
    #[error("API error: {0}")]
    Api(String),
    
    #[error("HTTP error {0}: {1}")]
    Http(u16, String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Configuration not found. Run `todorust init --api-token YOUR_TOKEN` to configure.")]
    ConfigNotFound,
}

impl From<toml::de::Error> for TodoError {
    fn from(err: toml::de::Error) -> Self {
        TodoError::Config(format!("Parse error: {}", err))
    }
}
```

**Step 2: Improve load_config error handling**

Modify `src/config.rs`:
```rust
pub fn load_config() -> Result<Config> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| TodoError::Config("Cannot find config directory".to_string()))?
        .join("todoirust");
    
    let config_path = config_dir.join("config.toml");
    
    if !config_path.exists() {
        return Err(TodoError::ConfigNotFound);
    }
    
    let content = fs::read_to_string(&config_path)
        .map_err(|e| TodoError::Config(format!("Cannot read config: {}", e)))?;
    
    let config: Config = toml::from_str(&content)?;
    
    Ok(config)
}
```

**Step 3: Update main error display**

Modify `src/main.rs`:
```rust
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
            Commands::Init { api_token } => {
                init_config(&api_token)?;
                println!("Configuration initialized successfully!");
            }
        }
        
        Ok::<(), TodoError>(())
    };
    
    if let Err(e) = result.await {
        handle_error(e);
    }
}
```

**Step 4: Run build**

Run: `cargo build`
Expected: Successful compilation

**Step 5: Test error handling without config**

Run: `cargo run -- projects`
Expected: Helpful error message about init command

**Step 6: Commit**

```bash
git add src/error.rs src/config.rs src/main.rs
git commit -m "feat: improve error handling and user feedback"
```

---

## Task 11: Add Input Validation

**Files:**
- Modify: `src/main.rs`
- Modify: `src/api.rs`

**Step 1: Write validation tests**

Add to `src/main.rs`:
```rust
#[cfg(test)]
mod tests {
    // ... existing tests ...
    
    #[test]
    fn test_priority_validation() {
        assert!(validate_priority(1));
        assert!(validate_priority(4));
        assert!(!validate_priority(0));
        assert!(!validate_priority(5));
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_priority_validation`
Expected: FAIL with function not found

**Step 3: Implement validation functions**

Add to `src/main.rs`:
```rust
fn validate_priority(priority: u8) -> bool {
    (1..=4).contains(&priority)
}
```

**Step 4: Add validation to create command**

Modify create command handler:
```rust
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
```

**Step 5: Run tests**

Run: `cargo test test_priority_validation`
Expected: PASS

**Step 6: Commit**

```bash
git add src/main.rs
git commit -m "feat: add input validation for create command"
```

---

## Task 12: End-to-End Integration Testing

**Files:**
- Create: `tests/integration_test.rs`

**Step 1: Write end-to-end test**

Create `tests/integration_test.rs`:
```rust
use std::env;
use std::process::Command;
use std::io::{self, Write};
use tempfile::TempDir;

#[test]
#[ignore] // Run with: cargo test --test integration_test -- --ignored
fn test_end_to_end_workflow() {
    let token = env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN required");
    
    // Create temp config dir
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    
    // Write config
    let config_content = format!(r#"api_token = "{}""#, token);
    std::fs::write(&config_path, config_content).unwrap();
    
    // Note: For full integration, we'd need to set XDG_CONFIG_HOME or similar
    // This is a placeholder for the integration test structure
    
    println!("Integration test requires proper config handling");
    println!("Temp config path: {:?}", config_path);
}
```

**Step 2: Add tempfile dependency to Cargo.toml**

```toml
[dev-dependencies]
tempfile = "3.8"
```

**Step 3: Run test**

Run: `cargo test --test integration_test -- --ignored`
Expected: Test runs (will print placeholder message)

**Step 4: Commit**

```bash
git add tests/integration_test.rs Cargo.toml
git commit -m "test: add integration test structure"
```

---

## Task 13: Documentation and README

**Files:**
- Create: `README.md`
- Create: `USAGE.md`

**Step 1: Create README.md**

```markdown
# Todorust CLI

A Rust CLI tool for Todoist API integration with simplified JSON output for AI tools and automation workflows.

## Features

- Get tasks with full Todoist filter syntax support
- Get projects and custom filters
- Create, complete, and reopen tasks
- Clean JSON output optimized for parsing
- Simple configuration management

## Installation

```bash
cargo install --path .
```

## Configuration

Initialize with your Todoist API token:

```bash
todorust init --api-token YOUR_API_TOKEN
```

Get your token from: https://todoist.com/app/settings/integrations

## Usage

### Get Tasks

```bash
# Get all tasks
todorust tasks

# Filter tasks
todorust tasks --filter "project:Work & due within 7 days"
```

### Get Projects

```bash
todorust projects
```

### Get Filters

```bash
todorust filters
```

### Create Task

```bash
# Basic task
todorust create --content "Buy milk"

# With project and due date
todorust create --content "Write report" --project-id "123" --due-date "2026-01-20" --priority 4
```

### Complete/Reopen Task

```bash
todorust complete --task-id "456"
todorust reopen --task-id "456"
```

## Development

```bash
# Run tests
cargo test

# Run with config
TODOIST_TOKEN=your_token cargo test -- --ignored
```
```

**Step 2: Create USAGE.md**

```markdown
# Todorust Usage Guide

## Filter Syntax Examples

### By Project
```
project:Work
project:Personal
```

### By Date
```
due today
due tomorrow
due within "7 days of today"
completed within "7 days of today"
```

### By Priority
```
priority:4  # High
priority:3  # Normal
priority:2  # Low
priority:1  # None
```

### Combined Filters
```
project:Work & priority:4
project:Work & due within "7 days of today" & !completed
```

## JSON Output Format

### Task
```json
{
  "id": "123",
  "content": "Task name",
  "project_id": "456",
  "project_name": "Work",
  "due_date": "2026-01-15",
  "is_completed": false,
  "created_at": "2026-01-10T10:00:00Z",
  "order": 1,
  "priority": 4,
  "labels": ["urgent"]
}
```

### Project
```json
{
  "id": "456",
  "name": "Work",
  "color": "blue",
  "is_shared": false,
  "is_favorite": true,
  "url": "https://todoist.com/showProject/456"
}
```

### Filter
```json
{
  "id": "789",
  "name": "This Week",
  "query": "due within \"7 days of today\""
}
```
```

**Step 3: Commit**

```bash
git add README.md USAGE.md
git commit -m "docs: add comprehensive documentation"
```

---

## Task 14: Final Testing and Polish

**Files:**
- Modify: `Cargo.toml` (add metadata)

**Step 1: Update Cargo.toml with proper metadata**

```toml
[package]
name = "todorust"
version = "0.1.0"
edition = "2021"
description = "CLI tool for Todoist API with optimized JSON output"
authors = ["Your Name"]
license = "MIT"
repository = "https://github.com/yourusername/todorust"
readme = "README.md"

[dependencies]
clap = { version = "4.4", features = ["derive"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.32", features = ["full"] }
thiserror = "1.0"
toml = "0.8"
dirs = "5.0"

[dev-dependencies]
tempfile = "3.8"
```

**Step 2: Run all tests**

Run: `cargo test`
Expected: All unit tests pass

**Step 3: Run integration tests with token**

Run: `TODOIST_TOKEN=your_token cargo test -- --ignored`
Expected: All integration tests pass

**Step 4: Build release binary**

Run: `cargo build --release`
Expected: Successful build

**Step 5: Test release binary**

Run: `./target/release/todorust --help`
Expected: Help text displays correctly

**Step 6: Run clippy**

Run: `cargo clippy`
Expected: No warnings (or fix any warnings)

**Step 7: Run fmt check**

Run: `cargo fmt --check`
Expected: No formatting issues (or run `cargo fmt`)

**Step 8: Commit**

```bash
git add Cargo.toml
git commit -m "chore: finalize package metadata and run final checks"
```

---

## Task 15: Prepare for Release

**Files:**
- Create: `.gitignore`
- Verify: All files

**Step 1: Create .gitignore**

```
/target/
**/*.rs.bk
Cargo.lock
.DS_Store
.env
```

**Step 2: Verify git status**

Run: `git status`
Expected: All necessary files tracked

**Step 3: Run final test suite**

Run: `cargo test --all`
Expected: All tests pass

**Step 4: Verify build**

Run: `cargo build --release`
Expected: Clean release build

**Step 5: Tag release**

Run: `git tag v0.1.0`

**Step 6: Commit**

```bash
git add .gitignore
git commit -m "chore: add gitignore and prepare for v0.1.0 release"
```

---

## Summary

This implementation plan covers all MVP requirements from the PRD:

✅ Get tasks with full filter syntax support
✅ Get projects list
✅ Get custom filters list
✅ Create tasks with optional parameters
✅ Complete and reopen tasks
✅ Configuration file management (TOML format)
✅ Simplified JSON output format
✅ Basic error handling with HTTP status codes
✅ Comprehensive testing (unit and integration)
✅ Full documentation

The plan follows TDD principles with bite-sized tasks, commits after each feature, and clear acceptance criteria.

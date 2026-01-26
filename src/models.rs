use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub color: String,
    pub is_shared: bool,
    pub is_favorite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub content: String,
    #[serde(default)]
    pub description: Option<String>,
    pub project_id: Option<String>,
    #[serde(default)]
    pub due: Option<Due>,
    #[serde(alias = "checked")]
    pub is_completed: bool,
    #[serde(alias = "added_at")]
    pub created_at: String,
    #[serde(alias = "child_order")]
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

#[derive(Debug, Serialize)]
pub struct TaskOutput {
    pub id: String,
    pub content: String,
    pub description: Option<String>,
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub due_date: Option<String>,
    pub is_recurring: bool,
    pub is_completed: bool,
    pub created_at: String,
    pub order: i32,
    pub priority: u8,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub id: String,
    pub name: String,
    pub query: String,
}

#[derive(Debug, Deserialize)]
pub struct SyncResponse {
    pub filters: Vec<Filter>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectsResponse {
    pub results: Vec<Project>,
}

#[derive(Debug, Deserialize)]
pub struct TasksResponse {
    pub results: Vec<Task>,
}

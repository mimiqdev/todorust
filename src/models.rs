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

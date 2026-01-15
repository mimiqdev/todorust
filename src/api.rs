use reqwest::Client as HttpClient;

use crate::models::{Project, Task, TaskOutput};

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

    fn get_auth_header(&self) -> String {
        format!("Bearer {}", self.token)
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>, crate::error::TodoError> {
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

    pub async fn get_tasks(&self, filter: Option<String>) -> Result<Vec<TaskOutput>, crate::error::TodoError> {
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

    #[test]
    fn test_get_auth_header() {
        let client = TodoistClient::new("test_token".to_string());
        let auth = client.get_auth_header();
        assert_eq!(auth, "Bearer test_token");
    }

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

        let task: crate::models::Task = serde_json::from_str(json).unwrap();
        assert_eq!(task.id, "456");
        assert_eq!(task.content, "Buy milk");
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_projects_real() {
        let client = TodoistClient::new(std::env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN env var"));
        let projects = client.get_projects().await.unwrap();
        assert!(!projects.is_empty());
        println!("Found {} projects", projects.len());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_tasks_real() {
        let client = TodoistClient::new(std::env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN env var"));
        let tasks = client.get_tasks(None).await.unwrap();
        println!("Found {} tasks", tasks.len());
    }
}

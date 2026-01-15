use reqwest::Client as HttpClient;
use serde::Serialize;

use crate::models::{Filter, Project, ProjectsResponse, SyncResponse, Task, TaskOutput, TasksResponse};
use crate::error::TodoError;

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
        let response_text = response.text().await?;

        if !status.is_success() {
            return Err(TodoError::Http(status.as_u16(), response_text));
        }

        let projects_response: ProjectsResponse = serde_json::from_str(&response_text)
            .map_err(|e| TodoError::Api(format!("Failed to parse projects response: {}\nResponse: {}", e, response_text)))?;
        Ok(projects_response.results)
    }

    pub async fn get_tasks(
        &self,
        filter: Option<String>,
    ) -> Result<Vec<TaskOutput>, crate::error::TodoError> {
        let mut request = self
            .http
            .get(format!("{}/tasks", self.base_url))
            .header("Authorization", self.get_auth_header());

        if let Some(filter_str) = filter {
            request = request.query(&[("filter", filter_str)]);
        }

        let response = request.send().await?;
        let status = response.status();
        let response_text = response.text().await?;

        if !status.is_success() {
            return Err(TodoError::Http(status.as_u16(), response_text));
        }

        let tasks_response: TasksResponse = serde_json::from_str(&response_text)
            .map_err(|e| TodoError::Api(format!("Failed to parse tasks response: {}\nResponse: {}", e, response_text)))?;
        Ok(self.enrich_tasks(tasks_response.results).await)
    }

    async fn enrich_tasks(&self, tasks: Vec<Task>) -> Vec<TaskOutput> {
        let projects = self.get_projects().await.unwrap_or_default();

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

    pub async fn get_filters(&self) -> Result<Vec<Filter>, crate::error::TodoError> {
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
        let response_text = response.text().await?;

        if !status.is_success() {
            return Err(TodoError::Http(status.as_u16(), response_text));
        }

        let sync_data: SyncResponse = serde_json::from_str(&response_text)
            .map_err(|e| TodoError::Api(format!("Failed to parse filters response: {}\nResponse: {}", e, response_text)))?;
        Ok(sync_data.filters)
    }

    pub async fn create_task(
        &self,
        content: &str,
        project_id: Option<String>,
        due_date: Option<String>,
        priority: Option<u8>,
    ) -> Result<TaskOutput, crate::error::TodoError> {
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
        let response_text = response.text().await?;

        if !status.is_success() {
            return Err(crate::error::TodoError::Http(status.as_u16(), response_text));
        }

        let task: Task = serde_json::from_str(&response_text)
            .map_err(|e| TodoError::Api(format!("Failed to parse task response: {}\nResponse: {}", e, response_text)))?;
        let enriched = self.enrich_tasks(vec![task]).await;
        Ok(enriched.into_iter().next().unwrap())
    }

    #[allow(dead_code)]
    async fn delete_task(&self, task_id: &str) -> Result<(), crate::error::TodoError> {
        let response = self
            .http
            .delete(format!("{}/tasks/{}", self.base_url, task_id))
            .header("Authorization", self.get_auth_header())
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;

        if status.is_success() || status.as_u16() == 404 {
            Ok(())
        } else {
            Err(TodoError::Http(status.as_u16(), response_text))
        }
    }

    pub async fn complete_task(&self, task_id: &str) -> Result<(), crate::error::TodoError> {
        let response = self
            .http
            .post(format!("{}/tasks/{}/close", self.base_url, task_id))
            .header("Authorization", self.get_auth_header())
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;

        if status.is_success() || status.as_u16() == 204 {
            Ok(())
        } else {
            Err(TodoError::Http(status.as_u16(), response_text))
        }
    }

    pub async fn reopen_task(&self, task_id: &str) -> Result<(), crate::error::TodoError> {
        let response = self
            .http
            .post(format!("{}/tasks/{}/reopen", self.base_url, task_id))
            .header("Authorization", self.get_auth_header())
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;

        if status.is_success() || status.as_u16() == 204 {
            Ok(())
        } else {
            Err(TodoError::Http(status.as_u16(), response_text))
        }
    }
}

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
            "is_favorite": true
        }"#;

        let project: Project = serde_json::from_str(json).unwrap();
        assert_eq!(project.id, "123");
        assert_eq!(project.name, "Work");
    }

    #[test]
    fn test_projects_response_wrapper() {
        let json = r#"{
            "results": [
                {
                    "id": "123",
                    "name": "Work",
                    "color": "blue",
                    "is_shared": false,
                    "is_favorite": true
                },
                {
                    "id": "456",
                    "name": "Personal",
                    "color": "green",
                    "is_shared": false,
                    "is_favorite": false
                }
            ]
        }"#;

        let response: ProjectsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.results.len(), 2);
        assert_eq!(response.results[0].name, "Work");
        assert_eq!(response.results[1].name, "Personal");
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

    #[test]
    fn test_task_deserialization_api_format() {
        let json = r#"{
            "id": "789",
            "content": "API format task",
            "project_id": "123",
            "due": {"date": "2026-01-15"},
            "checked": false,
            "added_at": "2026-01-10T10:00:00Z",
            "child_order": 1,
            "priority": 4,
            "labels": ["shopping"]
        }"#;

        let task: crate::models::Task = serde_json::from_str(json).unwrap();
        assert_eq!(task.id, "789");
        assert_eq!(task.content, "API format task");
        assert_eq!(task.is_completed, false);
        assert_eq!(task.created_at, "2026-01-10T10:00:00Z");
        assert_eq!(task.order, 1);
    }

    #[test]
    fn test_tasks_response_wrapper() {
        let json = r#"{
            "results": [
                {
                    "id": "789",
                    "content": "Task 1",
                    "project_id": "123",
                    "checked": false,
                    "added_at": "2026-01-10T10:00:00Z",
                    "child_order": 1,
                    "priority": 4,
                    "labels": []
                },
                {
                    "id": "790",
                    "content": "Task 2",
                    "project_id": "456",
                    "checked": true,
                    "added_at": "2026-01-11T10:00:00Z",
                    "child_order": 2,
                    "priority": 2,
                    "labels": ["urgent"]
                }
            ]
        }"#;

        let response: TasksResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.results.len(), 2);
        assert_eq!(response.results[0].content, "Task 1");
        assert_eq!(response.results[1].content, "Task 2");
    }

    #[test]
    fn test_filter_deserialization() {
        let json = r#"{
            "id": "789",
            "name": "This Week",
            "query": "due within \"7 days of today\""
        }"#;

        let filter: crate::models::Filter = serde_json::from_str(json).unwrap();
        assert_eq!(filter.id, "789");
        assert_eq!(filter.name, "This Week");
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_projects_real() {
        let client =
            TodoistClient::new(std::env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN env var"));
        let projects = client.get_projects().await.unwrap();
        assert!(!projects.is_empty());
        println!("Found {} projects", projects.len());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_tasks_real() {
        let client =
            TodoistClient::new(std::env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN env var"));
        let tasks = client.get_tasks(None).await.unwrap();
        println!("Found {} tasks", tasks.len());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_filters_real() {
        let client =
            TodoistClient::new(std::env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN env var"));
        let filters = client.get_filters().await.unwrap();
        println!("Found {} filters", filters.len());
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_task_real() {
        let client =
            TodoistClient::new(std::env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN env var"));

        let task_output = client
            .create_task("Test task from integration test", None, None, None)
            .await
            .unwrap();

        assert_eq!(task_output.content, "Test task from integration test");

        // Cleanup
        let _ = client.delete_task(&task_output.id).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_complete_task_real() {
        let client =
            TodoistClient::new(std::env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN env var"));

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

    #[tokio::test]
    #[ignore]
    async fn test_reopen_task_real() {
        let client =
            TodoistClient::new(std::env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN env var"));

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
}

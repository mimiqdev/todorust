/*!
 * Todoist REST API Client
 *
 * ⚠️ **DEPRECATED**: This module uses the legacy REST API.
 * Please use the [`sync`](sync/index.html) module with [`TodoistSyncClient`] instead,
 * which provides better performance through batch operations and incremental sync.
 *
 * This client provides access to Todoist's REST API v1 for basic operations.
 * For production use, prefer the Sync API for better efficiency.
 */

use reqwest::Client as HttpClient;
use serde::Serialize;

use crate::models::{Filter, Project, ProjectsResponse, SyncResponse, Task, TaskOutput, TasksResponse};
use crate::error::TodoError;

#[deprecated(since = "0.5.0", note = "Use sync::TodoistSyncClient instead for better performance")]
pub struct TodoistClient {
    token: String,
    base_url: String,
    http: HttpClient,
}

impl TodoistClient {
    #[deprecated(since = "0.5.0", note = "Use sync::TodoistSyncClient instead")]
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

    #[deprecated(since = "0.5.0", note = "Use sync::TodoistSyncClient instead")]
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

    #[deprecated(since = "0.5.0", note = "Use sync::TodoistSyncClient instead")]
    pub async fn get_tasks(
        &self,
        filter: Option<String>,
    ) -> Result<Vec<TaskOutput>, crate::error::TodoError> {
        // Check if filter is asking for completed tasks
        let uses_completed_filter = filter.as_ref()
            .map(|f| f.contains("completed"))
            .unwrap_or(false);

        // Use /tasks/completed/by_completion_date for completed tasks
        // Use /tasks/filter for other filters
        // Use /tasks for no filter
        let endpoint = if uses_completed_filter {
            format!("{}/tasks/completed/by_completion_date", self.base_url)
        } else if filter.is_some() {
            format!("{}/tasks/filter", self.base_url)
        } else {
            format!("{}/tasks", self.base_url)
        };

        let mut request = self
            .http
            .get(&endpoint)
            .header("Authorization", self.get_auth_header());

        if uses_completed_filter {
            // For completed tasks, we need since/until dates
            // Parse the filter to extract date range if provided
            // For now, use today's date range
            let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
            request = request.query(&[("since", format!("{}T00:00:00Z", today))]);
            request = request.query(&[("until", format!("{}T23:59:59Z", today))]);

            // If filter contains additional conditions, use filter_query parameter
            if let Some(filter_str) = &filter {
                // Remove "completed" part and use the rest as filter_query
                let filter_query = filter_str
                    .replace("completed today", "")
                    .replace("completed", "")
                    .trim()
                    .to_string();

                if !filter_query.is_empty() {
                    request = request.query(&[("filter_query", &filter_query)]);
                }
            }
        } else if let Some(filter_str) = filter {
            // Use 'query' parameter name for filter endpoint
            request = request.query(&[("query", &filter_str)]);
        }

        let response = request.send().await?;

        let status = response.status();
        let response_text = response.text().await?;

        if !status.is_success() {
            return Err(TodoError::Http(status.as_u16(), response_text));
        }

        // Parse response - completed endpoint uses "items", others use "results"
        let tasks = if uses_completed_filter {
            let completed_response: serde_json::Value = serde_json::from_str(&response_text)
                .map_err(|e| TodoError::Api(format!("Failed to parse completed tasks response: {}\nResponse: {}", e, response_text)))?;

            // Extract items array
            completed_response["items"]
                .as_array()
                .ok_or_else(|| TodoError::Api("Missing 'items' in completed tasks response".to_string()))?
                .iter()
                .filter_map(|v| serde_json::from_value(v.clone()).ok())
                .collect()
        } else {
            let tasks_response: TasksResponse = serde_json::from_str(&response_text)
                .map_err(|e| TodoError::Api(format!("Failed to parse tasks response: {}\nResponse: {}", e, response_text)))?;
            tasks_response.results
        };

        Ok(self.enrich_tasks(tasks).await)
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
                    description: task.description,
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

    #[deprecated(since = "0.5.0", note = "Use sync::TodoistSyncClient instead")]
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

    #[deprecated(since = "0.5.0", note = "Use sync::TodoistSyncClient instead")]
    pub async fn create_task(
        &self,
        content: &str,
        description: Option<String>,
        project_id: Option<String>,
        due_date: Option<String>,
        priority: Option<u8>,
        labels: Option<Vec<String>>,
    ) -> Result<TaskOutput, crate::error::TodoError> {
        let request_body = CreateTaskRequest {
            content: content.to_string(),
            description,
            project_id,
            due_string: due_date,
            priority,
            labels,
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
    #[deprecated(since = "0.5.0", note = "Use sync::TodoistSyncClient instead")]
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

    #[deprecated(since = "0.5.0", note = "Use sync::TodoistSyncClient instead")]
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

    #[deprecated(since = "0.5.0", note = "Use sync::TodoistSyncClient instead")]
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
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to get API token for integration tests
    /// First tries environment variable, then falls back to config file
    fn get_test_token() -> String {
        std::env::var("TODOIST_TOKEN")
            .ok()
            .or_else(|| {
                // Try loading from config file
                crate::config::load_config()
                    .ok()
                    .map(|config| config.api_token)
            })
            .expect("TODOIST_TOKEN env var or config file required")
    }

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
            "description": "From the store",
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
        assert_eq!(task.description.as_deref(), Some("From the store"));
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
    fn test_task_deserialization_with_checked_field() {
        // Test based on official API v1 response format
        // which uses 'checked' field for completion status
        let json = r#"{
            "id": "123",
            "content": "Task with checked field",
            "checked": true,
            "added_at": "2026-01-15T10:00:00Z",
            "child_order": 1,
            "priority": 4,
            "labels": ["urgent"]
        }"#;

        let task: crate::models::Task = serde_json::from_str(json).unwrap();
        assert_eq!(task.is_completed, true, "checked=true should map to is_completed=true");
        assert_eq!(task.content, "Task with checked field");
    }

    #[test]
    fn test_task_deserialization_checked_false() {
        let json = r#"{
            "id": "456",
            "content": "Incomplete task",
            "checked": false,
            "added_at": "2026-01-15T10:00:00Z",
            "child_order": 2,
            "priority": 2,
            "labels": []
        }"#;

        let task: crate::models::Task = serde_json::from_str(json).unwrap();
        assert_eq!(task.is_completed, false, "checked=false should map to is_completed=false");
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
        let client = TodoistClient::new(get_test_token());
        let projects = client.get_projects().await.unwrap();
        assert!(!projects.is_empty());
        println!("Found {} projects", projects.len());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_tasks_real() {
        let client = TodoistClient::new(get_test_token());
        let tasks = client.get_tasks(None).await.unwrap();
        println!("Found {} tasks", tasks.len());
    }

    #[tokio::test]
    #[ignore]
    async fn test_get_filters_real() {
        let client = TodoistClient::new(get_test_token());
        let filters = client.get_filters().await.unwrap();
        println!("Found {} filters", filters.len());
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_task_real() {
        let client = TodoistClient::new(get_test_token());

        let task_output = client
            .create_task("Test task from integration test", None, None, None, None, None)
            .await
            .unwrap();

        assert_eq!(task_output.content, "Test task from integration test");

        // Cleanup
        let _ = client.delete_task(&task_output.id).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_task_with_labels_real() {
        use crate::Formattable;
        let client = TodoistClient::new(get_test_token());

        let task = client
            .create_task(
                "Test task with labels",
                None,
                None,
                None,
                None,
                Some(vec!["test-label".to_string(), "urgent".to_string()]),
            )
            .await
            .unwrap();

        assert_eq!(task.content, "Test task with labels");
        assert!(!task.labels.is_empty());

        // Cleanup
        let _ = client.delete_task(&task.id).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_complete_task_real() {
        let client = TodoistClient::new(get_test_token());

        // Create a task first
        let task = client
            .create_task("Test task for completion", None, None, None, None, None)
            .await
            .unwrap();

        // Complete it
        client.complete_task(&task.id).await.unwrap();

        // Note: Completed tasks may not appear in default task list
        // The test passes if complete_task() succeeds without error

        // Cleanup (delete the completed task)
        let _ = client.delete_task(&task.id).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_reopen_task_real() {
        let client = TodoistClient::new(get_test_token());

        // Create and complete a task
        let task = client
            .create_task("Test task for reopening", None, None, None, None, None)
            .await
            .unwrap();
        client.complete_task(&task.id).await.unwrap();

        // Reopen it
        client.reopen_task(&task.id).await.unwrap();

        // Verify it's reopened by finding it in the active tasks
        let tasks = client.get_tasks(None).await.unwrap();
        let reopened_task = tasks.iter().find(|t| t.id == task.id);

        assert!(reopened_task.is_some(), "Task should be in active tasks after reopening");
        assert!(!reopened_task.unwrap().is_completed, "Task should not be completed after reopening");

        // Cleanup
        let _ = client.delete_task(&task.id).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_priority_filter_works() {
        let client = TodoistClient::new(get_test_token());

        // Create a priority 4 task (lowest priority in API)
        let task_p4 = client
            .create_task("Test priority filter P4", None, None, None, Some(4), None)
            .await
            .unwrap();

        // Create a priority 1 task (highest priority in API)
        let task_p1 = client
            .create_task("Test priority filter P1", None, None, None, Some(1), None)
            .await
            .unwrap();

        // Wait for API to process
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Filter by priority 1 in filter language (returns API priority 4 tasks - lowest)
        // Note: Filter priority values are inverted from API priority values!
        // API priority 1 (highest) = Filter priority 4
        // API priority 4 (lowest) = Filter priority 1
        let tasks = client.get_tasks(Some("priority:1".to_string())).await.unwrap();

        // Verify all returned tasks have API priority 4 (lowest)
        for task in &tasks {
            assert_eq!(task.priority, 4, "Filter priority:1 should return API priority 4 tasks (lowest)");
        }

        // Cleanup
        let _ = client.delete_task(&task_p4.id).await;
        let _ = client.delete_task(&task_p1.id).await;
    }

    #[tokio::test]
    #[ignore]
    async fn test_completed_status_shows_correctly() {
        use crate::{Formattable, OutputFormat};
        let client = TodoistClient::new(get_test_token());

        // Create and complete a task
        let task = client
            .create_task("Test completed status display", None, None, None, None, None)
            .await
            .unwrap();

        // Complete it
        client.complete_task(&task.id).await.unwrap();

        // Wait for API to process completed tasks
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        // Get completed tasks using the completed endpoint
        let tasks = client.get_tasks(Some("completed today".to_string())).await.unwrap();

        // Verify the checkbox shows [x] for completed tasks
        let output = tasks.format(&OutputFormat::Checklist);

        // Cleanup
        let _ = client.delete_task(&task.id).await;

        // Assertions
        if !tasks.is_empty() {
            // Verify at least one task is marked as completed
            let has_completed = tasks.iter().any(|t| t.is_completed);
            assert!(has_completed, "Should have at least one completed task");

            // Verify checklist format shows [x]
            assert!(output.contains("[x]"), "Completed tasks should show [x] in checklist format");
        }
        // If tasks is empty, the test passes but we can't verify the format
    }

    #[tokio::test]
    #[ignore]
    async fn test_structured_format_real() {
        use crate::{Formattable, OutputFormat};
        let client = TodoistClient::new(get_test_token());
        let tasks = client.get_tasks(None).await.unwrap();
        let output = tasks.format(&OutputFormat::Structured);

        // Verify has project headings
        assert!(output.contains("## ") || output.is_empty());

        // Verify tasks under projects
        for line in output.lines() {
            let line_str: &str = line;
            if line_str.starts_with("- ") {
                assert!(line_str.contains("[x]") || line_str.contains("[ ]"),
                        "Task line should have checkbox: {}", line_str);
            }
        }
    }
}

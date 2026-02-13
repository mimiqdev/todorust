use std::cell::RefCell;

use reqwest::Client as HttpClient;

use crate::error::TodoError;

use super::commands::{Command, CommandBuilder};
use super::models::{SyncReadResponse, SyncWriteResponse};

/// Todoist Sync API Client
///
/// This client provides access to Todoist's Sync API v1,
/// which supports batch operations and incremental synchronization.
///
/// # Features
/// - Batch read operations for all resource types
/// - Batch write operations via commands
/// - Incremental sync with sync_token
///
/// # Example
///
/// ```ignore
/// use todorust::sync::TodoistSyncClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = TodoistSyncClient::new("your_token".to_string());
///
///     // Full sync to get all resources
///     let response = client.sync(&["projects", "items", "sections"]).await?;
///     println!("Projects: {:?}", response.projects);
///     println!("Tasks: {:?}", response.items);
///
///     Ok(())
/// }
/// ```
pub struct TodoistSyncClient {
    token: String,
    sync_url: String,
    sync_token: RefCell<Option<String>>,
    http: HttpClient,
}

impl TodoistSyncClient {
    /// Creates a new TodoistSyncClient with the provided API token.
    ///
    /// # Arguments
    ///
    /// * `token` - Your Todoist API token
    ///
    /// # Returns
    ///
    /// A new `TodoistSyncClient` instance
    pub fn new(token: String) -> Self {
        Self {
            token,
            sync_url: "https://api.todoist.com/api/v1/sync".to_string(),
            sync_token: RefCell::new(None),
            http: HttpClient::new(),
        }
    }

    #[cfg(test)]
    pub fn new_with_url(token: String, sync_url: String) -> Self {
        Self {
            token,
            sync_url,
            sync_token: RefCell::new(None),
            http: HttpClient::new(),
        }
    }

    /// Get authorization header value
    fn get_auth_header(&self) -> String {
        format!("Bearer {}", self.token)
    }

    /// 基础同步请求（读取资源）
    ///
    /// # Arguments
    ///
    /// * `resource_types` - Array of resource types to sync (e.g., ["projects", "items"])
    ///
    /// # Returns
    ///
    /// A `SyncReadResponse` containing the synced resources and a new sync_token.
    pub async fn sync(&self, resource_types: &[&str]) -> Result<SyncReadResponse, TodoError> {
        let sync_token = self
            .sync_token
            .borrow()
            .clone()
            .unwrap_or_else(|| "*".to_string());

        let response = self
            .http
            .post(&self.sync_url)
            .header("Authorization", self.get_auth_header())
            .form(&[
                ("sync_token", sync_token),
                (
                    "resource_types",
                    serde_json::to_string(resource_types).unwrap(),
                ),
            ])
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            return Err(TodoError::Http(status.as_u16()));
        }

        serde_json::from_str(&body)
            .map_err(|e| TodoError::Api(format!("Failed to parse sync response: {}", e)))
    }

    /// 执行命令（写入资源）
    ///
    /// # Arguments
    ///
    /// * `commands` - Array of Command objects to execute
    ///
    /// # Returns
    ///
    /// A `SyncWriteResponse` containing the sync_token and command status.
    pub async fn execute_commands(
        &self,
        commands: &[Command],
    ) -> Result<SyncWriteResponse, TodoError> {
        let response = self
            .http
            .post(&self.sync_url)
            .header("Authorization", self.get_auth_header())
            .form(&[("commands", serde_json::to_string(commands).unwrap())])
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            return Err(TodoError::Http(status.as_u16()));
        }

        serde_json::from_str(&body)
            .map_err(|e| TodoError::Api(format!("Failed to parse command response: {}", e)))
    }

    /// Gets the current sync token.
    ///
    /// Returns `None` if no sync has been performed yet.
    pub fn get_sync_token(&self) -> Option<String> {
        self.sync_token.borrow().clone()
    }

    /// Sets the sync token for incremental sync.
    ///
    /// Use this to continue from a previous sync state.
    pub fn set_sync_token(&self, token: String) {
        *self.sync_token.borrow_mut() = Some(token);
    }

    /// 执行命令并检查状态
    pub async fn execute_commands_with_status(
        &self,
        commands: &[Command],
    ) -> Result<SyncWriteResponse, TodoError> {
        let response = self.execute_commands(commands).await?;
        self.check_sync_status(&response)?;
        Ok(response)
    }

    /// 检查命令执行状态
    fn check_sync_status(&self, response: &SyncWriteResponse) -> Result<(), TodoError> {
        for (uuid, status) in &response.sync_status {
            if status == "ok" {
                continue;
            }
            return Err(TodoError::Api(format!(
                "Command {} failed: {}",
                uuid, status
            )));
        }
        Ok(())
    }

    /// 辅助方法：从 CommandBuilder 执行命令
    pub async fn execute(&self, builder: CommandBuilder) -> Result<SyncWriteResponse, TodoError> {
        let commands = builder.build();
        self.execute_commands_with_status(&commands).await
    }

    // ==================== 资源读取方法 ====================

    /// 获取所有项目 (使用 Sync API)
    pub async fn get_projects(&self) -> Result<Vec<crate::models::Project>, TodoError> {
        let response = self.sync(&["projects"]).await?;
        Ok(response.projects.into_iter().map(Into::into).collect())
    }

    /// 获取所有任务/项目 (使用 Sync API)
    pub async fn get_tasks(&self) -> Result<Vec<crate::models::Task>, TodoError> {
        let response = self.sync(&["items"]).await?;
        Ok(response.items.into_iter().map(Into::into).collect())
    }

    /// 获取所有分区 (使用 Sync API)
    pub async fn get_sections(&self) -> Result<Vec<super::models::SyncSection>, TodoError> {
        let response = self.sync(&["sections"]).await?;
        Ok(response.sections)
    }

    /// 获取所有标签 (使用 Sync API)
    pub async fn get_labels(&self) -> Result<Vec<super::models::SyncLabel>, TodoError> {
        let response = self.sync(&["labels"]).await?;
        Ok(response.labels)
    }

    /// 获取所有过滤器 (使用 Sync API)
    pub async fn get_filters(&self) -> Result<Vec<super::models::SyncFilter>, TodoError> {
        let response = self.sync(&["filters"]).await?;
        Ok(response.filters)
    }

    // ==================== 资源写入方法 ====================

    /// 添加项目 (使用 Sync API)
    pub async fn add_project(
        &self,
        name: &str,
        color: Option<&str>,
        favorite: Option<bool>,
    ) -> Result<String, TodoError> {
        let args = super::commands::ProjectAddArgs::new(name.to_string())
            .color(color.map(|c| c.to_string()))
            .favorite(favorite);

        let builder = CommandBuilder::new().project_add(args);

        let response = self.execute(builder).await?;

        // 提取真实 ID
        response
            .temp_id_mapping
            .values()
            .next()
            .ok_or_else(|| TodoError::Api("No ID returned".to_string()))
            .cloned()
    }

    /// 添加任务 (使用 Sync API)
    #[allow(clippy::too_many_arguments)]
    pub async fn add_task(
        &self,
        content: &str,
        description: Option<&str>,
        project_id: Option<&str>,
        section_id: Option<&str>,
        due_string: Option<&str>,
        priority: Option<u8>,
        labels: Option<Vec<&str>>,
    ) -> Result<String, TodoError> {
        let args = super::commands::ItemAddArgs::new(content.to_string())
            .description(description.map(|d| d.to_string()))
            .project_id(project_id.map(|p| p.to_string()))
            .section_id(section_id.map(|s| s.to_string()))
            .due_string(due_string.map(|d| d.to_string()))
            .priority(priority)
            .labels(labels.map(|ls| ls.iter().map(|&s| s.to_string()).collect()));

        let builder = CommandBuilder::new().item_add(args);

        let response = self.execute(builder).await?;

        // 提取真实 ID
        response
            .temp_id_mapping
            .values()
            .next()
            .ok_or_else(|| TodoError::Api("No ID returned".to_string()))
            .cloned()
    }

    /// 更新任务 (使用 Sync API)
    pub async fn update_task(
        &self,
        id: &str,
        content: Option<&str>,
        description: Option<&str>,
        priority: Option<u8>,
        due_string: Option<&str>,
        labels: Option<Vec<&str>>,
    ) -> Result<(), TodoError> {
        let args = super::commands::ItemUpdateArgs::new(id.to_string())
            .content(content.map(|c| c.to_string()))
            .description(description.map(|d| d.to_string()))
            .priority(priority)
            .due_string(due_string.map(|d| d.to_string()))
            .labels(labels.map(|ls| ls.iter().map(|&s| s.to_string()).collect()));

        let builder = CommandBuilder::new().item_update(args);

        self.execute(builder).await?;
        Ok(())
    }

    /// 完成任务 (使用 Sync API)
    pub async fn complete_task(&self, id: &str) -> Result<(), TodoError> {
        let builder = CommandBuilder::new().item_complete(id);

        self.execute(builder).await?;
        Ok(())
    }

    /// 删除任务 (使用 Sync API)
    pub async fn delete_task(&self, id: &str) -> Result<(), TodoError> {
        let builder = CommandBuilder::new().item_delete(id);

        self.execute(builder).await?;
        Ok(())
    }

    /// 添加分区 (使用 Sync API)
    pub async fn add_section(&self, name: &str, project_id: &str) -> Result<String, TodoError> {
        let args = super::commands::SectionAddArgs::new(name.to_string(), project_id.to_string());

        let builder = CommandBuilder::new().section_add(args);

        let response = self.execute(builder).await?;

        // 提取真实 ID
        response
            .temp_id_mapping
            .values()
            .next()
            .ok_or_else(|| TodoError::Api("No ID returned".to_string()))
            .cloned()
    }

    /// 更新分区 (使用 Sync API)
    pub async fn update_section(&self, id: &str, name: &str) -> Result<(), TodoError> {
        let builder = CommandBuilder::new().section_update(id, name);

        self.execute(builder).await?;
        Ok(())
    }

    /// 删除分区 (使用 Sync API)
    pub async fn delete_section(&self, id: &str) -> Result<(), TodoError> {
        let builder = CommandBuilder::new().section_delete(id);

        self.execute(builder).await?;
        Ok(())
    }

    /// 添加标签 (使用 Sync API)
    pub async fn add_label(&self, name: &str, color: Option<&str>) -> Result<String, TodoError> {
        let args = super::commands::LabelAddArgs::new(name.to_string())
            .color(color.map(|c| c.to_string()));

        let builder = CommandBuilder::new().label_add(args);

        let response = self.execute(builder).await?;

        // 提取真实 ID
        response
            .temp_id_mapping
            .values()
            .next()
            .ok_or_else(|| TodoError::Api("No ID returned".to_string()))
            .cloned()
    }

    /// 更新标签 (使用 Sync API)
    pub async fn update_label(
        &self,
        id: &str,
        name: Option<&str>,
        color: Option<&str>,
    ) -> Result<(), TodoError> {
        let builder = CommandBuilder::new().label_update(id, name, color);

        self.execute(builder).await?;
        Ok(())
    }

    /// 删除标签 (使用 Sync API)
    pub async fn delete_label(&self, id: &str) -> Result<(), TodoError> {
        let builder = CommandBuilder::new().label_delete(id);

        self.execute(builder).await?;
        Ok(())
    }

    /// 更新过滤器顺序 (使用 Sync API)
    pub async fn update_filter_order(&self, filters: &[(&str, i64)]) -> Result<(), TodoError> {
        let filter_args: Vec<super::commands::FilterOrderArgs> = filters
            .iter()
            .map(|(id, order)| super::commands::FilterOrderArgs::new(id.to_string(), *order))
            .collect();

        let builder = CommandBuilder::new().filter_update_orders(&filter_args);

        self.execute(builder).await?;
        Ok(())
    }

    /// 添加过滤器 (使用 Sync API)
    pub async fn add_filter(
        &self,
        name: &str,
        query: &str,
        color: Option<&str>,
    ) -> Result<String, TodoError> {
        let args = super::commands::FilterAddArgs::new(name.to_string(), query.to_string())
            .color(color.map(|c| c.to_string()));

        let builder = CommandBuilder::new().filter_add(args);

        let response = self.execute(builder).await?;

        // 提取真实 ID
        response
            .temp_id_mapping
            .values()
            .next()
            .ok_or_else(|| TodoError::Api("No ID returned".to_string()))
            .cloned()
    }

    /// 更新过滤器 (使用 Sync API)
    pub async fn update_filter(
        &self,
        id: &str,
        name: Option<&str>,
        query: Option<&str>,
        color: Option<&str>,
    ) -> Result<(), TodoError> {
        let builder = CommandBuilder::new().filter_update(id, name, query, color);

        self.execute(builder).await?;
        Ok(())
    }

    /// 删除过滤器 (使用 Sync API)
    pub async fn delete_filter(&self, id: &str) -> Result<(), TodoError> {
        let builder = CommandBuilder::new().filter_delete(id);

        self.execute(builder).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::{Method, MockServer, HttpMockResponse};
    use tokio::test;

    // ... existing tests ...

    #[test]
    async fn test_client_new() {
        let client = TodoistSyncClient::new("test_token".to_string());
        assert_eq!(client.token, "test_token");
        assert_eq!(client.sync_url, "https://api.todoist.com/api/v1/sync");
    }

    #[test]
    async fn test_get_auth_header() {
        let client = TodoistSyncClient::new("my_token".to_string());
        let header = client.get_auth_header();
        assert_eq!(header, "Bearer my_token");
    }

    #[test]
    async fn test_sync_token_initial_none() {
        let client = TodoistSyncClient::new("test_token".to_string());
        assert_eq!(client.get_sync_token(), None);
    }

    #[test]
    async fn test_set_sync_token() {
        let client = TodoistSyncClient::new("test_token".to_string());
        client.set_sync_token("initial_token".to_string());
        assert_eq!(client.get_sync_token(), Some("initial_token".to_string()));
    }

    #[test]
    async fn test_set_sync_token_overwrites() {
        let client = TodoistSyncClient::new("test_token".to_string());
        client.set_sync_token("token1".to_string());
        client.set_sync_token("token2".to_string());
        assert_eq!(client.get_sync_token(), Some("token2".to_string()));
    }

    #[test]
    async fn test_sync_token_refcell_behavior() {
        let client = TodoistSyncClient::new("test_token".to_string());

        // Set initial token
        client.set_sync_token("token_a".to_string());
        let first_token = client.get_sync_token();
        assert_eq!(first_token, Some("token_a".to_string()));

        // Update token
        client.set_sync_token("token_b".to_string());
        let second_token = client.get_sync_token();
        assert_eq!(second_token, Some("token_b".to_string()));
    }

    #[test]
    async fn test_client_with_empty_token() {
        let client = TodoistSyncClient::new("".to_string());
        assert_eq!(client.token, "");
        let header = client.get_auth_header();
        assert_eq!(header, "Bearer ");
    }

    #[test]
    async fn test_sync_url_is_correct() {
        let client = TodoistSyncClient::new("test".to_string());
        assert!(client.sync_url.starts_with("https://api.todoist.com"));
        assert!(client.sync_url.contains("/v1/sync"));
    }

    #[tokio::test]
    async fn test_sync_success() {
        // 启动 mock server
        let server = MockServer::start_async().await;

        // 设置 mock 响应
        let mock_response = serde_json::json!({
            "projects": [
                {"id": "1", "name": "Test Project", "color": "red", "shared": false, "favorite": false, "sort_order": 1, "is_archived": false, "is_deleted": false, "created_at": "2024-01-01T00:00:00Z", "updated_at": "2024-01-01T00:00:00Z"}
            ],
            "items": [],
            "sections": [],
            "labels": [],
            "filters": [],
            "sync_token": "test_token_123"
        });

        let mock_response_clone = mock_response.clone();
        server.mock(|when, then| {
            when.method(Method::POST)
                .path("/api/v1/sync");
            then.respond_with(move |_req: &httpmock::HttpMockRequest| {
                HttpMockResponse::builder()
                    .status(200)
                    .body(mock_response_clone.to_string())
                    .build()
            });
        });

        // 创建 client 指向 mock server
        let client = TodoistSyncClient::new_with_url(
            "test_token".to_string(),
            server.url("/api/v1/sync")
        );

        let response = client.sync(&["projects"]).await.unwrap();

        assert_eq!(response.projects.len(), 1);
        assert_eq!(response.projects[0].name, "Test Project");
    }

    #[tokio::test]
    async fn test_sync_http_error() {
        let server = MockServer::start_async().await;
        
        // Mock 401 错误响应
        server.mock(|when, then| {
            when.method(Method::POST)
                .path("/api/v1/sync");
            then.respond_with(|_req: &httpmock::HttpMockRequest| {
                HttpMockResponse::builder()
                    .status(401)
                    .body("Unauthorized")
                    .build()
            });
        });
        
        let client = TodoistSyncClient::new_with_url(
            "bad_token".to_string(),
            server.url("/api/v1/sync")
        );
        
        let result = client.sync(&["projects"]).await;
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(matches!(e, TodoError::Http(401)));
        }
    }
}

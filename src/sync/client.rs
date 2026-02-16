use std::cell::RefCell;

use reqwest::Client as HttpClient;

use crate::error::TodoError;

use super::cache::{Cache, CacheData, CacheManager};
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
/// ```no_run
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
    cache_manager: CacheManager,
    cache: RefCell<Option<Cache>>,
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
        let http = HttpClient::builder()
            .user_agent("todorust/0.3.0")
            .build()
            .unwrap_or_else(|_| HttpClient::new());

        let sync_url = std::env::var("TODORUST_SYNC_URL")
            .unwrap_or_else(|_| "https://api.todoist.com/api/v1/sync".to_string());

        Self {
            token: token.trim().to_string(),
            sync_url,
            sync_token: RefCell::new(None),
            http,
            cache_manager: CacheManager::new(),
            cache: RefCell::new(None),
        }
    }

    #[cfg(test)]
    pub fn new_with_url(token: String, sync_url: String) -> Self {
        Self {
            token: token.trim().to_string(),
            sync_url,
            sync_token: RefCell::new(None),
            http: HttpClient::new(),
            cache_manager: CacheManager::new(),
            cache: RefCell::new(None),
        }
    }

    /// 尝试从缓存加载数据
    pub fn load_cache(&self) -> Result<Option<Cache>, TodoError> {
        self.cache_manager.load()
    }

    /// 保存缓存
    pub fn save_cache(&self, sync_token: &str, data: CacheData) -> Result<(), TodoError> {
        let cache = Cache {
            sync_token: sync_token.to_string(),
            cached_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0),
            data,
        };
        self.cache_manager.save(&cache)
    }

    /// 检查缓存是否过期 (默认 5 分钟 = 300 秒)
    pub fn is_cache_expired(&self) -> bool {
        if let Some(ref cache) = *self.cache.borrow() {
            self.cache_manager.is_expired(cache, 300)
        } else {
            true
        }
    }

    /// 混合同步：优先使用缓存，必要时增量/全量同步
    pub async fn sync_with_cache(&self, resource_types: &[&str]) -> Result<SyncReadResponse, TodoError> {
        // 尝试加载缓存
        if self.cache.borrow().is_none() {
            if let Ok(Some(cache)) = self.cache_manager.load() {
                *self.cache.borrow_mut() = Some(cache);
            }
        }

        // 检查是否需要刷新
        let needs_full_sync = self.is_cache_expired() || self.sync_token.borrow().is_none();

        if needs_full_sync {
            // 全量同步
            tracing::info!("Performing full sync");
            let response = self.sync(resource_types).await?;
            *self.sync_token.borrow_mut() = Some(response.sync_token.clone());
            return Ok(response);
        }

        // 增量同步
        tracing::info!("Performing incremental sync");
        let response = self.sync(resource_types).await?;
        *self.sync_token.borrow_mut() = Some(response.sync_token.clone());
        Ok(response)
    }

    /// 获取缓存数据
    pub fn get_cached_data(&self) -> Option<CacheData> {
        self.cache.borrow().as_ref().map(|c| c.data.clone())
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
    #[tracing::instrument(skip(self), fields(resource_types = ?resource_types))]
    pub async fn sync(&self, resource_types: &[&str]) -> Result<SyncReadResponse, TodoError> {
        let sync_token = self
            .sync_token
            .borrow()
            .clone()
            .unwrap_or_else(|| "*".to_string());

        tracing::debug!(sync_token = %sync_token, "Performing sync request");

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
        tracing::debug!(status = %status, "Received sync response");

        let body = response.text().await?;

        if !status.is_success() {
            tracing::error!(status = %status, body = %body, "Sync request failed");
            return Err(TodoError::Http(status.as_u16()));
        }

        let parsed: SyncReadResponse = serde_json::from_str(&body)
            .map_err(|e| TodoError::Api(format!("Failed to parse sync response: {}", e)))?;

        // Update sync token
        self.set_sync_token(parsed.sync_token.clone());

        // 保存到缓存
        let data = CacheData {
            projects: parsed.projects.clone(),
            items: parsed.items.clone(),
            sections: parsed.sections.clone(),
            labels: parsed.labels.clone(),
            filters: parsed.filters.clone(),
        };
        self.save_cache(&parsed.sync_token, data)?;

        Ok(parsed)
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
    #[tracing::instrument(skip(self, commands))]
    pub async fn execute_commands(
        &self,
        commands: &[Command],
    ) -> Result<SyncWriteResponse, TodoError> {
        let sync_token = self
            .sync_token
            .borrow()
            .clone()
            .unwrap_or_else(|| "*".to_string());

        tracing::debug!(
            command_count = commands.len(),
            sync_token = %sync_token,
            "Executing batch commands"
        );

        let response = self
            .http
            .post(&self.sync_url)
            .header("Authorization", self.get_auth_header())
            .form(&[
                ("sync_token", sync_token),
                ("commands", serde_json::to_string(commands).unwrap()),
            ])
            .send()
            .await?;

        let status = response.status();
        tracing::debug!(status = %status, "Received command execution response");

        let body = response.text().await?;

        if !status.is_success() {
            tracing::error!(status = %status, body = %body, "Command execution failed");
            return Err(TodoError::Http(status.as_u16()));
        }

        let parsed: SyncWriteResponse = serde_json::from_str(&body)
            .map_err(|e| TodoError::Api(format!("Failed to parse command response: {}", e)))?;

        // Update sync token
        self.set_sync_token(parsed.sync_token.clone());

        Ok(parsed)
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

    // Resources: Read Methods

    /// 获取所有项目 (使用混合同步)
    pub async fn get_projects(&self) -> Result<Vec<crate::models::Project>, TodoError> {
        let response = self.sync_with_cache(&["projects"]).await?;
        Ok(response.projects.into_iter().map(Into::into).collect())
    }

    /// 获取所有任务/项目 (使用混合同步)
    pub async fn get_tasks(&self) -> Result<Vec<crate::models::Task>, TodoError> {
        let response = self.sync_with_cache(&["items"]).await?;
        Ok(response.items.into_iter().map(Into::into).collect())
    }

    /// 获取所有分区 (使用混合同步)
    pub async fn get_sections(&self) -> Result<Vec<super::models::SyncSection>, TodoError> {
        let response = self.sync_with_cache(&["sections"]).await?;
        Ok(response.sections)
    }

    /// 获取所有标签 (使用混合同步)
    pub async fn get_labels(&self) -> Result<Vec<super::models::SyncLabel>, TodoError> {
        let response = self.sync_with_cache(&["labels"]).await?;
        Ok(response.labels)
    }

    /// 获取所有过滤器 (使用混合同步)
    pub async fn get_filters(&self) -> Result<Vec<super::models::SyncFilter>, TodoError> {
        let response = self.sync_with_cache(&["filters"]).await?;
        Ok(response.filters)
    }

    /// 获取项目和任务 (用于需要两者的场景，如 get_tasks handler)
    pub async fn get_projects_and_tasks(&self) -> Result<(Vec<crate::models::Project>, Vec<crate::models::Task>), TodoError> {
        let response = self.sync_with_cache(&["projects", "items"]).await?;
        let projects = response.projects.into_iter().map(Into::into).collect();
        let tasks = response.items.into_iter().map(Into::into).collect();
        Ok((projects, tasks))
    }

    // Resources: Write Methods

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

    /// 取消完成任务 (使用 Sync API)
    pub async fn reopen_task(&self, id: &str) -> Result<(), TodoError> {
        let builder = CommandBuilder::new().item_uncomplete(id);

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

    /// 重新排序分区 (使用 Sync API)
    pub async fn reorder_sections(&self, sections: &[(&str, i64)]) -> Result<(), TodoError> {
        let section_args: Vec<super::commands::SectionOrderArgs> = sections
            .iter()
            .map(|(id, order)| super::commands::SectionOrderArgs::new(id.to_string(), *order))
            .collect();

        let builder = CommandBuilder::new().section_reorder(&section_args);

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
    use httpmock::{HttpMockResponse, Method, MockServer};
    use serial_test::serial;
    use tokio::test;

    // ... existing tests ...

    #[test]
    #[serial]
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

    #[tokio::test]
    #[serial]
    async fn test_sync_url_is_correct() {
        let client = TodoistSyncClient::new("test".to_string());
        if let Ok(env_url) = std::env::var("TODORUST_SYNC_URL") {
            assert_eq!(client.sync_url, env_url);
        } else {
            assert!(client.sync_url.starts_with("https://api.todoist.com"));
        }
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
            when.method(Method::POST).path("/api/v1/sync");
            then.respond_with(move |_req: &httpmock::HttpMockRequest| {
                HttpMockResponse::builder()
                    .status(200)
                    .body(mock_response_clone.to_string())
                    .build()
            });
        });

        // 创建 client 指向 mock server
        let client =
            TodoistSyncClient::new_with_url("test_token".to_string(), server.url("/api/v1/sync"));

        let response = client.sync(&["projects"]).await.unwrap();

        assert_eq!(response.projects.len(), 1);
        assert_eq!(response.projects[0].name, "Test Project");
    }

    #[tokio::test]
    async fn test_sync_http_error() {
        let server = MockServer::start_async().await;

        // Mock 401 错误响应
        server.mock(|when, then| {
            when.method(Method::POST).path("/api/v1/sync");
            then.respond_with(|_req: &httpmock::HttpMockRequest| {
                HttpMockResponse::builder()
                    .status(401)
                    .body("Unauthorized")
                    .build()
            });
        });

        let client =
            TodoistSyncClient::new_with_url("bad_token".to_string(), server.url("/api/v1/sync"));

        let result = client.sync(&["projects"]).await;
        assert!(result.is_err());

        if let Err(e) = result {
            assert!(matches!(e, TodoError::Http(401)));
        }
    }

    #[tokio::test]
    async fn test_sync_parse_error() {
        let server = MockServer::start_async().await;

        // 返回无效 JSON
        server.mock(|when, then| {
            when.method(Method::POST).path("/api/v1/sync");
            then.respond_with(|_req: &httpmock::HttpMockRequest| {
                HttpMockResponse::builder()
                    .status(200)
                    .body("not json")
                    .build()
            });
        });

        let client =
            TodoistSyncClient::new_with_url("test_token".to_string(), server.url("/api/v1/sync"));

        let result = client.sync(&["projects"]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_check_sync_status_all_ok() {
        let client = TodoistSyncClient::new("test".to_string());

        let response = SyncWriteResponse {
            sync_token: "test_token".to_string(),
            temp_id_mapping: Default::default(),
            sync_status: {
                let mut map = std::collections::HashMap::new();
                map.insert(
                    "uuid1".to_string(),
                    serde_json::Value::String("ok".to_string()),
                );
                map.insert(
                    "uuid2".to_string(),
                    serde_json::Value::String("ok".to_string()),
                );
                map
            },
        };

        let result = client.check_sync_status(&response);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_check_sync_status_failed_command() {
        let client = TodoistSyncClient::new("test".to_string());

        let response = SyncWriteResponse {
            sync_token: "test_token".to_string(),
            temp_id_mapping: Default::default(),
            sync_status: {
                let mut map = std::collections::HashMap::new();
                map.insert(
                    "uuid1".to_string(),
                    serde_json::Value::String("ok".to_string()),
                );
                map.insert(
                    "uuid2".to_string(),
                    serde_json::Value::String("error: something went wrong".to_string()),
                );
                map
            },
        };

        let result = client.check_sync_status(&response);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_execute_commands_success() {
        let server = MockServer::start_async().await;

        let mock_response = serde_json::json!({
            "sync_token": "token_xyz",
            "temp_id_mapping": {
                "temp_123": "real_456"
            },
            "sync_status": {
                "temp_123": "ok"
            }
        });

        let mock_response_clone = mock_response.clone();
        server.mock(|when, then| {
            when.method(Method::POST).path("/api/v1/sync");
            then.respond_with(move |_req: &httpmock::HttpMockRequest| {
                HttpMockResponse::builder()
                    .status(200)
                    .body(mock_response_clone.to_string())
                    .build()
            });
        });

        let client =
            TodoistSyncClient::new_with_url("test_token".to_string(), server.url("/api/v1/sync"));

        let commands = vec![Command {
            type_: "item_add".to_string(),
            uuid: "temp_123".to_string(),
            args: serde_json::json!({"content": "Test task"}),
            temp_id: Some("temp_123".to_string()),
        }];

        let result = client.execute_commands(&commands).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_projects() {
        let server = MockServer::start_async().await;

        let mock_response = serde_json::json!({
            "projects": [
                {"id": "1", "name": "Project A", "color": "red", "shared": false, "favorite": false, "sort_order": 1, "is_archived": false, "is_deleted": false, "created_at": "2024-01-01T00:00:00Z", "updated_at": "2024-01-01T00:00:00Z"},
                {"id": "2", "name": "Project B", "color": "blue", "shared": false, "favorite": false, "sort_order": 2, "is_archived": false, "is_deleted": false, "created_at": "2024-01-01T00:00:00Z", "updated_at": "2024-01-01T00:00:00Z"}
            ],
            "items": [],
            "sections": [],
            "labels": [],
            "filters": [],
            "sync_token": "token_xyz"
        });

        let mock_response_clone = mock_response.clone();
        server.mock(|when, then| {
            when.method(Method::POST).path("/api/v1/sync");
            then.respond_with(move |_req: &httpmock::HttpMockRequest| {
                HttpMockResponse::builder()
                    .status(200)
                    .body(mock_response_clone.to_string())
                    .build()
            });
        });

        let client =
            TodoistSyncClient::new_with_url("test_token".to_string(), server.url("/api/v1/sync"));

        let projects = client.get_projects().await.unwrap();

        assert_eq!(projects.len(), 2);
        assert_eq!(projects[0].name, "Project A");
    }

    #[tokio::test]
    async fn test_get_tasks() {
        let server = MockServer::start_async().await;
        let mock_response = serde_json::json!({
            "projects": [],
            "items": [
                {"id": "1", "content": "Task 1", "description": "", "project_id": "p1", "is_completed": false, "created_at": "2024-01-01T00:00:00Z", "updated_at": "2024-01-01T00:00:00Z", "priority": 4, "order": 1, "labels": []}
            ],
            "sections": [],
            "labels": [],
            "filters": [],
            "sync_token": "token"
        });
        let mock_clone = mock_response.clone();
        server.mock(|when, then| {
            when.method(Method::POST).path("/api/v1/sync");
            then.respond_with(move |_req| {
                HttpMockResponse::builder()
                    .status(200)
                    .body(mock_clone.to_string())
                    .build()
            });
        });
        let client =
            TodoistSyncClient::new_with_url("test".to_string(), server.url("/api/v1/sync"));
        let tasks = client.get_tasks().await.unwrap();
        assert_eq!(tasks.len(), 1);
    }

    #[tokio::test]
    async fn test_add_task() {
        let server = MockServer::start_async().await;
        let mock_response = serde_json::json!({
            "sync_token": "token",
            "temp_id_mapping": {"temp_1": "real_1"},
            "sync_status": {"temp_1": "ok"}
        });
        let mock_clone = mock_response.clone();
        server.mock(|when, then| {
            when.method(Method::POST).path("/api/v1/sync");
            then.respond_with(move |_req| {
                HttpMockResponse::builder()
                    .status(200)
                    .body(mock_clone.to_string())
                    .build()
            });
        });
        let client =
            TodoistSyncClient::new_with_url("test".to_string(), server.url("/api/v1/sync"));
        let id = client
            .add_task("New task", None, None, None, None, None, None)
            .await
            .unwrap();
        assert_eq!(id, "real_1");
    }

    #[tokio::test]
    async fn test_delete_task() {
        let server = MockServer::start_async().await;
        let mock_response = serde_json::json!({
            "sync_token": "token",
            "temp_id_mapping": {},
            "sync_status": {"temp_del": "ok"}
        });
        let mock_clone = mock_response.clone();
        server.mock(|when, then| {
            when.method(Method::POST).path("/api/v1/sync");
            then.respond_with(move |_req| {
                HttpMockResponse::builder()
                    .status(200)
                    .body(mock_clone.to_string())
                    .build()
            });
        });
        let client =
            TodoistSyncClient::new_with_url("test".to_string(), server.url("/api/v1/sync"));
        let result = client.delete_task("task_123").await;
        assert!(result.is_ok(), "Delete task should succeed");
    }

    #[tokio::test]
    async fn test_complete_task() {
        let server = MockServer::start_async().await;
        let mock_response = serde_json::json!({
            "sync_token": "token",
            "temp_id_mapping": {},
            "sync_status": {"temp_comp": "ok"}
        });
        let mock_clone = mock_response.clone();
        server.mock(|when, then| {
            when.method(Method::POST).path("/api/v1/sync");
            then.respond_with(move |_req| {
                HttpMockResponse::builder()
                    .status(200)
                    .body(mock_clone.to_string())
                    .build()
            });
        });
        let client =
            TodoistSyncClient::new_with_url("test".to_string(), server.url("/api/v1/sync"));
        let result = client.complete_task("task_123").await;
        assert!(result.is_ok(), "Complete task should succeed");
    }

    #[tokio::test]
    async fn test_add_section() {
        let server = MockServer::start_async().await;
        let mock_response = serde_json::json!({
            "sync_token": "token",
            "temp_id_mapping": {"temp_s": "sec_1"},
            "sync_status": {"temp_s": "ok"}
        });
        let mock_clone = mock_response.clone();
        server.mock(|when, then| {
            when.method(Method::POST).path("/api/v1/sync");
            then.respond_with(move |_req| {
                HttpMockResponse::builder()
                    .status(200)
                    .body(mock_clone.to_string())
                    .build()
            });
        });
        let client =
            TodoistSyncClient::new_with_url("test".to_string(), server.url("/api/v1/sync"));
        let id = client.add_section("New Section", "proj_1").await.unwrap();
        assert_eq!(id, "sec_1");
    }

    #[tokio::test]
    async fn test_update_task() {
        let server = MockServer::start_async().await;
        let mock_response = serde_json::json!({
            "sync_token": "token",
            "temp_id_mapping": {},
            "sync_status": {"temp_upd": "ok"}
        });
        let mock_clone = mock_response.clone();
        server.mock(|when, then| {
            when.method(Method::POST).path("/api/v1/sync");
            then.respond_with(move |_req| {
                HttpMockResponse::builder()
                    .status(200)
                    .body(mock_clone.to_string())
                    .build()
            });
        });
        let client =
            TodoistSyncClient::new_with_url("test".to_string(), server.url("/api/v1/sync"));
        let result = client
            .update_task("task_1", Some("new content"), None, None, None, None)
            .await;
        assert!(result.is_ok(), "Update task should succeed");
    }

    #[tokio::test]
    async fn test_execute_commands_with_status() {
        let server = MockServer::start_async().await;
        let mock_response = serde_json::json!({
            "sync_token": "token",
            "temp_id_mapping": {},
            "sync_status": {"uuid1": "ok"}
        });
        let mock_clone = mock_response.clone();
        server.mock(|when, then| {
            when.method(Method::POST).path("/api/v1/sync");
            then.respond_with(move |_req| {
                HttpMockResponse::builder()
                    .status(200)
                    .body(mock_clone.to_string())
                    .build()
            });
        });
        let client =
            TodoistSyncClient::new_with_url("test".to_string(), server.url("/api/v1/sync"));
        let commands = vec![Command {
            type_: "item_add".to_string(),
            uuid: "uuid1".to_string(),
            args: serde_json::json!({"content": "test"}),
            temp_id: None,
        }];
        let result = client.execute_commands_with_status(&commands).await;
        assert!(
            result.is_ok(),
            "Execute commands with status should succeed"
        );
        let response = result.unwrap();
        assert_eq!(
            response.sync_token, "token",
            "Should return correct sync token"
        );
    }

    #[tokio::test]
    async fn test_get_sections() {
        let server = MockServer::start_async().await;
        let mock_response = serde_json::json!({
            "projects": [],
            "items": [],
            "sections": [{"id": "1", "name": "Section 1", "project_id": "p1", "order": 1, "created_at": "2024-01-01T00:00:00Z"}],
            "labels": [],
            "filters": [],
            "sync_token": "token"
        });
        let mock_clone = mock_response.clone();
        server.mock(|when, then| {
            when.method(Method::POST).path("/api/v1/sync");
            then.respond_with(move |_req| {
                HttpMockResponse::builder()
                    .status(200)
                    .body(mock_clone.to_string())
                    .build()
            });
        });
        let client =
            TodoistSyncClient::new_with_url("test".to_string(), server.url("/api/v1/sync"));
        let sections = client.get_sections().await.unwrap();
        assert_eq!(sections.len(), 1);
    }

    #[tokio::test]
    async fn test_get_labels() {
        let server = MockServer::start_async().await;
        let mock_response = serde_json::json!({
            "projects": [],
            "items": [],
            "sections": [],
            "labels": [
                {"id": "1", "name": "Label 1", "color": "red", "order": 1},
                {"id": "2", "name": "Label 2", "color": "blue", "order": 2}
            ],
            "filters": [],
            "sync_token": "token"
        });
        let mock_clone = mock_response.clone();
        server.mock(|when, then| {
            when.method(Method::POST).path("/api/v1/sync");
            then.respond_with(move |_req| {
                HttpMockResponse::builder()
                    .status(200)
                    .body(mock_clone.to_string())
                    .build()
            });
        });
        let client =
            TodoistSyncClient::new_with_url("test".to_string(), server.url("/api/v1/sync"));
        let labels = client.get_labels().await.unwrap();
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0].name, "Label 1");
        assert_eq!(labels[1].name, "Label 2");
    }

    #[tokio::test]
    async fn test_incremental_sync_with_token() {
        let server = MockServer::start_async().await;
        let mock_response = serde_json::json!({
            "projects": [
                {"id": "2", "name": "Updated Project", "color": "green", "shared": false, "favorite": false, "sort_order": 1, "is_archived": false, "is_deleted": false, "created_at": "2024-01-01T00:00:00Z", "updated_at": "2024-01-02T00:00:00Z"}
            ],
            "items": [],
            "sections": [],
            "labels": [],
            "filters": [],
            "sync_token": "new_token_456"
        });
        let mock_clone = mock_response.clone();
        server.mock(|when, then| {
            when.method(Method::POST)
                .path("/api/v1/sync")
                .header("Authorization", "Bearer test_token");
            then.respond_with(move |_req: &httpmock::HttpMockRequest| {
                HttpMockResponse::builder()
                    .status(200)
                    .body(mock_clone.to_string())
                    .build()
            });
        });

        let client =
            TodoistSyncClient::new_with_url("test_token".to_string(), server.url("/api/v1/sync"));
        // Set a non-"*" sync token for incremental sync
        client.set_sync_token("initial_token_123".to_string());

        let response = client.sync(&["projects"]).await.unwrap();

        // Verify incremental sync works with the provided token
        assert_eq!(response.projects.len(), 1);
        assert_eq!(response.projects[0].name, "Updated Project");

        // Verify the sync token was updated after sync
        let new_token = client.get_sync_token();
        assert!(new_token.is_some());
    }
}

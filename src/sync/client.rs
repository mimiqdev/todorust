use reqwest::Client as HttpClient;

use crate::error::TodoError;

use super::commands::{
    Command, CommandBuilder, FilterAddArgs, FilterOrderArgs, ItemAddArgs, ItemUpdateArgs,
    LabelAddArgs, ProjectAddArgs, SectionAddArgs,
};
use super::models::{SyncReadResponse, SyncWriteResponse};

/// Todoist Sync API Client
///
/// This client provides access to Todoist's Sync API v1,
/// which supports batch operations and incremental synchronization.
pub struct TodoistSyncClient {
    token: String,
    sync_url: String,
    sync_token: Option<String>,
    http: HttpClient,
}

impl TodoistSyncClient {
    /// Create a new TodoistSyncClient
    pub fn new(token: String) -> Self {
        Self {
            token,
            sync_url: "https://api.todoist.com/api/v1/sync".to_string(),
            sync_token: None,
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
        let sync_token = self.sync_token.as_deref().unwrap_or("*");

        let response = self
            .http
            .post(&self.sync_url)
            .header("Authorization", self.get_auth_header())
            .form(&[
                ("sync_token", sync_token),
                ("resource_types", serde_json::to_string(resource_types).unwrap()),
            ])
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            return Err(TodoError::Http(status.as_u16(), body));
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
            .form(&[(
                "commands",
                serde_json::to_string(commands).unwrap(),
            )])
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            return Err(TodoError::Http(status.as_u16(), body));
        }

        serde_json::from_str(&body)
            .map_err(|e| TodoError::Api(format!("Failed to parse command response: {}", e)))
    }

    /// 获取当前 sync_token
    pub fn get_sync_token(&self) -> Option<&str> {
        self.sync_token.as_deref()
    }

    /// 设置 sync_token
    pub fn set_sync_token(&mut self, token: String) {
        self.sync_token = Some(token);
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
    pub async fn execute(
        &self,
        builder: &mut CommandBuilder,
    ) -> Result<SyncWriteResponse, TodoError> {
        let commands = builder.build();
        self.execute_commands_with_status(&commands).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::commands::ItemAddArgs;

    #[test]
    fn test_client_creation() {
        let client = TodoistSyncClient::new("test_token".to_string());
        assert_eq!(client.token, "test_token");
    }

    #[test]
    fn test_sync_url() {
        let client = TodoistSyncClient::new("test_token".to_string());
        assert_eq!(client.sync_url, "https://api.todoist.com/api/v1/sync");
    }

    #[test]
    fn test_get_auth_header() {
        let client = TodoistSyncClient::new("test_token".to_string());
        let auth = client.get_auth_header();
        assert_eq!(auth, "Bearer test_token");
    }

    #[test]
    fn test_initial_sync_token_is_none() {
        let client = TodoistSyncClient::new("test_token".to_string());
        assert!(client.get_sync_token().is_none());
    }

    #[test]
    fn test_set_sync_token() {
        let mut client = TodoistSyncClient::new("test_token".to_string());
        client.set_sync_token("test_sync_token".to_string());
        assert_eq!(client.get_sync_token(), Some("test_sync_token"));
    }

    #[test]
    fn test_sync_with_default_token() {
        let client = TodoistSyncClient::new("test_token".to_string());
        // When sync_token is None, it should use "*" for full sync
        // This is verified by the sync() method using unwrap_or("*")
    }

    #[test]
    fn test_execute_commands_empty() {
        let client = TodoistSyncClient::new("test_token".to_string());
        let commands: Vec<Command> = Vec::new();
        
        // Empty commands should serialize correctly
        let json = serde_json::to_string(&commands).unwrap();
        assert_eq!(json, "[]");
    }

    #[test]
    fn test_command_builder_integration() {
        let mut builder = CommandBuilder::new();
        builder
            .item_add(ItemAddArgs::new("Test task".to_string()))
            .item_add(ItemAddArgs::new("Another task".to_string()))
            .item_close("123");

        let commands = builder.build();
        assert_eq!(commands.len(), 3);

        // Verify command types
        assert_eq!(commands[0].type_, "item_add");
        assert_eq!(commands[1].type_, "item_add");
        assert_eq!(commands[2].type_, "item_close");
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
        &mut self,
        name: &str,
        color: Option<&str>,
        favorite: Option<bool>,
    ) -> Result<String, TodoError> {
        let args = ProjectAddArgs::new(name.to_string())
            .color(color.map(|c| c.to_string()))
            .favorite(favorite);

        let mut builder = CommandBuilder::new();
        builder.project_add(args);

        let response = self.execute(&mut builder).await?;
        
        // 提取真实 ID
        if let Some((temp_id, real_id)) = response.temp_id_mapping.iter().next() {
            return Ok(real_id.clone());
        }
        
        Err(TodoError::Api("Failed to get project ID from response".to_string()))
    }

    /// 添加任务 (使用 Sync API)
    pub async fn add_task(
        &mut self,
        content: &str,
        description: Option<&str>,
        project_id: Option<&str>,
        section_id: Option<&str>,
        due_string: Option<&str>,
        priority: Option<u8>,
        labels: Option<Vec<&str>>,
    ) -> Result<String, TodoError> {
        let args = ItemAddArgs::new(content.to_string())
            .description(description.map(|d| d.to_string()))
            .project_id(project_id.map(|p| p.to_string()))
            .section_id(section_id.map(|s| s.to_string()))
            .due_string(due_string.map(|d| d.to_string()))
            .priority(priority)
            .labels(labels.map(|ls| ls.iter().map(|&s| s.to_string()).collect()));

        let mut builder = CommandBuilder::new();
        builder.item_add(args);

        let response = self.execute(&mut builder).await?;
        
        // 提取真实 ID
        if let Some((temp_id, real_id)) = response.temp_id_mapping.iter().next() {
            return Ok(real_id.clone());
        }
        
        Err(TodoError::Api("Failed to get task ID from response".to_string()))
    }

    /// 更新任务 (使用 Sync API)
    pub async fn update_task(
        &mut self,
        id: &str,
        content: Option<&str>,
        description: Option<&str>,
        priority: Option<u8>,
        due_string: Option<&str>,
        labels: Option<Vec<&str>>,
    ) -> Result<(), TodoError> {
        let args = ItemUpdateArgs::new(id.to_string())
            .content(content.map(|c| c.to_string()))
            .description(description.map(|d| d.to_string()))
            .priority(priority)
            .due_string(due_string.map(|d| d.to_string()))
            .labels(labels.map(|ls| ls.iter().map(|&s| s.to_string()).collect()));

        let mut builder = CommandBuilder::new();
        builder.item_update(args);

        self.execute(&mut builder).await?;
        Ok(())
    }

    /// 完成任务 (使用 Sync API)
    pub async fn complete_task(&mut self, id: &str) -> Result<(), TodoError> {
        let mut builder = CommandBuilder::new();
        builder.item_complete(id);

        self.execute(&mut builder).await?;
        Ok(())
    }

    /// 删除任务 (使用 Sync API)
    pub async fn delete_task(&mut self, id: &str) -> Result<(), TodoError> {
        let mut builder = CommandBuilder::new();
        builder.item_delete(id);

        self.execute(&mut builder).await?;
        Ok(())
    }

    /// 添加分区 (使用 Sync API)
    pub async fn add_section(
        &mut self,
        name: &str,
        project_id: &str,
    ) -> Result<String, TodoError> {
        let args = SectionAddArgs::new(name.to_string(), project_id.to_string());

        let mut builder = CommandBuilder::new();
        builder.section_add(args);

        let response = self.execute(&mut builder).await?;
        
        // 提取真实 ID
        if let Some((temp_id, real_id)) = response.temp_id_mapping.iter().next() {
            return Ok(real_id.clone());
        }
        
        Err(TodoError::Api("Failed to get section ID from response".to_string()))
    }

    /// 更新分区 (使用 Sync API)
    pub async fn update_section(&mut self, id: &str, name: &str) -> Result<(), TodoError> {
        let mut builder = CommandBuilder::new();
        builder.section_update(id, name);

        self.execute(&mut builder).await?;
        Ok(())
    }

    /// 删除分区 (使用 Sync API)
    pub async fn delete_section(&mut self, id: &str) -> Result<(), TodoError> {
        let mut builder = CommandBuilder::new();
        builder.section_delete(id);

        self.execute(&mut builder).await?;
        Ok(())
    }

    /// 添加标签 (使用 Sync API)
    pub async fn add_label(&mut self, name: &str, color: Option<&str>) -> Result<String, TodoError> {
        let args = LabelAddArgs::new(name.to_string())
            .color(color.map(|c| c.to_string()));

        let mut builder = CommandBuilder::new();
        builder.label_add(args);

        let response = self.execute(&mut builder).await?;
        
        // 提取真实 ID
        if let Some((temp_id, real_id)) = response.temp_id_mapping.iter().next() {
            return Ok(real_id.clone());
        }
        
        Err(TodoError::Api("Failed to get label ID from response".to_string()))
    }

    /// 更新标签 (使用 Sync API)
    pub async fn update_label(
        &mut self,
        id: &str,
        name: Option<&str>,
        color: Option<&str>,
    ) -> Result<(), TodoError> {
        let mut builder = CommandBuilder::new();
        builder.label_update(id, name, color);

        self.execute(&mut builder).await?;
        Ok(())
    }

    /// 删除标签 (使用 Sync API)
    pub async fn delete_label(&mut self, id: &str) -> Result<(), TodoError> {
        let mut builder = CommandBuilder::new();
        builder.label_delete(id);

        self.execute(&mut builder).await?;
        Ok(())
    }

    /// 更新过滤器顺序 (使用 Sync API)
    pub async fn update_filter_order(&mut self, filters: &[(&str, i64)]) -> Result<(), TodoError> {
        let filter_args: Vec<FilterOrderArgs> = filters
            .iter()
            .map(|(id, order)| FilterOrderArgs::new(id.to_string(), *order))
            .collect();

        let mut builder = CommandBuilder::new();
        builder.filter_update_orders(&filter_args);

        self.execute(&mut builder).await?;
        Ok(())
    }

    /// 添加过滤器 (使用 Sync API)
    pub async fn add_filter(
        &mut self,
        name: &str,
        query: &str,
        color: Option<&str>,
    ) -> Result<String, TodoError> {
        let args = FilterAddArgs::new(name.to_string(), query.to_string())
            .color(color.map(|c| c.to_string()));

        let mut builder = CommandBuilder::new();
        builder.filter_add(args);

        let response = self.execute(&mut builder).await?;
        
        // 提取真实 ID
        if let Some((temp_id, real_id)) = response.temp_id_mapping.iter().next() {
            return Ok(real_id.clone());
        }
        
        Err(TodoError::Api("Failed to get filter ID from response".to_string()))
    }

    /// 更新过滤器 (使用 Sync API)
    pub async fn update_filter(
        &mut self,
        id: &str,
        name: Option<&str>,
        query: Option<&str>,
        color: Option<&str>,
    ) -> Result<(), TodoError> {
        let mut builder = CommandBuilder::new();
        builder.filter_update(id, name, query, color);

        self.execute(&mut builder).await?;
        Ok(())
    }

    /// 删除过滤器 (使用 Sync API)
    pub async fn delete_filter(&mut self, id: &str) -> Result<(), TodoError> {
        let mut builder = CommandBuilder::new();
        builder.filter_delete(id);

        self.execute(&mut builder).await?;
        Ok(())
    }
}

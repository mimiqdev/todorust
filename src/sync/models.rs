//! # Sync API Models
//!
//! This module contains data structures specific to the Todoist Sync API.
//! These models handle the JSON responses from the sync endpoint.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::{Due, Project, Task};

/// Sync API 读取响应
#[derive(Debug, Deserialize, Serialize)]
pub struct SyncReadResponse {
    pub sync_token: String,
    #[serde(default)]
    pub full_sync: bool,
    #[serde(default)]
    pub projects: Vec<SyncProject>,
    #[serde(default)]
    pub items: Vec<SyncTask>,
    #[serde(default)]
    pub sections: Vec<SyncSection>,
    #[serde(default)]
    pub labels: Vec<SyncLabel>,
    #[serde(default)]
    pub filters: Vec<SyncFilter>,
}

/// Sync API 写入响应
#[derive(Debug, Deserialize, Serialize)]
pub struct SyncWriteResponse {
    pub sync_token: String,
    #[serde(default)]
    pub sync_status: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub temp_id_mapping: HashMap<String, String>,
}

/// Sync 项目
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncProject {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub color: String,
    #[serde(default, alias = "is_shared")]
    pub shared: bool,
    #[serde(default, alias = "is_favorite")]
    pub favorite: bool,
    #[serde(default, alias = "child_order")]
    pub sort_order: i64,
    #[serde(default)]
    pub is_archived: bool,
    #[serde(default)]
    pub is_deleted: bool,
    #[serde(default, alias = "added_at")]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// Sync 任务 (item)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncTask {
    pub id: String,
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub section_id: Option<String>,
    pub content: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub priority: u8,
    #[serde(default)]
    pub due: Option<SyncDue>,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default, alias = "child_order")]
    pub order: i64,
    #[serde(default)]
    pub indentation: i64,
    #[serde(default, alias = "checked")]
    pub is_completed: bool,
    #[serde(default)]
    pub is_archived: bool,
    #[serde(default)]
    pub is_deleted: bool,
    #[serde(default, alias = "added_at")]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

/// Sync 分区
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncSection {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub order: i64,
    #[serde(default)]
    pub is_archived: bool,
    #[serde(default)]
    pub is_deleted: bool,
    pub created_at: String,
    #[serde(default)]
    pub archived_at: Option<String>,
    #[serde(default)]
    pub is_collapsed: Option<bool>,
}

/// Sync 标签
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncLabel {
    pub id: String,
    pub name: String,
    pub color: String,
    #[serde(default)]
    pub is_favorite: bool,
}

/// Sync 过滤器
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncFilter {
    pub id: String,
    pub name: String,
    pub query: String,
}

/// Sync 截止日期
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncDue {
    pub date: Option<String>,
    #[serde(default)]
    pub is_recurring: bool,
    pub datetime: Option<String>,
    #[serde(default)]
    pub timezone: Option<String>,
}

// ==================== 类型转换 ====================

impl From<SyncProject> for Project {
    fn from(sync: SyncProject) -> Self {
        Self {
            id: sync.id,
            name: sync.name,
            color: sync.color,
            is_shared: sync.shared,
            is_favorite: sync.favorite,
        }
    }
}

impl From<SyncTask> for Task {
    fn from(sync: SyncTask) -> Self {
        Self {
            id: sync.id,
            content: sync.content,
            description: sync.description,
            project_id: sync.project_id,
            due: sync.due.map(|d| Due {
                date: d.date,
                is_recurring: Some(d.is_recurring),
                datetime: d.datetime,
            }),
            is_completed: sync.is_completed,
            created_at: sync.created_at,
            order: sync.order as i32,
            priority: sync.priority,
            labels: sync.labels,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_read_response_with_empty_sync_token() {
        let json = r#"{"sync_token": "", "full_sync": true}"#;
        let response: SyncReadResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.sync_token, "");
        assert!(response.full_sync);
        assert!(response.projects.is_empty());
        assert!(response.items.is_empty());
        assert!(response.sections.is_empty());
        assert!(response.labels.is_empty());
        assert!(response.filters.is_empty());
    }

    #[test]
    fn test_sync_write_response_with_empty_sync_token() {
        let json = r#"{"sync_token": ""}"#;
        let response: SyncWriteResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.sync_token, "");
        assert!(response.sync_status.is_empty());
        assert!(response.temp_id_mapping.is_empty());
    }

    #[test]
    fn test_sync_project_deserialization() {
        let json = r#"{
            "id": "123",
            "name": "Test Project",
            "color": "red",
            "shared": false,
            "favorite": true,
            "sort_order": 0,
            "is_archived": false,
            "is_deleted": false,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-02T00:00:00Z"
        }"#;

        let project: SyncProject = serde_json::from_str(json).unwrap();
        assert_eq!(project.id, "123");
        assert_eq!(project.name, "Test Project");
        assert_eq!(project.color, "red");
        assert!(!project.shared);
        assert!(project.favorite);
    }

    #[test]
    fn test_sync_task_deserialization() {
        let json = r#"{
            "id": "456",
            "content": "Test Task",
            "priority": 3,
            "order": 1,
            "is_completed": false,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-02T00:00:00Z"
        }"#;

        let task: SyncTask = serde_json::from_str(json).unwrap();
        assert_eq!(task.id, "456");
        assert_eq!(task.content, "Test Task");
        assert_eq!(task.priority, 3);
        assert!(!task.is_completed);
        assert!(task.description.is_none());
    }

    #[test]
    fn test_sync_section_deserialization() {
        let json = r#"{
            "id": "789",
            "project_id": "123",
            "name": "Section 1",
            "order": 1,
            "is_archived": false,
            "is_deleted": false,
            "created_at": "2024-01-01T00:00:00Z"
        }"#;

        let section: SyncSection = serde_json::from_str(json).unwrap();
        assert_eq!(section.id, "789");
        assert_eq!(section.project_id, "123");
        assert_eq!(section.name, "Section 1");
    }

    #[test]
    fn test_sync_label_deserialization() {
        let json = r#"{
            "id": "label1",
            "name": "Important",
            "color": "blue",
            "is_favorite": true
        }"#;

        let label: SyncLabel = serde_json::from_str(json).unwrap();
        assert_eq!(label.id, "label1");
        assert_eq!(label.name, "Important");
        assert_eq!(label.color, "blue");
        assert!(label.is_favorite);
    }

    #[test]
    fn test_sync_filter_deserialization() {
        let json = r#"{
            "id": "filter1",
            "name": "My Filter",
            "query": "today | overdue"
        }"#;

        let filter: SyncFilter = serde_json::from_str(json).unwrap();
        assert_eq!(filter.id, "filter1");
        assert_eq!(filter.name, "My Filter");
        assert_eq!(filter.query, "today | overdue");
    }

    #[test]
    fn test_sync_due_deserialization() {
        let json = r#"{
            "date": "2024-01-15",
            "is_recurring": true,
            "datetime": "2024-01-15T09:00:00",
            "timezone": "UTC"
        }"#;

        let due: SyncDue = serde_json::from_str(json).unwrap();
        assert_eq!(due.date, Some("2024-01-15".to_string()));
        assert!(due.is_recurring);
        assert_eq!(due.datetime, Some("2024-01-15T09:00:00".to_string()));
    }

    #[test]
    fn test_sync_due_optional_fields() {
        let json = r#"{
            "date": "2024-01-15"
        }"#;

        let due: SyncDue = serde_json::from_str(json).unwrap();
        assert_eq!(due.date, Some("2024-01-15".to_string()));
        assert!(!due.is_recurring);
        assert!(due.datetime.is_none());
    }

    #[test]
    fn test_sync_project_to_project_conversion() {
        let sync_project = SyncProject {
            id: "p1".to_string(),
            name: "My Project".to_string(),
            color: "green".to_string(),
            shared: true,
            favorite: false,
            sort_order: 10,
            is_archived: false,
            is_deleted: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-02T00:00:00Z".to_string(),
        };

        let project: Project = sync_project.into();
        assert_eq!(project.id, "p1");
        assert_eq!(project.name, "My Project");
        assert_eq!(project.color, "green");
        assert!(project.is_shared);
        assert!(!project.is_favorite);
    }

    #[test]
    fn test_sync_task_to_task_conversion() {
        let sync_task = SyncTask {
            id: "t1".to_string(),
            project_id: Some("p1".to_string()),
            section_id: None,
            content: "My Task".to_string(),
            description: Some("Task description".to_string()),
            priority: 4,
            due: Some(SyncDue {
                date: Some("2024-01-15".to_string()),
                is_recurring: false,
                datetime: None,
                timezone: None,
            }),
            labels: vec!["label1".to_string(), "label2".to_string()],
            order: 5,
            indentation: 0,
            is_completed: false,
            is_archived: false,
            is_deleted: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-02T00:00:00Z".to_string(),
        };

        let task: Task = sync_task.into();
        assert_eq!(task.id, "t1");
        assert_eq!(task.content, "My Task");
        assert_eq!(task.priority, 4);
        assert!(!task.is_completed);
        assert_eq!(task.labels.len(), 2);
    }

    #[test]
    fn test_sync_task_deserialization_legacy() {
        let json = r#"{
            "id": "456",
            "content": "Test Task",
            "priority": 3,
            "child_order": 5,
            "checked": true,
            "added_at": "2024-01-01T00:00:00Z"
        }"#;

        let task: SyncTask = serde_json::from_str(json).unwrap();
        assert_eq!(task.id, "456");
        assert_eq!(task.order, 5);
        assert!(task.is_completed);
        assert_eq!(task.created_at, "2024-01-01T00:00:00Z");
    }

    #[test]
    fn test_sync_project_deserialization_legacy() {
        let json = r#"{
            "id": "p1",
            "name": "Project 1",
            "child_order": 10,
            "is_favorite": true,
            "added_at": "2024-01-01T00:00:00Z"
        }"#;

        let project: SyncProject = serde_json::from_str(json).unwrap();
        assert_eq!(project.id, "p1");
        assert_eq!(project.sort_order, 10);
        assert!(project.favorite);
        assert_eq!(project.created_at, "2024-01-01T00:00:00Z");
    }
}

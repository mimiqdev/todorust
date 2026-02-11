use reqwest::Client as HttpClient;
use serde::Deserialize;
use std::collections::HashMap;

use crate::error::TodoError;

/// Sync API 读取响应
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
pub struct SyncWriteResponse {
    pub sync_token: String,
    #[serde(default)]
    pub sync_status: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub temp_id_mapping: HashMap<String, String>,
}

/// Sync 项目
#[derive(Debug, Clone, Deserialize)]
pub struct SyncProject {
    pub id: String,
    pub name: String,
    pub color: String,
    #[serde(default)]
    pub shared: bool,
    #[serde(default)]
    pub favorite: bool,
    pub sort_order: i64,
    #[serde(default)]
    pub is_archived: bool,
    #[serde(default)]
    pub is_deleted: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Sync 任务 (item)
#[derive(Debug, Clone, Deserialize)]
pub struct SyncTask {
    pub id: String,
    #[serde(default)]
    pub project_id: Option<String>,
    #[serde(default)]
    pub section_id: Option<String>,
    pub content: String,
    #[serde(default)]
    pub description: Option<String>,
    pub priority: u8,
    #[serde(default)]
    pub due: Option<SyncDue>,
    #[serde(default)]
    pub labels: Vec<String>,
    pub order: i64,
    #[serde(default)]
    pub indentation: i64,
    #[serde(default)]
    pub is_completed: bool,
    #[serde(default)]
    pub is_archived: bool,
    #[serde(default)]
    pub is_deleted: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Sync 分区
#[derive(Debug, Clone, Deserialize)]
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
}

/// Sync 标签
#[derive(Debug, Clone, Deserialize)]
pub struct SyncLabel {
    pub id: String,
    pub name: String,
    pub color: String,
    #[serde(default)]
    pub is_favorite: bool,
}

/// Sync 过滤器
#[derive(Debug, Clone, Deserialize)]
pub struct SyncFilter {
    pub id: String,
    pub name: String,
    pub query: String,
}

/// Sync 截止日期
#[derive(Debug, Clone, Deserialize)]
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

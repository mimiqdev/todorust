# Todoist Sync API 详细开发计划

> **创建日期**: 2026-02-11
> **版本**: v1.0
> **状态**: 规划完成

---

## 目录

1. [当前 API 使用情况分析](#1-当前-api-使用情况分析)
2. [Sync API 映射分析](#2-sync-api-映射分析)
3. [分阶段实施计划](#3-分阶段实施计划)
4. [技术细节设计](#4-技术细节设计)
5. [API 兼容性策略](#5-api-兼容性策略)
6. [测试策略](#6-测试策略)
7. [风险评估与缓解](#7-风险评估与缓解)

---

## 1. 当前 API 使用情况分析

### 1.1 当前使用的 REST API 端点

基于 `src/api.rs` 源代码分析：

| 功能 | HTTP 方法 | 端点 | 使用频率 | 优先级 |
|-----|----------|------|---------|-------|
| 获取项目 | GET | `/projects` | 高 | P0 |
| 获取任务 | GET | `/tasks` | 高 | P0 |
| 过滤任务 | GET | `/tasks/filter` | 中 | P0 |
| 获取已完成任务 | GET | `/tasks/completed/by_completion_date` | 低 | P1 |
| 创建任务 | POST | `/tasks` | 高 | P0 |
| 完成任务 | POST | `/tasks/{id}/close` | 中 | P0 |
| 重新打开任务 | POST | `/tasks/{id}/reopen` | 低 | P0 |
| 删除任务 | DELETE | `/tasks/{id}` | 低 | P0 |
| 获取过滤器 | POST | `/sync` | 低 | P2 |

### 1.2 资源操作统计

| 资源类型 | 创建 (C) | 读取 (R) | 更新 (U) | 删除 (D) |
|---------|---------|---------|---------|---------|
| Projects | 0 | 1 | 0 | 0 |
| Tasks | 1 | 3 | 0 | 1 |
| Filters | 0 | 1 | 0 | 0 |
| Sections | 0 | 0 | 0 | 0 |
| Labels | 0 | 0 | 0 | 0 |

### 1.3 当前代码结构

```
src/api.rs
├── TodoistClient 结构体
│   ├── token: String
│   ├── base_url: "https://api.todoist.com/api/v1"
│   └── http: HttpClient
│
├── 方法列表
│   ├── get_projects() → GET /projects
│   ├── get_tasks(filter) → 多端点路由
│   │   ├── filter.contains("completed") → /tasks/completed/by_completion_date
│   │   ├── filter.is_some() → /tasks/filter
│   │   └── default → /tasks
│   ├── create_task() → POST /tasks
│   ├── complete_task() → POST /tasks/{id}/close
│   ├── reopen_task() → POST /tasks/{id}/reopen
│   ├── delete_task() → DELETE /tasks/{id}
│   └── get_filters() → POST /sync (仅用于 filters)
│
└── 辅助方法
    ├── enrich_tasks() - 丰富任务数据
    └── CreateTaskRequest - 请求结构体
```

### 1.4 当前数据模型

```rust
// models.rs 中定义的数据结构
pub struct Project {
    pub id: String,
    pub name: String,
    pub color: String,
    pub is_shared: bool,
    pub is_favorite: bool,
}

pub struct Task {
    pub id: String,
    pub content: String,
    pub description: Option<String>,
    pub project_id: Option<String>,
    pub due: Option<Due>,
    pub is_completed: bool,
    pub created_at: String,
    pub order: i32,
    pub priority: u8,
    pub labels: Vec<String>,
}

pub struct TaskOutput {
    pub id: String,
    pub content: String,
    pub description: Option<String>,
    pub project_id: Option<String>,
    pub project_name: Option<String>,  // 丰富字段
    pub due_date: Option<String>,
    pub is_completed: bool,
    pub created_at: String,
    pub order: i32,
    pub priority: u8,
    pub labels: Vec<String>,
}
```

### 1.5 未使用的功能

| 功能 | 当前状态 | Sync API 支持 | 优先级 |
|-----|---------|-------------|-------|
| Sections (分区) | 未实现 | ✅ 完整 CRUD | P0 |
| Labels (标签) | 仅数据模型 | ✅ 完整 CRUD | P1 |
| Notes (备注) | 未实现 | ✅ 完整 CRUD | P2 |
| Reminders (提醒) | 未实现 | ✅ 完整 CRUD | P2 |
| 增量同步 | 未实现 | ✅ sync_token | P1 |
| 批量操作 | 未实现 | ✅ commands | P1 |

---

## 2. Sync API 映射分析

### 2.1 读取资源映射

| 当前端点 | Sync Command | 参数转换 | 响应差异 |
|---------|-------------|---------|---------|
| `GET /projects` | `sync(resource_types: ["projects"])` | `["projects"]` → `"projects"` | 数组直接返回，无 `results` 包装 |
| `GET /tasks` | `sync(resource_types: ["items"])` | `["items"]` → `"items"` | `items` vs `results` 字段名 |
| `GET /tasks/filter` | `sync(resource_types: ["items"])` + 过滤 | 需客户端过滤 | 需本地实现过滤逻辑 |
| `GET /tasks/completed/by_completion_date` | `sync(resource_types: ["completed_info"])` | 完全不同 | `completed_info` 数组结构 |
| `POST /sync` (filters) | `sync(resource_types: ["filters"])` | 相同 | 相同 |

### 2.2 写入资源映射

| 操作 | 当前方法 | Sync Command | 临时 ID 支持 |
|-----|---------|-------------|------------|
| 创建任务 | `POST /tasks` | `item_add` | ✅ 支持 |
| 完成任务 | `POST /tasks/{id}/close` | `item_close` | N/A |
| 重新打开任务 | `POST /tasks/{id}/reopen` | `item_reopen` | N/A |
| 删除任务 | `DELETE /tasks/{id}` | `item_delete` | N/A |
| 创建项目 | 未实现 | `project_add` | ✅ 支持 |
| 更新项目 | 未实现 | `project_update` | N/A |
| 删除项目 | 未实现 | `project_delete` | N/A |
| 创建分区 | 未实现 | `section_add` | ✅ 支持 |
| 更新分区 | 未实现 | `section_update` | N/A |
| 删除分区 | 未实现 | `section_delete` | N/A |
| 移动任务 | 未实现 | `item_move` | ✅ 支持 |

### 2.3 参数转换详情

#### 2.3.1 项目参数

**REST API**:
```json
{
  "name": "Project Name",
  "color": "blue",
  "is_favorite": true
}
```

**Sync API Command**:
```json
{
  "type": "project_add",
  "uuid": "unique-command-uuid",
  "temp_id": "unique-temp-id",
  "args": {
    "name": "Project Name",
    "color": "berry_red",
    "favorite": true
  }
}
```

**字段映射**:
| REST | Sync | 说明 |
|-----|------|------|
| `name` | `name` | 相同 |
| `color` | `color` | 颜色名称不同，需映射表 |
| `is_favorite` | `favorite` | 布尔值反转 |

#### 2.3.2 任务参数

**REST API**:
```json
{
  "content": "Task content",
  "description": "Description",
  "project_id": "123",
  "due_string": "tomorrow",
  "priority": 4,
  "labels": ["label1", "label2"]
}
```

**Sync API Command**:
```json
{
  "type": "item_add",
  "uuid": "unique-command-uuid",
  "temp_id": "unique-temp-id",
  "args": {
    "content": "Task content",
    "description": "Description",
    "project_id": "123",
    "due_string": "tomorrow",
    "priority": 4,
    "labels": ["label1", "label2"]
  }
}
```

**字段映射**:
| REST | Sync | 说明 |
|-----|------|------|
| `content` | `content` | 相同 |
| `description` | `description` | 相同 |
| `project_id` | `project_id` | 相同 |
| `due_string` | `due_string` | 相同 |
| `priority` | `priority` | 相同 (1=高, 4=低) |
| `labels` | `labels` | 相同 |

### 2.4 响应数据结构差异

#### 2.4.1 Sync 读取响应

```json
{
  "sync_token": "TnYUZEpuzf2FMA9qzyY3j4xky6dXiYejmSO85S5paZ_a9y1FI85mBbIWZGpW",
  "full_sync": true,
  "projects": [...],
  "items": [...],
  "sections": [...],
  "labels": [...],
  "filters": [...]
}
```

#### 2.4.2 Sync 命令响应

```json
{
  "sync_token": "TnYUZEpuzf2FMA9...",
  "sync_status": {
    "uuid-1": "ok",
    "uuid-2": { "error": "..." }
  },
  "temp_id_mapping": {
    "temp-uuid-1": "real-id-123"
  }
}
```

---

## 3. 分阶段实施计划

### 阶段 1: 基础架构重构 (预计 3-5 天)

**目标**: 创建 `TodoistSyncClient`，保持功能等价

#### 3.1.1 目录结构

```
src/sync/
├── mod.rs           # 模块入口
├── client.rs        # TodoistSyncClient
├── models.rs        # 数据模型
└── commands.rs      # Command 构建器
```

#### 3.1.2 新增数据结构

```rust
// src/sync/models.rs

/// Sync API 命令结构
#[derive(Debug, Serialize)]
pub struct Command {
    #[serde(rename = "type")]
    pub type_: String,
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_id: Option<String>,
    pub args: serde_json::Value,
}

/// Sync 读取响应
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

/// Sync 写入响应
#[derive(Debug, Deserialize)]
pub struct SyncWriteResponse {
    pub sync_token: String,
    #[serde(default)]
    pub sync_status: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub temp_id_mapping: HashMap<String, String>,
}

/// Sync 项目
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
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
```

#### 3.1.3 新增 Sync Client

```rust
// src/sync/client.rs

pub struct TodoistSyncClient {
    token: String,
    sync_url: String,
    sync_token: Option<String>,
    http: HttpClient,
}

impl TodoistSyncClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            sync_url: "https://api.todoist.com/api/v1/sync".to_string(),
            sync_token: None,
            http: HttpClient::new(),
        }
    }

    /// 基础同步请求（读取资源）
    pub async fn sync(
        &self,
        resource_types: &[&str],
    ) -> Result<SyncReadResponse, TodoError> {
        let response = self
            .http
            .post(&self.sync_url)
            .header("Authorization", format!("Bearer {}", self.token))
            .form(&[
                ("sync_token", self.sync_token.as_deref().unwrap_or("*")),
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
    pub async fn execute_commands(
        &self,
        commands: &[Command],
    ) -> Result<SyncWriteResponse, TodoError> {
        let response = self
            .http
            .post(&self.sync_url)
            .header("Authorization", format!("Bearer {}", self.token))
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
}
```

#### 3.1.4 迁移现有方法

```rust
impl TodoistSyncClient {
    /// 迁移 get_projects()
    pub async fn get_projects(&self) -> Result<Vec<Project>, TodoError> {
        let response = self.sync(&["projects"]).await?;
        Ok(response.projects.into_iter().map(convert_sync_project).collect())
    }

    /// 迁移 get_tasks()
    pub async fn get_tasks(&self, filter: Option<&str>) -> Result<Vec<TaskOutput>, TodoError> {
        let response = self.sync(&["projects", "items", "sections"]).await?;

        let mut tasks: Vec<_> = response.items.into_iter()
            .filter(|t| !t.is_deleted && !t.is_archived)
            .map(convert_sync_task)
            .collect();

        if let Some(f) = filter {
            tasks = apply_task_filter(&tasks, f);
        }

        // 丰富项目名称和分区名称
        let project_map: HashMap<_, _> = response.projects
            .into_iter()
            .map(|p| (p.id.clone(), p.name))
            .collect();

        let section_map: HashMap<_, _> = response.sections
            .into_iter()
            .map(|s| (s.id.clone(), s.name))
            .collect();

        Ok(tasks.into_iter()
            .map(|mut t| {
                t.project_name = t.project_id.as_ref()
                    .and_then(|id| project_map.get(id)).cloned();
                t.section_name = t.section_id.as_ref()
                    .and_then(|id| section_map.get(id)).cloned();
                t
            })
            .collect())
    }
}
```

#### 3.1.5 阶段 1 里程碑

- [ ] 创建 `src/sync/` 目录结构
- [ ] 实现 `TodoistSyncClient` 基本结构
- [ ] 实现 `sync()` 读取方法
- [ ] 实现 `execute_commands()` 写入方法
- [ ] 迁移 `get_projects()` → Sync API
- [ ] 迁移 `get_tasks()` → Sync API (含过滤)
- [ ] 迁移 `create_task()` → Sync API command
- [ ] 所有现有单元测试通过
- [ ] 集成测试覆盖基本流程

---

### 阶段 2: 读操作迁移 (预计 2-3 天)

**目标**: 完成所有读取操作的迁移

#### 3.2.1 迁移剩余读取方法

```rust
impl TodoistSyncClient {
    /// 迁移 get_filters()
    pub async fn get_filters(&self) -> Result<Vec<Filter>, TodoError> {
        let response = self.sync(&["filters"]).await?;
        Ok(response.filters.into_iter().map(convert_sync_filter).collect())
    }

    /// 新增 get_sections()
    pub async fn get_sections(&self, project_id: Option<&str>) -> Result<Vec<Section>, TodoError> {
        let response = self.sync(&["sections"]).await?;

        let sections: Vec<_> = response.sections.into_iter()
            .filter(|s| !s.is_deleted && !s.is_archived)
            .filter(|s| project_id.map(|pid| &s.project_id == pid).unwrap_or(true))
            .map(convert_sync_section)
            .collect();

        Ok(sections)
    }

    /// 新增 get_labels()
    pub async fn get_labels(&self) -> Result<Vec<Label>, TodoError> {
        let response = self.sync(&["labels"]).await?;
        Ok(response.labels.into_iter().map(convert_sync_label).collect())
    }

    /// 新增 get_section_tasks()
    pub async fn get_section_tasks(&self, section_id: &str) -> Result<Vec<TaskOutput>, TodoError> {
        let response = self.sync(&["projects", "items", "sections"]).await?;

        let tasks: Vec<_> = response.items.into_iter()
            .filter(|t| t.section_id.as_deref() == Some(section_id))
            .filter(|t| !t.is_deleted && !t.is_archived)
            .map(convert_sync_task)
            .collect();

        // 丰富数据...
        Ok(tasks)
    }
}
```

#### 3.2.2 新增 Section 模型

```rust
// src/sync/models.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub order: i64,
    pub is_archived: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub id: String,
    pub name: String,
    pub color: String,
    pub is_favorite: bool,
}
```

#### 3.2.3 阶段 2 里程碑

- [ ] 迁移 `get_filters()` → Sync API
- [ ] 实现 `get_sections()` → Sync API
- [ ] 实现 `get_labels()` → Sync API
- [ ] 实现 `get_section_tasks()` → 新功能
- [ ] 创建 `Section` 数据模型
- [ ] 创建 `Label` 数据模型
- [ ] 更新 `TaskOutput` 添加 `section_id` 和 `section_name`
- [ ] 测试覆盖所有读取路径

---

### 阶段 3: 写操作迁移 (预计 3-5 天)

**目标**: 迁移所有 CRUD 操作到 Sync commands

#### 3.3.1 Command 构建器

```rust
// src/sync/commands.rs

use uuid::Uuid;

pub struct CommandBuilder {
    commands: Vec<Command>,
}

impl CommandBuilder {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn item_add(&mut self, args: ItemAddArgs) -> &mut Self {
        self.commands.push(Command {
            type_: "item_add".to_string(),
            uuid: Uuid::new_v4().to_string(),
            temp_id: Some(Uuid::new_v4().to_string()),
            args: serde_json::to_value(args).unwrap(),
        });
        self
    }

    pub fn item_close(&mut self, id: &str) -> &mut Self {
        self.commands.push(Command {
            type_: "item_close".to_string(),
            uuid: Uuid::new_v4().to_string(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    pub fn item_reopen(&mut self, id: &str) -> &mut Self {
        self.commands.push(Command {
            type_: "item_reopen".to_string(),
            uuid: Uuid::new_v4().to_string(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    pub fn item_delete(&mut self, id: &str) -> &mut Self {
        self.commands.push(Command {
            type_: "item_delete".to_string(),
            uuid: Uuid::new_v4().to_string(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    pub fn item_move(&mut self, id: &str, project_id: &str, section_id: Option<&str>) -> &mut Self {
        let mut args = serde_json::json!({
            "id": id,
            "project_id": project_id
        });
        if let Some(sid) = section_id {
            args["section_id"] = serde_json::json!(sid);
        }
        self.commands.push(Command {
            type_: "item_move".to_string(),
            uuid: Uuid::new_v4().to_string(),
            temp_id: None,
            args,
        });
        self
    }

    pub fn project_add(&mut self, args: ProjectAddArgs) -> &mut Self {
        self.commands.push(Command {
            type_: "project_add".to_string(),
            uuid: Uuid::new_v4().to_string(),
            temp_id: Some(Uuid::new_v4().to_string()),
            args: serde_json::to_value(args).unwrap(),
        });
        self
    }

    pub fn section_add(&mut self, args: SectionAddArgs) -> &mut Self {
        self.commands.push(Command {
            type_: "section_add".to_string(),
            uuid: Uuid::new_v4().to_string(),
            temp_id: Some(Uuid::new_v4().to_string()),
            args: serde_json::to_value(args).unwrap(),
        });
        self
    }

    pub fn section_update(&mut self, id: &str, name: &str) -> &mut Self {
        self.commands.push(Command {
            type_: "section_update".to_string(),
            uuid: Uuid::new_v4().to_string(),
            temp_id: None,
            args: serde_json::json!({ "id": id, "name": name }),
        });
        self
    }

    pub fn section_delete(&mut self, id: &str) -> &mut Self {
        self.commands.push(Command {
            type_: "section_delete".to_string(),
            uuid: Uuid::new_v4().to_string(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    pub fn build(self) -> Vec<Command> {
        self.commands
    }
}

// 参数结构体
#[derive(Debug, Serialize)]
pub struct ItemAddArgs {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct ProjectAddArgs {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct SectionAddArgs {
    pub name: String,
    pub project_id: String,
}
```

#### 3.3.2 实现写操作方法

```rust
impl TodoistSyncClient {
    /// 迁移 complete_task()
    pub async fn complete_task(&self, task_id: &str) -> Result<(), TodoError> {
        let mut builder = CommandBuilder::new();
        builder.item_close(task_id);
        let response = self.execute_commands(&builder.build()).await?;
        self.check_sync_status(&response)
    }

    /// 迁移 reopen_task()
    pub async fn reopen_task(&self, task_id: &str) -> Result<(), TodoError> {
        let mut builder = CommandBuilder::new();
        builder.item_reopen(task_id);
        let response = self.execute_commands(&builder.build()).await?;
        self.check_sync_status(&response)
    }

    /// 迁移 delete_task()
    pub async fn delete_task(&self, task_id: &str) -> Result<(), TodoError> {
        let mut builder = CommandBuilder::new();
        builder.item_delete(task_id);
        let response = self.execute_commands(&builder.build()).await?;
        self.check_sync_status(&response)
    }

    /// 辅助方法：检查命令状态
    fn check_sync_status(&self, response: &SyncWriteResponse) -> Result<(), TodoError> {
        for (uuid, status) in &response.sync_status {
            if status == "ok" {
                continue;
            }
            return Err(TodoError::SyncCommandFailed(format!(
                "Command {} failed: {}",
                uuid, status
            )));
        }
        Ok(())
    }
}
```

#### 3.3.3 阶段 3 里程碑

- [ ] 实现 `CommandBuilder` 模式
- [ ] 迁移 `complete_task()` → Sync command
- [ ] 迁移 `reopen_task()` → Sync command
- [ ] 迁移 `delete_task()` → Sync command
- [ ] 实现 `create_tasks_batch()` → 批量操作
- [ ] 实现 `move_task()` → 新功能
- [ ] 实现 `project_add()` → 新功能
- [ ] 实现 `section_add/update/delete()` → 新功能
- [ ] 测试覆盖所有写入路径

---

### 阶段 4: 完整 Section 支持 (预计 2-3 天)

**目标**: 添加完整的 Section 操作支持

#### 3.4.1 Section 操作方法

```rust
impl TodoistSyncClient {
    /// 获取项目下的所有分区
    pub async fn get_project_sections(
        &self,
        project_id: &str,
    ) -> Result<Vec<Section>, TodoError> {
        let response = self.sync(&["sections"]).await?;

        let sections: Vec<_> = response.sections.into_iter()
            .filter(|s| s.project_id == project_id)
            .filter(|s| !s.is_deleted && !s.is_archived)
            .map(|s| Section {
                id: s.id,
                project_id: s.project_id,
                name: s.name,
                order: s.order,
                is_archived: s.is_archived,
                created_at: s.created_at,
            })
            .collect();

        Ok(sections)
    }

    /// 创建分区
    pub async fn create_section(
        &self,
        name: &str,
        project_id: &str,
    ) -> Result<Section, TodoError> {
        let mut builder = CommandBuilder::new();
        builder.section_add(SectionAddArgs {
            name: name.to_string(),
            project_id: project_id.to_string(),
        });

        let response = self.execute_commands(&builder.build()).await?;
        self.check_sync_status(&response)?;

        // 获取新分区的实际 ID
        let new_section_id = response.temp_id_mapping.values().next()
            .ok_or_else(|| TodoError::SyncCommandFailed(
                "No temp_id_mapping for new section".to_string()
            ))?.clone();

        // 获取完整的分区数据
        let sections = self.get_project_sections(project_id).await?;
        sections.into_iter()
            .find(|s| s.id == new_section_id)
            .ok_or_else(|| TodoError::SyncCommandFailed(
                "Created section not found".to_string()
            ))
    }

    /// 更新分区名称
    pub async fn update_section_name(
        &self,
        section_id: &str,
        name: &str,
    ) -> Result<(), TodoError> {
        let mut builder = CommandBuilder::new();
        builder.section_update(section_id, name);
        let response = self.execute_commands(&builder.build()).await?;
        self.check_sync_status(&response)
    }

    /// 删除分区
    pub async fn delete_section(&self, section_id: &str) -> Result<(), TodoError> {
        let mut builder = CommandBuilder::new();
        builder.section_delete(section_id);
        let response = self.execute_commands(&builder.build()).await?;
        self.check_sync_status(&response)
    }
}
```

#### 3.4.2 更新 TaskOutput 模型

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskOutput {
    pub id: String,
    pub content: String,
    pub description: Option<String>,
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub section_id: Option<String>,       // 新增
    pub section_name: Option<String>,     // 新增
    pub due_date: Option<String>,
    pub is_completed: bool,
    pub created_at: String,
    pub order: i32,
    pub priority: u8,
    pub labels: Vec<String>,
}
```

#### 3.4.3 阶段 4 里程碑

- [ ] 实现 `get_project_sections()` 完整查询
- [ ] 实现 `create_section()` 创建分区
- [ ] 实现 `update_section_name()` 更新分区
- [ ] 实现 `delete_section()` 删除分区
- [ ] 更新 `TaskOutput` 模型添加分区字段
- [ ] 更新 `formatter.rs` 显示分区信息
- [ ] 测试覆盖所有 Section 操作

---

### 阶段 5: 旧代码清理和优化 (预计 2-3 天)

**目标**: 移除冗余代码，优化性能

#### 3.5.1 清理策略

```rust
// Cargo.toml 添加 feature flag
[features]
default = ["sync"]
legacy-client = []

// lib.rs 选择性导出
#[cfg(feature = "sync")]
pub use sync::client::TodoistSyncClient as TodoistClient;

#[cfg(not(feature = "sync"))]
pub use api::TodoistClient;
```

#### 3.5.2 优化项目

1. **连接池优化**
   ```rust
   // 复用 HttpClient，避免每个请求新建连接
   http: HttpClient::builder()
       .pool_idle_timeout(Duration::from_secs(300))
       .build()?
   ```

2. **Sync Token 持久化**
   ```rust
   // 存储到配置目录
   pub fn save_sync_token(token: &str) -> Result<(), TodoError> {
       let config_dir = dirs::config_dir()
           .ok_or_else(|| TodoError::Config("Config dir not found".to_string()))?;
       let token_path = config_dir.join("todorust_sync_token");
       std::fs::write(token_path, token)?;
       Ok(())
   }
   ```

3. **本地过滤缓存**
   ```rust
   // 对常用查询结果进行缓存
   struct SyncCache {
       projects: Vec<Project>,
       tasks: Vec<TaskOutput>,
       sections: Vec<Section>,
       last_sync: Instant,
   }
   ```

#### 3.5.3 阶段 5 里程碑

- [ ] 添加 Cargo.toml feature flags
- [ ] 更新 lib.rs 选择性导出
- [ ] 更新 main.rs 使用新 Client
- [ ] 实现 HttpClient 连接池
- [ ] 实现 sync_token 持久化
- [ ] 移除旧 API 代码或标记为废弃
- [ ] 更新 README.md 文档
- [ ] 最终集成测试

---

## 4. 技术细节设计

### 4.1 Command UUID 生成策略

```rust
// 使用 uuid crate 生成唯一 ID
use uuid::Uuid;

impl CommandBuilder {
    fn generate_uuid(&self) -> String {
        Uuid::new_v4().to_string()
    }

    fn generate_temp_id(&self) -> String {
        // 临时 ID 使用 UUID v4，确保全局唯一
        Uuid::new_v4().to_string()
    }
}
```

**设计决策**:
- UUID v4 提供足够的随机性，避免碰撞
- 每个 command 独立生成 UUID，支持幂等性
- temp_id 仅在创建资源时使用

### 4.2 临时 ID 映射处理

```rust
impl TodoistSyncClient {
    /// 创建任务并返回映射
    pub async fn create_task_with_mapping(
        &self,
        request: CreateTaskRequest,
    ) -> Result<(String, TaskOutput), TodoError> {
        let mut builder = CommandBuilder::new();
        let temp_id = builder.generate_temp_id();
        builder.item_add(ItemAddArgs::from(request.clone()));

        let response = self.execute_commands(&builder.build()).await?;
        self.check_sync_status(&response)?;

        // 从映射获取真实 ID
        let real_id = response.temp_id_mapping
            .get(&temp_id)
            .ok_or_else(|| TodoError::SyncCommandFailed(
                "Failed to get temp_id mapping".to_string()
            ))?.clone();

        // 获取完整任务数据
        let tasks = self.get_tasks(None).await?;
        let task_output = tasks.into_iter()
            .find(|t| t.id == real_id)
            .ok_or_else(|| TodoError::SyncCommandFailed(
                "Created task not found".to_string()
            ))?;

        Ok((real_id, task_output))
    }
}
```

### 4.3 Sync Token 管理

```rust
impl TodoistSyncClient {
    /// 增量同步
    pub async fn incremental_sync(&mut self) -> Result<SyncReadResponse, TodoError> {
        let token = self.sync_token.as_deref().unwrap_or("*");
        let response = self.sync_with_token(token).await?;
        self.sync_token = Some(response.sync_token.clone());
        Ok(response)
    }

    /// 强制全量同步
    pub async fn full_sync(&mut self) -> Result<SyncReadResponse, TodoError> {
        self.sync_token = None;
        self.incremental_sync().await
    }

    /// 获取当前 sync_token
    pub fn get_sync_token(&self) -> Option<&str> {
        self.sync_token.as_deref()
    }
}
```

### 4.4 错误处理策略

```rust
// error.rs 新增错误类型

#[derive(Debug)]
pub enum TodoError {
    // 现有错误...
    SyncCommandFailed(String),
    SyncStatusError {
        uuid: String,
        error: serde_json::Value,
    },
    TempIdMappingError(String),
}

// 错误解析辅助函数
impl TodoistSyncClient {
    fn parse_sync_error(&self, error: &serde_json::Value) -> TodoError {
        let error_tag = error.get("error_tag")
            .and_then(|v| v.as_str())
            .unwrap_or("UNKNOWN");

        let error_code = error.get("error_code")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let error_msg = error.get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown error");

        let error_extra = error.get("error_extra")
            .map(|v| v.to_string())
            .unwrap_or_default();

        TodoError::SyncStatusError {
            uuid: "unknown".to_string(), // 实际从 context 获取
            error: serde_json::json!({
                "tag": error_tag,
                "code": error_code,
                "message": error_msg,
                "extra": error_extra
            })
        }
    }
}
```

---

## 5. API 兼容性策略

### 5.1 渐进式迁移方案

采用 **Feature Flag** 方式实现渐进式迁移，允许在迁移期间同时支持新旧 API：

```rust
// Cargo.toml
[features]
default = ["sync"]
legacy = []

// lib.rs
#[cfg(feature = "sync")]
mod sync;

#[cfg(feature = "sync")]
pub use sync::client::TodoistSyncClient;

#[cfg(not(feature = "sync"))]
pub use api::TodoistClient;
```

### 5.2 公共 API 保持不变

**设计原则**: 对外暴露的 `TodoistClient` 接口保持不变，内部实现切换到 Sync API。

```rust
// 用户的调用代码无需修改
let client = TodoistClient::new(token);
let tasks = client.get_tasks(None).await?;
let task = client.create_task("New task", None, None, None, None, None).await?;
client.complete_task(&task.id).await?;
```

### 5.3 CLI 命令兼容性

```rust
// main.rs 保持现有命令结构
#[derive(Subcommand)]
enum Commands {
    Tasks { filter: Option<String> },
    Projects,
    Filters,
    Create { content: String, ... },
    Complete { task_id: String },
    Reopen { task_id: String },
    // 新增 Section 命令
    Sections {
        #[arg(long)]
        project_id: Option<String>,
    },
    CreateSection {
        #[arg(long)]
        name: String,
        #[arg(long)]
        project_id: String,
    },
    DeleteSection {
        #[arg(long)]
        section_id: String,
    },
}
```

### 5.4 返回值向后兼容

```rust
// TaskOutput 添加可选字段，不破坏现有使用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskOutput {
    pub id: String,
    pub content: String,
    // ... 现有字段保持不变 ...
    pub section_id: Option<String>,    // 新增，默认为 None
    pub section_name: Option<String>,  // 新增，默认为 None
}
```

---

## 6. 测试策略

### 6.1 Mock API 响应

```rust
// tests/mocks/sync_responses.rs

pub fn mock_sync_response_full() -> &'static str {
    r#"{
        "sync_token": "mock_sync_token_12345",
        "full_sync": true,
        "projects": [
            {
                "id": "123",
                "name": "Work",
                "color": "berry_red",
                "shared": false,
                "favorite": true,
                "sort_order": 1,
                "is_archived": false,
                "is_deleted": false,
                "created_at": "2024-01-15T10:00:00Z",
                "updated_at": "2024-01-15T10:00:00Z"
            }
        ],
        "items": [
            {
                "id": "456",
                "project_id": "123",
                "section_id": null,
                "content": "Test Task",
                "description": "",
                "priority": 4,
                "due": null,
                "labels": [],
                "order": 1,
                "indentation": 1,
                "is_completed": false,
                "is_archived": false,
                "is_deleted": false,
                "created_at": "2024-01-15T10:00:00Z",
                "updated_at": "2024-01-15T10:00:00Z"
            }
        ],
        "sections": [],
        "labels": [],
        "filters": []
    }"#
}

pub fn mock_command_response_success() -> &'static str {
    r#"{
        "sync_token": "mock_sync_token_67890",
        "sync_status": {
            "cmd-uuid-1": "ok"
        },
        "temp_id_mapping": {
            "temp-uuid-1": "789"
        }
    }"#
}

pub fn mock_command_response_error() -> &'static str {
    r#"{
        "sync_token": "mock_sync_token_67890",
        "sync_status": {
            "cmd-uuid-1": {
                "error_tag": "INVALID_ARGUMENT_VALUE",
                "error_code": 20,
                "error": "Invalid argument value",
                "http_code": 400,
                "error_extra": {
                    "argument": "content",
                    "explanation": "content cannot be empty"
                }
            }
        },
        "temp_id_mapping": {}
    }"#
}
```

### 6.2 单元测试

```rust
// tests/unit/sync_client_tests.rs

#[cfg(test)]
mod sync_client_tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, header, body_form};

    #[tokio::test]
    async fn test_get_projects_sync() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/sync"))
            .and(header("Authorization", "Bearer test_token"))
            .and(body_form(partial_eq![
                ("sync_token", "*"),
                ("resource_types", "[\"projects\"]")
            ]))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                mock_sync_response_full()
            ))
            .mount(&mock_server)
            .await;

        let client = TodoistSyncClient::new("test_token".to_string());
        let projects = client.get_projects().await.unwrap();

        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].name, "Work");
    }

    #[tokio::test]
    async fn test_execute_commands_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/sync"))
            .and(header("Authorization", "Bearer test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                mock_command_response_success()
            ))
            .mount(&mock_server)
            .await;

        let client = TodoistSyncClient::new("test_token".to_string());
        let mut builder = CommandBuilder::new();
        builder.item_close("456");
        let result = client.execute_commands(&builder.build()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_command_error_handling() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/sync"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                mock_command_response_error()
            ))
            .mount(&mock_server)
            .await;

        let client = TodoistSyncClient::new("test_token".to_string());
        let mut builder = CommandBuilder::new();
        builder.item_add(ItemAddArgs {
            content: "".to_string(),  // 空内容会触发错误
            description: None,
            project_id: None,
            section_id: None,
            due_string: None,
            priority: None,
            labels: None,
        });
        let result = client.execute_commands(&builder.build()).await;

        assert!(result.is_err());
    }
}
```

### 6.3 集成测试

```rust
// tests/integration/sync_api_tests.rs

#[tokio::test]
#[ignore]
async fn test_full_sync_workflow() {
    // 此测试需要真实的 Todoist Token
    // 使用环境变量 TODORUST_TEST_TOKEN 运行

    let token = std::env::var("TODORUST_TEST_TOKEN")
        .expect("TODORUST_TEST_TOKEN not set");

    let mut client = TodoistSyncClient::new(token);

    // 1. 全量同步
    let response = client.full_sync().await.unwrap();
    assert!(response.projects.len() > 0);

    // 2. 创建分区
    let project_id = &response.projects[0].id;
    let section = client.create_section("Test Section", project_id).await.unwrap();

    // 3. 在分区中创建任务
    let task = client.create_task_in_section(
        "Task in section",
        &section.id,
        project_id
    ).await.unwrap();

    assert!(task.section_id.is_some());
    assert_eq!(task.section_id.unwrap(), section.id);

    // 4. 完成任务
    client.complete_task(&task.id).await.unwrap();

    // 5. 清理
    client.delete_section(&section.id).await.unwrap();

    // 6. 验证
    let updated_tasks = client.get_tasks(None).await.unwrap();
    let deleted_task = updated_tasks.iter().find(|t| t.id == task.id);
    assert!(deleted_task.is_none() || deleted_task.unwrap().is_completed);
}
```

### 6.4 测试覆盖清单

| 测试类型 | 覆盖场景 | 优先级 |
|---------|---------|-------|
| 单元测试 | Command 构建器 | P0 |
| 单元测试 | UUID 生成 | P0 |
| 单元测试 | 响应解析 | P0 |
| 单元测试 | 错误处理 | P1 |
| 单元测试 | 过滤逻辑 | P1 |
| 集成测试 | 全量同步 | P0 |
| 集成测试 | 增量同步 | P1 |
| 集成测试 | 批量操作 | P1 |
| 集成测试 | Section CRUD | P0 |
| 集成测试 | 任务 CRUD + Section | P0 |

---

## 7. 风险评估与缓解

### 7.1 技术风险

| 风险 | 影响 | 概率 | 缓解措施 |
|-----|------|-----|---------|
| Sync API 速率限制 | 高频操作被限制 | 中 | 实现请求节流，添加重试逻辑 |
| 迁移期间 API 不可用 | 服务中断 | 低 | 保留 REST API 降级能力 |
| 增量同步状态丢失 | 数据不一致 | 低 | sync_token 持久化到文件 |
| 临时 ID 冲突 | 命令执行失败 | 极低 | UUID v4 随机性足够 |

### 7.2 兼容性风险

| 风险 | 影响 | 缓解措施 |
|-----|------|---------|
| 第三方集成失效 | 维护 REST API Client | 提供 feature flag 选择 |
| 现有配置不兼容 | 提供迁移脚本 | 自动迁移配置文件 |

### 7.3 回滚计划

```rust
// 保留旧 Client 降级路径
#[cfg(feature = "allow-fallback")]
async fn fallback_to_rest(&self, operation: &str) -> Result<(), TodoError> {
    warn!("Falling back to REST API for {}", operation);
    // 调用原有的 REST API Client
}
```

---

## 8. 项目估算

### 8.1 时间估算

| 阶段 | 预计时间 | 工作量 |
|-----|---------|-------|
| 阶段 1: 基础架构 | 3-5 天 | 8-12 人时 |
| 阶段 2: 读操作迁移 | 2-3 天 | 6-10 人时 |
| 阶段 3: 写操作迁移 | 3-5 天 | 10-15 人时 |
| 阶段 4: Section 支持 | 2-3 天 | 6-10 人时 |
| 阶段 5: 清理优化 | 2-3 天 | 5-8 人时 |
| **总计** | **12-19 天** | **35-55 人时** |

### 8.2 依赖变更

```toml
# Cargo.toml 新增依赖

[dependencies]
uuid = { version = "1.6", features = ["v4"] }
serde_json = "1.0"
wiremock = "0.6"  # 测试用
```

---

## 9. 附录

### 9.1 Sync API 资源类型列表

| 资源类型 | 描述 | 支持的操作 |
|---------|------|---------|
| `projects` | 项目 | 读取、创建、更新、删除 |
| `items` | 任务 | 读取、创建、更新、删除、移动 |
| `sections` | 分区 | 读取、创建、更新、删除、排序 |
| `labels` | 标签 | 读取、创建、更新、删除 |
| `filters` | 过滤器 | 读取、创建、更新、删除 |
| `notes` | 备注 | 读取、创建、更新、删除 |
| `reminders` | 提醒 | 读取、创建、更新、删除 |

### 9.2 常用 Sync Commands 列表

| Command | 描述 | 参数 |
|---------|------|------|
| `project_add` | 创建项目 | name, color, favorite |
| `project_update` | 更新项目 | id, name, color, favorite |
| `project_delete` | 删除项目 | id |
| `item_add` | 创建任务 | content, description, project_id, section_id, due_string, priority, labels |
| `item_update` | 更新任务 | id, content, description, priority, labels |
| `item_delete` | 删除任务 | id |
| `item_close` | 完成任务 | id |
| `item_reopen` | 重新打开任务 | id |
| `item_move` | 移动任务 | id, project_id, section_id |
| `section_add` | 创建分区 | name, project_id |
| `section_update` | 更新分区 | id, name |
| `section_delete` | 删除分区 | id |
| `section_reorder` | 排序分区 | id, order |

### 9.3 颜色映射表

| REST API 颜色 | Sync API 颜色 |
|--------------|-------------|
| `blue` | `blue` |
| `green` | `green` |
| `red` | `red` |
| `orange` | `orange` |
| `yellow` | `yellow` |
| `purple` | `purple` |
| `pink` | `pink` |
| `cyan` | `cyan` |
| `gray` | `gray` |
| `berry_red` | `berry_red` |
| `blueberry_blue` | `blueberry_blue` |
| `lavender` | `lavender` |
| `tangerine` | `tangerine` |
| `kiwi_green` | `kiwi_green` |
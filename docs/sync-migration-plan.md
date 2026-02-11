# Todoist Sync API 迁移计划

> **创建日期**: 2026-02-11
> **版本**: v1.0

## 1. 当前 API 使用情况分析

### 1.1 当前使用的 REST API v1 端点

基于 `src/api.rs` 分析，当前 todorust 使用以下 API 端点：

| 功能 | HTTP 方法 | 端点 | 说明 |
|-----|----------|------|------|
| 获取项目 | GET | `/projects` | 返回 `ProjectsResponse` |
| 获取任务 | GET | `/tasks` | 获取所有任务 |
| 获取已完成任务 | GET | `/tasks/completed/by_completion_date` | 返回 `items` 数组 |
| 过滤任务 | GET | `/tasks/filter` | 使用 `query` 参数 |
| 获取过滤器 | POST | `/sync` (v1) | 唯一使用 sync 的地方 |
| 创建任务 | POST | `/tasks` | 传入 `CreateTaskRequest` |
| 删除任务 | DELETE | `/tasks/{id}` | |
| 完成任务 | POST | `/tasks/{id}/close` | |
| 重新打开任务 | POST | `/tasks/{id}/reopen` | |

### 1.2 当前代码结构

```
base_url: "https://api.todoist.com/api/v1"
├── /projects
├── /tasks
├── /tasks/completed/by_completion_date
├── /tasks/filter
├── /sync (仅用于 filters)
├── /tasks (POST)
├── /tasks/{id} (DELETE)
├── /tasks/{id}/close (POST)
└── /tasks/{id}/reopen (POST)
```

### 1.3 当前未使用的功能

- **分区 (Sections)**: 完全未使用
- **标签 (Labels)**: 仅在 Task 模型中存在，API 层未完整实现
- **备注 (Notes)**: 未实现
- **提醒 (Reminders)**: 未实现
- **增量同步**: 未实现

---

## 2. Sync 端点能力对比

### 2.1 核心优势

| 特性 | REST API v1 | Sync API v9 |
|------|-------------|-------------|
| **增量同步** | ❌ 不支持 | ✅ 支持 `sync_token` |
| **批量操作** | ❌ 每次请求一个资源 | ✅ 单次最多 100+ commands |
| **临时资源 ID** | ❌ 需预先存在 | ✅ 支持 `temp_<uuid>` 引用 |
| **分区操作** | ❌ 有限 | ✅ 完整 CRUD + 排序 |
| **原子性** | ❌ 每个请求独立 | ✅ 单个请求多操作 |
| **资源统一获取** | ❌ 多个端点 | ✅ 单个 `/sync` 端点 |

### 2.2 分区 (Sections) 支持

这是迁移的关键驱动因素：

| 操作 | REST API v1 | Sync API v9 |
|-----|-------------|-------------|
| 获取分区 | ❌ 不支持 | ✅ `resource_types: ["sections"]` |
| 创建分区 | ❌ 不支持 | ✅ `section_add` command |
| 更新分区 | ❌ 不支持 | ✅ `section_update` command |
| 删除分区 | ❌ 不支持 | ✅ `section_delete` command |
| 排序分区 | ❌ 不支持 | ✅ `section_reorder` command |

### 2.3 迁移后的新能力

1. **按分区显示任务**: 任务可按 `section_id` 分组显示
2. **更好的数据一致性**: 一次同步获取所有相关数据
3. **更少的 API 调用**: 减少网络往返
4. **支持任务移动**: `item_move` 可跨分区移动任务

---

## 3. 需要修改的功能列表

### 3.1 必需修改 (P0)

| 序号 | 功能 | 修改类型 | 优先级 |
|-----|------|---------|-------|
| 1 | Client 结构体 | 新增 `sync_token` 字段 | P0 |
| 2 | `get_projects()` | 重构为使用 Sync API | P0 |
| 3 | `get_tasks()` | 重构为使用 Sync API | P0 |
| 4 | `create_task()` | 重构为使用 commands | P0 |
| 5 | `complete_task()` | 重构为使用 commands | P0 |
| 6 | `reopen_task()` | 重构为使用 commands | P0 |
| 7 | `delete_task()` | 重构为使用 commands | P0 |

### 3.2 新增功能 (P1)

| 序号 | 功能 | 描述 |
|-----|------|------|
| 1 | `get_sections()` | 获取项目分区 |
| 2 | `create_section()` | 创建分区 |
| 3 | `update_section()` | 更新分区 |
| 4 | `delete_section()` | 删除分区 |
| 5 | `move_task()` | 移动任务到分区 |
| 6 | `reorder_sections()` | 重排序分区 |

### 3.3 增强功能 (P2)

| 序号 | 功能 | 描述 |
|-----|------|------|
| 1 | 增量同步实现 | 使用 sync_token 减少数据量 |
| 2 | 批量任务操作 | 一次请求创建/更新多个任务 |
| 3 | 标签管理 | 完整的 CRUD 操作 |
| 4 | 备注管理 | 完整的 CRUD 操作 |

---

## 4. 分阶段实施计划

### 阶段 1: 基础架构重构 (1-2 周)

**目标**: 建立 Sync API 基础架构，保持功能等价

#### 1.1 新增 Sync Client 结构

```rust
pub struct TodoistSyncClient {
    token: String,
    sync_url: String,
    sync_token: Option<String>,
    http: HttpClient,
}
```

#### 1.2 实现新的同步方法

```rust
impl TodoistSyncClient {
    pub async fn sync(&self, resource_types: &[&str]) -> Result<SyncResponse, TodoError> { ... }
    pub async fn execute_commands(&self, commands: &[Command]) -> Result<CommandResponse, TodoError> { ... }
}
```

#### 1.3 迁移现有方法

- `get_projects()` → 使用 `sync(["projects"])`
- `get_tasks()` → 使用 `sync(["items"])`
- `get_filters()` → 使用 `sync(["filters"])`

**里程碑**: 所有现有功能通过 Sync API 工作

---

### 阶段 2: 分区功能实现 (1 周)

**目标**: 添加分区支持

#### 2.1 新增 Section 模型和响应

```rust
#[derive(Debug, Deserialize)]
pub struct Section {
    pub id: String,
    pub name: String,
    pub project_id: String,
    pub order: i32,
    pub archived: bool,
    pub added_at: String,
}
```

#### 2.2 实现分区操作

```rust
impl TodoistSyncClient {
    pub async fn get_sections(&self, project_id: Option<&str>) -> Result<Vec<Section>, TodoError> { ... }
    pub async fn create_section(&self, name: &str, project_id: &str) -> Result<Section, TodoError> { ... }
    pub async fn delete_section(&self, section_id: &str) -> Result<(), TodoError> { ... }
}
```

#### 2.3 增强 TaskOutput

```rust
#[derive(Debug, Serialize)]
pub struct TaskOutput {
    // ... 现有字段
    pub section_id: Option<String>,  // 新增
    pub section_name: Option<String>, // 新增
}
```

**里程碑**: 用户可以查看和操作分区

---

### 阶段 3: 高级功能 (1 周)

**目标**: 增量同步和批量操作

#### 3.1 实现增量同步

```rust
impl TodoistSyncClient {
    pub async fn incremental_sync(&mut self) -> Result<&mut Self, TodoError> {
        let token = self.sync_token.as_deref().unwrap_or("*");
        let response = self.sync_with_token(token).await?;
        self.sync_token = Some(response.sync_token);
        // 更新本地缓存
        Ok(self)
    }
}
```

#### 3.2 批量任务操作

```rust
impl TodoistSyncClient {
    pub async fn create_tasks_batch(&self, tasks: &[CreateTaskRequest]) -> Result<Vec<TaskOutput>, TodoError> { ... }
}
```

**里程碑**: 支持增量更新和批量操作

---

### 阶段 4: 清理和优化 (3-5 天)

**目标**: 移除旧代码，优化性能

- [ ] 移除 REST API v1 Client 或保留兼容性
- [ ] 添加单元测试覆盖新功能
- [ ] 更新文档
- [ ] 性能优化 (连接池、缓存)

---

## 5. 潜在风险和注意事项

### 5.1 技术风险

| 风险 | 影响 | 缓解措施 |
|-----|------|---------|
| Sync API 速率限制 | 可能影响高频操作 | 实现请求节流 |
| 迁移期间 API 不可用 | 服务中断 | 保持双 Client 切换能力 |
| 增量同步状态丢失 | 数据不一致 | 持久化 sync_token |

### 5.2 兼容性风险

| 风险 | 影响 | 缓解措施 |
|-----|------|---------|
| 旧版 token 不兼容 | 无法迁移 | 检查 token 格式 |
| 第三方集成 | 可能失效 | 维护 REST API Client |
| 现有用户配置 | 需要迁移 | 提供迁移脚本 |

### 5.3 数据一致性

- **临时 ID 引用**: 确保在同一 command 批处理中使用
- **资源不存在**: 正确处理 404 错误
- **并发修改**: 考虑使用乐观锁

### 5.4 回滚计划

如果迁移出现问题：

1. **保留双 Client**: 在配置中添加 `api_type: "sync" | "rest"` 选择
2. **Feature Flag**: 通过环境变量控制使用哪个 API
3. **快速回滚**: 可以立即切换回 REST API (功能降级)

---

## 6. 实施检查清单

### 代码变更

- [ ] 创建 `TodoistSyncClient` 结构体
- [ ] 实现 `sync()` 方法
- [ ] 实现 `execute_commands()` 方法
- [ ] 迁移 `get_projects()`
- [ ] 迁移 `get_tasks()`
- [ ] 迁移 `create_task()`, `complete_task()`, `reopen_task()`, `delete_task()`
- [ ] 新增 `Section` 模型
- [ ] 实现 `get_sections()` 等分区方法
- [ ] 更新 `TaskOutput` 添加 `section_id` 和 `section_name`
- [ ] 移除或保留旧 REST API Client

### 测试

- [ ] 单元测试覆盖 Sync Client
- [ ] 集成测试 (需要真实 Todoist token)
- [ ] 回滚测试

### 文档

- [ ] 更新 README.md
- [ ] 更新 USAGE.md
- [ ] API 变更日志

---

## 7. 成功标准

1. ✅ 所有现有功能通过 Sync API 正常工作
2. ✅ 新增分区功能
3. ✅ API 调用次数减少 ≥ 50%
4. ✅ 增量同步功能可用
5. ✅ 测试覆盖率 ≥ 80%

---

## 参考资料

- 官方文档: https://developer.todoist.com/sync/v9/
- 本地文档: `docs/todoist-sync-api.md`
- REST API 对比: https://developer.todoist.com/rest/v2/

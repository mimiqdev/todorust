# CI 测试覆盖开发计划

## 1. 项目现状分析

### 1.1 现有测试结构

| 模块 | 文件 | 测试状态 |
|------|------|----------|
| **Commands** | `src/sync/commands.rs` | ✅ 有 13 个单元测试 |
| **Config** | `src/config.rs` | ✅ 有 1 个配置解析测试 |
| **Client** | `src/sync/client.rs` | ❌ 空测试模块 |
| **Models** | `src/sync/models.rs` | ❌ 无测试 |
| **Error** | `src/error.rs` | ❌ 无测试 |
| **Integration** | `tests/integration_test.rs` | ⚠️ 只有一个被忽略的测试 |

### 1.2 当前测试覆盖率评估

- **Commands 模块**: ~85% (builder、args 结构测试完整)
- **Config 模块**: ~50% (只测试了解析，未测试文件操作)
- **Client 模块**: 0% (所有方法未测试)
- **Models 模块**: ~30% (只有类型转换测试)
- **Error 模块**: 0% (错误类型未测试)

### 1.3 缺失的测试

#### 单元测试缺失:
1. **client.rs**:
   - `sync()` 方法 (HTTP 请求/响应处理)
   - `execute_commands()` 方法
   - `check_sync_status()` 错误处理
   - 所有 CRUD 方法 (get_projects, add_task, etc.)
   - sync_token 管理

2. **models.rs**:
   - SyncDue 反序列化
   - SyncSection/SyncLabel/SyncFilter 反序列化
   - API 响应 JSON 解析边界情况

3. **error.rs**:
   - 所有错误变体的 Display 实现
   - From trait 实现

4. **config.rs**:
   - `load_config()` 文件不存在场景
   - `init_config()` 目录创建场景

#### 集成测试缺失:
1. 完整的 E2E 工作流测试
2. API 错误响应处理测试
3. 并发请求测试

---

## 2. CI 配置计划

### 2.1 GitHub Actions 工作流

**文件**: `.github/workflows/ci.yml`

```yaml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  # 1. 格式化检查
  fmt:
    name: Format
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - name: Check formatting
        run: cargo fmt --check --all

  # 2. Lint 检查
  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - name: Clippy check
        run: cargo clippy --all-features --all-targets -- -D warnings

  # 3. 单元测试 (无网络)
  unit-test:
    name: Unit Tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Run unit tests
        run: cargo test --lib --bins

  # 4. 集成测试 (需要 API Token)
  integration-test:
    name: Integration Tests
    runs-on: ubuntu-latest
    # 仅在 main 分支或手动触发时运行
    if: github.ref == 'refs/heads/main' || github.event_name == 'workflow_dispatch'
    needs: [fmt, clippy, unit-test]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Run integration tests
        env:
          TODOIST_API_TOKEN: ${{ secrets.TODOIST_API_TOKEN }}
        run: cargo test --test integration_test -- --ignored

  # 5. 代码覆盖率
  coverage:
    name: Coverage
    runs-on: ubuntu-latest
    needs: [fmt, clippy, unit-test]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin
      - name: Generate coverage report
        run: |
          cargo tarpaulin \
            --out Xml \
            --lib \
            --bins \
            --tests \
            --exclude-files target \
            --fail-under 70
      - name: Upload to Codecov
        uses: codecov/codecov-action@v4
        with:
          files: ./target/tarpaulin/report.xml
          token: ${{ secrets.CODECOV_TOKEN }}
          fail_ci_if_error: true
```

### 2.2 CI 流程图

```
Push / PR
    │
    ▼
┌─────────────────────────────────────┐
│  1. Format Check (cargo fmt --check) │
└─────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────┐
│  2. Clippy Check                    │
└─────────────────────────────────────┘
    │
    ▼
┌─────────────────────────────────────┐
│  3. Unit Tests (cargo test --lib)   │
└─────────────────────────────────────┘
    │
    ├──────────────────────────────────┐
    │                                  │
    ▼                                  ▼
┌──────────────────┐          ┌──────────────────┐
│ Integration Test │          │   Coverage       │
│ (main only)      │          │   (tarpaulin)    │
└──────────────────┘          └──────────────────┘
    │                                  │
    │                                  ▼
    │                          ┌──────────────────┐
    │                          │ Upload Codecov   │
    │                          └──────────────────┘
    │
    ▼
┌─────────────────────────────────────┐
│  Merge (if all checks pass)          │
└─────────────────────────────────────┘
```

---

## 3. 测试覆盖计划

### 3.1 单元测试补充

#### 3.1.1 `src/sync/client.rs` 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::commands::{CommandBuilder, ItemAddArgs};
    use crate::sync::models::{SyncReadResponse, SyncWriteResponse};
    use mockito::mock;

    #[tokio::test]
    async fn test_sync_success() {
        // Mock HTTP response
        let _m = mock("POST", "/api/v1/sync")
            .with_status(200)
            .with_body(r#"{"sync_token": "abc", "projects": [], "items": []}"#)
            .create();

        let client = TodoistSyncClient::new("test_token".to_string());
        let result = client.sync(&["projects"]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sync_http_error() {
        let _m = mock("POST", "/api/v1/sync")
            .with_status(500)
            .create();

        let client = TodoistSyncClient::new("test_token".to_string());
        let result = client.sync(&["projects"]).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_sync_token_management() {
        let client = TodoistSyncClient::new("test_token".to_string());
        assert!(client.get_sync_token().is_none());

        client.set_sync_token("test_token_123".to_string());
        assert_eq!(client.get_sync_token(), Some("test_token_123".to_string()));
    }

    #[tokio::test]
    async fn test_execute_commands() {
        let _m = mock("POST", "/api/v1/sync")
            .with_status(200)
            .with_body(r#"{"sync_token": "abc", "sync_status": {}, "temp_id_mapping": {}}"#)
            .create();

        let client = TodoistSyncClient::new("test_token".to_string());
        let commands = CommandBuilder::new()
            .item_add(ItemAddArgs::new("Test".to_string()))
            .build();
        let result = client.execute_commands(&commands).await;
        assert!(result.is_ok());
    }
}
```

#### 3.1.2 `src/sync/models.rs` 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_sync_project_deserialize() {
        let json = r#"{
            "id": "123",
            "name": "Test Project",
            "color": "blue",
            "shared": false,
            "favorite": true,
            "sort_order": 1,
            "is_archived": false,
            "is_deleted": false,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }"#;
        let project: SyncProject = serde_json::from_str(json).unwrap();
        assert_eq!(project.id, "123");
        assert_eq!(project.name, "Test Project");
    }

    #[test]
    fn test_sync_task_with_due() {
        let json = r#"{
            "id": "456",
            "content": "Test Task",
            "priority": 4,
            "due": {
                "date": "2024-01-15",
                "is_recurring": false
            },
            "order": 1,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }"#;
        let task: SyncTask = serde_json::from_str(json).unwrap();
        assert!(task.due.is_some());
        assert_eq!(task.due.unwrap().date, Some("2024-01-15".to_string()));
    }

    #[test]
    fn test_sync_section_deserialize() {
        let json = r#"{
            "id": "789",
            "project_id": "123",
            "name": "Section 1",
            "order": 1,
            "created_at": "2024-01-01T00:00:00Z"
        }"#;
        let section: SyncSection = serde_json::from_str(json).unwrap();
        assert_eq!(section.name, "Section 1");
    }

    #[test]
    fn test_sync_label_deserialize() {
        let json = r#"{
            "id": "label1",
            "name": "urgent",
            "color": "red"
        }"#;
        let label: SyncLabel = serde_json::from_str(json).unwrap();
        assert_eq!(label.name, "urgent");
    }

    #[test]
    fn test_sync_filter_deserialize() {
        let json = r#"{
            "id": "filter1",
            "name": "Today",
            "query": "today"
        }"#;
        let filter: SyncFilter = serde_json::from_str(json).unwrap();
        assert_eq!(filter.query, "today");
    }

    #[test]
    fn test_sync_project_to_project_conversion() {
        let sync_project = SyncProject {
            id: "123".to_string(),
            name: "Test".to_string(),
            color: "blue".to_string(),
            shared: true,
            favorite: false,
            sort_order: 1,
            is_archived: false,
            is_deleted: false,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };
        let project: Project = sync_project.into();
        assert_eq!(project.id, "123");
        assert!(project.is_shared);
    }

    #[test]
    fn test_empty_response() {
        let json = r#"{"sync_token": "abc"}"#;
        let response: SyncReadResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.sync_token, "abc");
        assert!(response.projects.is_empty());
        assert!(response.items.is_empty());
    }
}
```

#### 3.1.3 `src/error.rs` 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_config_error_display() {
        let error = TodoError::Config("test error".to_string());
        assert_eq!(format!("{}", error), "Config error: test error");
    }

    #[test]
    fn test_api_error_display() {
        let error = TodoError::Api("API failed".to_string());
        assert_eq!(format!("{}", error), "API error: API failed");
    }

    #[test]
    fn test_http_error_display() {
        let error = TodoError::Http(404);
        assert_eq!(format!("{}", error), "HTTP error 404");
    }

    #[test]
    fn test_invalid_input_error_display() {
        let error = TodoError::InvalidInput("Invalid value".to_string());
        assert_eq!(format!("{}", error), "Invalid input: Invalid value");
    }

    #[test]
    fn test_serialize_error_display() {
        let error = TodoError::Serialize("JSON error".to_string());
        assert_eq!(format!("{}", error), "Serialize error: JSON error");
    }

    #[test]
    fn test_config_not_found_display() {
        let error = TodoError::ConfigNotFound;
        let display = format!("{}", error);
        assert!(display.contains("Configuration not found"));
    }

    #[test]
    fn test_from_reqwest_error() {
        let io_error = io::Error::new(io::ErrorKind::other, "connection refused");
        let reqwest_error = reqwest::Error::from(io_error);
        let todo_error = TodoError::from(reqwest_error);
        assert!(matches!(todo_error, TodoError::Request(_)));
    }

    #[test]
    fn test_from_serde_json_error() {
        let json_error = serde_json::Error::custom("parse error");
        let todo_error: TodoError = json_error.into();
        assert!(matches!(todo_error, TodoError::Api(_)));
    }
}
```

#### 3.1.4 `src/config.rs` 测试补充

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_init_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("todorust");
        
        // Mock dirs::config_dir
        // Note: This test would need to use a mock or test helper
        
        let config = Config {
            api_token: "test_token_123".to_string()
        };
        
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("test_token_123"));
    }

    #[test]
    fn test_config_with_special_chars() {
        let toml_str = r#"
            api_token = "token_with_special_chars!@#$%"
        "#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.api_token, "token_with_special_chars!@#$%");
    }
}
```

### 3.2 集成测试计划

#### 3.2.1 `tests/integration_test.rs` 完善

```rust
use std::env;
use todorust::sync::TodoistSyncClient;
use todorust::config::load_config;

fn get_test_token() -> String {
    env::var("TODOIST_API_TOKEN")
        .ok()
        .or_else(|| {
            load_config().ok().map(|c| c.api_token)
        })
        .expect("TODOIST_API_TOKEN must be set or config must be loaded")
}

#[tokio::test]
#[ignore]
async fn test_full_sync_workflow() {
    let token = get_test_token();
    let client = TodoistSyncClient::new(token);

    // Full sync
    let response = client.sync(&["projects", "items", "sections"]).await;
    assert!(response.is_ok(), "Full sync should succeed");
    let data = response.unwrap();
    
    // Verify response structure
    assert!(data.sync_token.len() > 0);
    println!("Sync token: {}", data.sync_token);
    println!("Projects count: {}", data.projects.len());
    println!("Items count: {}", data.items.len());
}

#[tokio::test]
#[ignore]
async fn test_project_crud() {
    let token = get_test_token();
    let client = TodoistSyncClient::new(token);

    // Create
    let project_id = client.add_project("Test Project", None, Some(true)).await
        .expect("Should create project");
    assert!(project_id.len() > 0);
    println!("Created project: {}", project_id);

    // Read
    let projects = client.get_projects().await
        .expect("Should get projects");
    assert!(projects.iter().any(|p| p.id == project_id));
    println!("Found project in list");

    // Update would go here (project_update command)

    // Delete would go here (project_delete command)
}

#[tokio::test]
#[ignore]
async fn test_task_crud() {
    let token = get_test_token();
    let client = TodoistSyncClient::new(token);

    // Create task
    let task_id = client.add_task(
        "Integration Test Task",
        Some("Test description"),
        None,
        None,
        None,
        Some(4),
        Some(vec!["test"])
    ).await
        .expect("Should create task");
    assert!(task_id.len() > 0);
    println!("Created task: {}", task_id);

    // Read
    let tasks = client.get_tasks().await
        .expect("Should get tasks");
    assert!(tasks.iter().any(|t| t.id == task_id));

    // Update
    client.update_task(
        &task_id,
        Some("Updated Task"),
        Some("Updated description"),
        Some(3),
        None,
        None
    ).await
        .expect("Should update task");

    // Complete
    client.complete_task(&task_id).await
        .expect("Should complete task");

    // Delete
    client.delete_task(&task_id).await
        .expect("Should delete task");
}

#[tokio::test]
#[ignore]
async fn test_section_crud() {
    let token = get_test_token();
    let client = TodoistSyncClient::new(token);

    // First create a project
    let project_id = client.add_project("Section Test Project", None, None).await
        .expect("Should create project");

    // Create section
    let section_id = client.add_section("Test Section", &project_id).await
        .expect("Should create section");
    assert!(section_id.len() > 0);
    println!("Created section: {}", section_id);

    // Update
    client.update_section(&section_id, "Updated Section").await
        .expect("Should update section");

    // Cleanup
    client.delete_section(&section_id).await
        .expect("Should delete section");
    client.delete_task(&project_id).await
        .expect("Should delete project");
}

#[tokio::test]
#[ignore]
async fn test_label_crud() {
    let token = get_test_token();
    let client = TodoistSyncClient::new(token);

    // Create label
    let label_id = client.add_label("integration_test_label", Some("green")).await
        .expect("Should create label");
    assert!(label_id.len() > 0);
    println!("Created label: {}", label_id);

    // Update
    client.update_label(&label_id, Some("updated_label"), Some("red")).await
        .expect("Should update label");

    // Delete
    client.delete_label(&label_id).await
        .expect("Should delete label");
}

#[tokio::test]
#[ignore]
async fn test_filter_crud() {
    let token = get_test_token();
    let client = TodoistSyncClient::new(token);

    // Create filter
    let filter_id = client.add_filter(
        "Integration Test Filter",
        "today & !completed",
        Some("blue"
    ).await
        .expect("Should create filter");
    assert!(filter_id.len() > 0);
    println!("Created filter: {}", filter_id);

    // Update
    client.update_filter(
        &filter_id,
        Some("Updated Filter"),
        Some("overdue"),
        Some("red")
    ).await
        .expect("Should update filter");

    // Delete
    client.delete_filter(&filter_id).await
        .expect("Should delete filter");
}

#[tokio::test]
#[ignore]
async test_batch_operations() {
    let token = get_test_token();
    let client = TodoistSyncClient::new(token);

    // Batch create tasks
    use todorust::sync::commands::{CommandBuilder, ItemAddArgs};

    let commands = CommandBuilder::new()
        .item_add(ItemAddArgs::new("Batch Task 1".to_string()))
        .item_add(ItemAddArgs::new("Batch Task 2".to_string()))
        .item_add(ItemAddArgs::new("Batch Task 3".to_string()))
        .build();

    let response = client.execute_commands(&commands).await
        .expect("Should execute batch commands");
    
    assert_eq!(response.temp_id_mapping.len(), 3);
    println!("Batch created {} tasks", response.temp_id_mapping.len());
}
```

### 3.3 Mock 测试 (可选)

对于不需要真实 API 的测试，可以使用 `mockito` crate：

```rust
// tests/mock_test.rs
use mockito::{mock, Matcher};
use todorust::sync::TodoistSyncClient;

#[tokio::test]
async fn test_sync_with_mock_server() {
    let _m = mock("POST", "/api/v1/sync")
        .with_status(200)
        .with_header("content-type", "application/json")
        .match_body(Matcher::Regex(r#"sync_token=\*"#.to_string()))
        .match_body(Matcher::Regex(r#"resource_types"# .to_string()))
        .with_body(r#"{
            "sync_token": "abc123",
            "full_sync": true,
            "projects": [{"id": "1", "name": "Test", "color": "blue", "shared": false, "favorite": false, "sort_order": 1, "is_archived": false, "is_deleted": false, "created_at": "2024-01-01T00:00:00Z", "updated_at": "2024-01-01T00:00:00Z"}],
            "items": [],
            "sections": [],
            "labels": [],
            "filters": []
        }"#)
        .create();

    let client = TodoistSyncClient::new("test_token".to_string());
    let result = client.sync(&["projects"]).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.projects.len(), 1);
    assert_eq!(response.projects[0].name, "Test");
}
```

---

## 4. 需要的 Secrets

### 4.1 GitHub Secrets 列表

| Secret 名称 | 必需 | 用途 | 获取方式 |
|------------|------|------|---------|
| `TODOIST_API_TOKEN` | ✅ 是 | 集成测试的 API 调用 | Todoist Settings → Integrations → API token |
| `CODECOV_TOKEN` | ✅ 是 (已配置) | 上传覆盖率报告 | Codecov 仓库设置页面 |

### 4.2 设置步骤

#### 获取 TODOIST_API_TOKEN:
1. 登录 Todoist
2. 进入 Settings → Integrations
3. 复制 API token

#### 添加到 GitHub:
```bash
# 使用 GitHub CLI
gh secret set TODOIST_API_TOKEN
# 输入你的 token 值

# CODECOV_TOKEN 应该已经配置好
gh secret list | grep CODECOV
```

---

## 5. 执行步骤

### 5.1 第一阶段：完善 CI 配置 (优先级: 高)

1. **创建 GitHub Actions 工作流**
   ```bash
   mkdir -p .github/workflows
   touch .github/workflows/ci.yml
   ```

2. **添加 coverage job 配置** (使用 cargo-tarpaulin)

3. **测试工作流是否正常**

### 5.2 第二阶段：补充单元测试 (优先级: 高)

1. **完善 `client.rs` 测试** (添加 HTTP mock 测试)
   - 安装 `mockito` 或使用 wiremock
   - 测试所有 CRUD 方法的错误处理

2. **完善 `models.rs` 测试**
   - 添加 JSON 解析边界测试
   - 测试空响应和缺失字段

3. **添加 `error.rs` 测试**
   - 所有错误变体的 Display 测试
   - From trait 测试

4. **补充 `config.rs` 测试**
   - 测试特殊字符
   - 测试 TOML 解析边界

### 5.3 第三阶段：完善集成测试 (优先级: 中)

1. **完善 `tests/integration_test.rs`**
   - 添加所有资源类型的 CRUD 测试
   - 添加批量操作测试
   - 添加错误场景测试

2. **添加 Mock API 测试** (可选)
   - 创建 mock_server 测试
   - 不需要真实 token 的快速测试

### 5.4 第四阶段：优化覆盖率 (优先级: 中)

1. **运行覆盖率检查**
   ```bash
   cargo tarpaulin --out Html
   # 查看 target/tarpaulin/index.html
   ```

2. **识别低覆盖率区域**
   - 目标: 整体覆盖率 > 70%
   - 关键模块目标: > 80%

3. **针对性补充测试**

---

## 6. 预期成果

### 6.1 测试覆盖率目标

| 模块 | 当前覆盖率 | 目标覆盖率 |
|------|-----------|-----------|
| commands.rs | ~85% | **95%** |
| client.rs | 0% | **90%** |
| models.rs | ~30% | **90%** |
| error.rs | 0% | **100%** |
| config.rs | ~50% | **90%** |
| formatter.rs | ~60% | **90%** |
| **总体** | **~40%** | **95%** |

### 6.2 达到 95% 的策略

95% 是非常高的标准，需要以下策略：

1. **Mock 所有 HTTP 调用** (client.rs 90%+)
   - 使用 `mockito` 或 `wiremock`
   - 测试所有成功/失败路径
   - 测试不同 HTTP 状态码

2. **100% 错误处理** (error.rs)
   - 所有错误变体的 Display
   - 所有 From trait 实现

3. **完整边界测试** (models.rs 90%+)
   - JSON 解析边界
   - 空值和可选字段

4. **批量命令测试** (commands.rs 95%+)
   - 添加所有缺失命令测试
   - 测试空命令和批量命令

### 6.3 CI Pipeline 预期

- ✅ Format check 每次 PR 都会运行
- ✅ Clippy check 每次 PR 都会运行
- ✅ Unit tests 每次 PR 都会运行 (快速)
- ✅ Integration tests 仅 main 分支运行 (需要 token)
- ✅ Coverage 每次 PR 运行，上传 Codecov
- ✅ Codecov 检查确保覆盖率不低于阈值

### 6.3 开发体验改进

- 快速反馈 (lint/format/unit tests < 2分钟)
- 集成测试自动运行在 main 分支
- 覆盖率报告自动生成和追踪
- 问题早发现 (CI 检查)

---

## 7. 附录

### 7.1 推荐工具

| 工具 | 用途 | 安装 |
|------|------|------|
| cargo-tarpaulin | 代码覆盖率 | `cargo install cargo-tarpaulin` |
| cargo-nextest | 更快的测试运行 | `cargo install cargo-nextest` |
| grcov | 替代覆盖率工具 | `cargo install grcov` |

### 7.2 配置示例

#### codecov.yml

```yaml
codecov:
  require_ci_to_pass: true
  token: ${{ secrets.CODECOV_TOKEN }}

coverage:
  status:
    project:
      default:
        target: 95%
        threshold: 5%
    patch:
      default:
        target: 95%
```

#### .cargo/config.toml (可选)

```toml
[profile.test]
incremental = false  # CI 环境优化

[tool.cargo-tarpaulin]
out = "Html"
fail-under = 95
exclude-files = ["tests/*", "target/*"]
```

### 7.3 相关链接

- [GitHub Actions 文档](https://docs.github.com/en/actions)
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)
- [Codecov GitHub Action](https://github.com/codecov/codecov-action)
- [Todoist Sync API](https://developer.todoist.com/sync/v1/)

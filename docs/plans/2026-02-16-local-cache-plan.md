# Local Cache Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 实现本地缓存功能，利用 Todoist Sync API 的 sync_token 做增量同步，减少 API 调用

**Architecture:** 在 src/sync/ 下新增 cache.rs 模块，管理 cache.json 的读写；在 TodoistSyncClient 中集成缓存逻辑；CLI 新增 sync/cache 命令

**Tech Stack:** Rust, serde_json, std::fs

---

## Task 1: 创建 Cache 数据结构和基础模块

**Files:**
- Create: `src/sync/cache.rs`
- Modify: `src/sync/mod.rs`
- Test: `tests/unit/cache_test.rs`

**Step 1: 创建 cache.rs 基础结构**

```rust
// src/sync/cache.rs
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct Cache {
    pub sync_token: String,
    pub cached_at: i64,
    pub data: CacheData,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CacheData {
    pub projects: Vec<super::models::SyncProject>,
    pub items: Vec<super::models::SyncTask>,
    pub sections: Vec<super::models::SyncSection>,
    pub labels: Vec<super::models::SyncLabel>,
    pub filters: Vec<super::models::SyncFilter>,
}

pub struct CacheManager {
    cache_path: PathBuf,
}

impl CacheManager {
    pub fn new() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("todorust");
        Self {
            cache_path: config_dir.join("cache.json"),
        }
    }

    pub fn load(&self) -> Result<Option<Cache>, crate::error::TodoError> {
        if !self.cache_path.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(&self.cache_path)?;
        let cache: Cache = serde_json::from_str(&content).map_err(|e| {
            crate::error::TodoError::InvalidInput(format!("Failed to parse cache: {}", e))
        })?;
        Ok(Some(cache))
    }

    pub fn save(&self, cache: &Cache) -> Result<(), crate::error::TodoError> {
        if let Some(parent) = self.cache_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(cache).map_err(|e| {
            crate::error::TodoError::InvalidInput(format!("Failed to serialize cache: {}", e))
        })?;
        std::fs::write(&self.cache_path, content)?;
        Ok(())
    }

    pub fn is_expired(&self, cache: &Cache, threshold_secs: u64) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        (now - cache.cached_at) > threshold_secs as i64
    }
}
```

**Step 2: 更新 src/sync/mod.rs 导出 cache 模块**

```rust
pub mod cache;
pub mod client;
pub mod commands;
pub mod models;
```

**Step 3: 添加 dirs 依赖到 Cargo.toml**

```toml
[dependencies]
dirs = "5"
```

**Step 4: 运行 cargo build 验证编译**

Run: `cd .worktrees/local-cache && cargo build`
Expected: COMPILATION ERROR (dirs not found) - add it

**Step 5: 添加 dirs 后重新编译**

Run: `cd .worktrees/local-cache && cargo build`
Expected: SUCCESS

**Step 6: 提交**

```bash
cd .worktrees/local-cache
git add src/sync/cache.rs src/sync/mod.rs Cargo.toml
git commit -m "feat: add cache module with CacheManager"
```

---

## Task 2: 集成缓存到 TodoistSyncClient

**Files:**
- Modify: `src/sync/client.rs`
- Test: `tests/unit/client_cache_test.rs`

**Step 1: 修改 TodoistSyncClient 结构体，添加缓存支持**

在 `src/sync/client.rs` 中添加:

```rust
use super::cache::{Cache, CacheData, CacheManager};

pub struct TodoistSyncClient {
    token: String,
    sync_url: String,
    sync_token: RefCell<Option<String>>,
    http: HttpClient,
    cache_manager: CacheManager,
    cache: RefCell<Option<Cache>>,
}
```

**Step 2: 修改 new() 初始化 cache_manager**

```rust
impl TodoistSyncClient {
    pub fn new(token: String) -> Self {
        // ... existing code ...
        Self {
            // ... existing fields ...
            cache_manager: CacheManager::new(),
            cache: RefCell::new(None),
        }
    }
}
```

**Step 3: 添加缓存同步方法**

```rust
impl TodoistSyncClient {
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

    /// 检查缓存是否过期 (默认 5 分钟)
    pub fn is_cache_expired(&self) -> bool {
        if let Some(ref cache) = *self.cache.borrow() {
            self.cache_manager.is_expired(cache, 300)
        } else {
            true
        }
    }

    /// 获取缓存数据
    pub fn get_cached_data(&self) -> Option<CacheData> {
        self.cache.borrow().as_ref().map(|c| c.data.clone())
    }
}
```

**Step 4: 修改 sync() 方法，在成功后保存缓存**

在 `sync` 方法末尾添加:

```rust
// 保存到缓存
let data = CacheData {
    projects: response.projects.clone(),
    items: response.items.clone(),
    sections: response.sections.clone(),
    labels: response.labels.clone(),
    filters: response.filters.clone(),
};
self.save_cache(&response.sync_token, data)?;
```

**Step 5: 运行 cargo build**

Run: `cd .worktrees/local-cache && cargo build`
Expected: SUCCESS

**Step 6: 提交**

```bash
cd .worktrees/local-cache
git add src/sync/client.rs
git commit -m "feat: integrate cache into TodoistSyncClient"
```

---

## Task 3: 实现混合同步逻辑

**Files:**
- Modify: `src/sync/client.rs`
- Modify: `src/cli/handlers.rs`

**Step 1: 在 TodoistSyncClient 添加混合同步方法**

```rust
impl TodoistSyncClient {
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
}
```

**Step 2: 修改 handlers.rs 中的 get_tasks 等方法使用缓存**

```rust
// 在 get_tasks 函数中
let response = client.sync_with_cache(&["items"]).await?;
let tasks = response.items;
```

**Step 3: 运行 cargo build**

Run: `cd .worktrees/local-cache && cargo build`
Expected: SUCCESS

**Step 4: 提交**

```bash
cd .worktrees/local-cache
git add src/sync/client.rs src/cli/handlers.rs
git commit -m "feat: implement hybrid sync with cache"
```

---

## Task 4: 添加 CLI sync 命令

**Files:**
- Modify: `src/cli/mod.rs`
- Modify: `src/main.rs`

**Step 1: 在 cli/mod.rs 添加 SyncCommands 枚举**

```rust
#[derive(Parser)]
pub enum SyncCommands {
    /// 同步数据 (默认增量，--force 全量)
    Sync {
        #[arg(long)]
        force: bool,
    },
}
```

**Step 2: 在 Commands 枚举添加 sync 变体**

```rust
pub enum Commands {
    // ... existing ...
    Sync(SyncCommands),
    Cache(CacheCommands),
}
```

**Step 3: 添加 CacheCommands 枚举**

```rust
#[derive(Parser)]
pub enum CacheCommands {
    /// 显示缓存状态
    Status,
    /// 清除缓存
    Clear,
}
```

**Step 4: 在 main.rs 实现 sync/cache 命令处理**

```rust
Commands::Sync(SyncCommands::Sync { force }) => {
    let response = if force {
        client.sync(&["projects", "items", "sections", "labels", "filters"]).await?
    } else {
        client.sync_with_cache(&["projects", "items", "sections", "labels", "filters"]).await?
    };
    println!("Synced: {} projects, {} tasks", response.projects.len(), response.items.len());
}
Commands::Cache(CacheCommands::Status) => {
    if let Ok(Some(cache)) = client.load_cache() {
        println!("Cached at: {}", cache.cached_at);
        println!("Projects: {}", cache.data.projects.len());
        println!("Tasks: {}", cache.data.items.len());
    } else {
        println!("No cache found");
    }
}
Commands::Cache(CacheCommands::Clear) => {
    client.clear_cache()?;
    println!("Cache cleared");
}
```

**Step 5: 在 TodoistSyncClient 添加 clear_cache 方法**

```rust
pub fn clear_cache(&self) -> Result<(), TodoError> {
    if self.cache_manager.cache_path.exists() {
        std::fs::remove_file(&self.cache_manager.cache_path)?;
    }
    *self.cache.borrow_mut() = None;
    Ok(())
}
```

**Step 6: 运行 cargo build**

Run: `cd .worktrees/local-cache && cargo build`
Expected: SUCCESS

**Step 7: 提交**

```bash
cd .worktrees/local-cache
git add src/cli/mod.rs src/main.rs src/sync/client.rs
git commit -m "feat: add sync and cache CLI commands"
```

---

## Task 5: 测试覆盖

**Files:**
- Create: `tests/unit/cache_test.rs`
- Modify: `tests/integration_test.rs`

**Step 1: 单元测试**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_cache_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let cache_path = temp_dir.path().join("cache.json");
        
        let cache = Cache {
            sync_token: "test_token".to_string(),
            cached_at: 1234567890,
            data: CacheData::default(),
        };
        
        let content = serde_json::to_string_pretty(&cache).unwrap();
        fs::write(&cache_path, &content).unwrap();
        
        let loaded: Cache = serde_json::from_str(&fs::read_to_string(&cache_path).unwrap()).unwrap();
        assert_eq!(loaded.sync_token, "test_token");
    }

    #[test]
    fn test_cache_expired() {
        let manager = CacheManager { cache_path: PathBuf::from("/nonexistent") };
        let old_cache = Cache {
            sync_token: "test".to_string(),
            cached_at: 1, // very old
            data: CacheData::default(),
        };
        assert!(manager.is_expired(&old_cache, 300));
    }
}
```

**Step 2: 运行测试**

Run: `cd .worktrees/local-cache && cargo test`
Expected: ALL PASS

**Step 3: 提交**

```bash
cd .worktrees/local-cache
git add tests/
git commit -m "test: add cache unit tests"
```

---

## Task 6: 最终验证

**Step 1: 运行完整测试**

Run: `cd .worktrees/local-cache && cargo test --all`
Expected: ALL PASS

**Step 2: 构建 release 版本**

Run: `cd .worktrees/local-cache && cargo build --release`
Expected: SUCCESS

**Step 3: 提交**

```bash
cd .worktrees/local-cache
git add .
git commit -m "chore: ready for PR - local cache feature complete"
```

---

## 执行选择

**Plan complete and saved to `docs/plans/2026-02-16-local-cache-plan.md`. 两个执行选项:**

1. **Subagent-Driven (本会话)** - 每个任务 spawn 一个子代理，任务间 review，快速迭代
2. **Parallel Session (新会话)** - 在 worktree 中打开新会话，用 executing-plans 批量执行

**选哪个？** 🔮

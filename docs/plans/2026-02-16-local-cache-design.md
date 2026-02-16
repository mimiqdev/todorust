# Todoist 本地缓存设计

> **创建日期**: 2026-02-16
> **版本**: v1.0

## 1. 目标

利用 Todoist Sync API 的 `sync_token` 实现增量同步，将数据持久化到本地文件，减少 API 调用次数。

## 2. 存储方案

### 2.1 文件位置

```
~/.config/todorust/
├── config.json
└── cache.json      # 本地缓存
```

### 2.2 cache.json 结构

```json
{
  "sync_token": "xxx",
  "cached_at": 1234567890,
  "data": {
    "projects": [...],
    "items": [...],
    "sections": [...],
    "labels": [...],
    "filters": [...]
  }
}
```

## 3. 同步策略 (混合模式)

### 3.1 首次同步
- 调用 `sync` 不带 `sync_token` (即 `sync_token: "*"`)
- 获取全量数据
- 保存到 cache.json

### 3.2 混合触发条件
- **定时刷新**: get 命令时检查 `cached_at`，超过阈值(默认 5 分钟)则增量 sync
- **命令后同步**: 每次 add/edit/complete/delete/move 后执行增量 sync
- **手动强制刷新**: `todorust sync --force` 强制全量 sync

### 3.3 增量同步流程
1. 读取本地 cache.json 获取 sync_token
2. 调用 `sync` 传入 sync_token
3. Todoist 返回增量数据 + 新 sync_token
4. 合并增量数据到本地缓存
5. 保存更新后的 cache.json

## 4. 核心模块设计

### 4.1 CacheManager

```rust
pub struct CacheManager {
    cache_path: PathBuf,
}

impl CacheManager {
    pub fn load() -> Result<Cache, TodoError>;
    pub fn save(&self, cache: &Cache) -> Result<(), TodoError>;
    pub fn is_expired(&self, cache: &Cache, threshold_secs: u64) -> bool;
}
```

### 4.2 缓存数据结构

```rust
#[derive(Serialize, Deserialize)]
pub struct Cache {
    pub sync_token: String,
    pub cached_at: i64,
    pub data: CacheData,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CacheData {
    pub projects: Vec<SyncProject>,
    pub items: Vec<SyncTask>,
    pub sections: Vec<SyncSection>,
    pub labels: Vec<SyncLabel>,
    pub filters: Vec<SyncFilter>,
}
```

## 5. CLI 扩展

### 5.1 新增命令

```bash
# 手动同步
todorust sync              # 增量同步
todorust sync --force      # 强制全量同步

# 查看缓存状态
todorust cache status      # 显示缓存时间、状态
todorust cache clear       # 清除缓存
```

### 5.2 配置扩展

```bash
# 设置缓存过期时间 (秒)
todorust config set cache_ttl 300

# 禁用缓存
todorust config set cache_enabled false
```

## 6. 错误处理

| 场景 | 处理 |
|-----|------|
| cache.json 不存在 | 执行全量 sync |
| sync_token 过期 | 自动执行全量 sync |
| 网络错误 | 返回错误，可选使用过期缓存 |
| 缓存写入失败 | 回退到内存，重试写入 |

## 7. 离线支持 (后续)

- 保留字段: `offline_mode: bool`
- get 命令读取本地缓存
- 命令队列缓存，联网后执行

## 8. 实施计划

### Phase 1: 核心功能
1. 创建 CacheManager 模块
2. 实现 cache.json 读写
3. 修改 TodoistSyncClient 集成缓存
4. 实现混合同步逻辑

### Phase 2: CLI 集成
1. 添加 `sync` 命令
2. 添加 `cache` 子命令
3. 添加配置选项

### Phase 3: 优化
1. 错误处理增强
2. 离线支持 (可选)

# Todoist Sync API 迁移

## 文档获取记录

**日期**: 2026-02-11

### 已完成的操作

1. ✅ 获取 Todoist Sync API v9 官方文档
2. ✅ 保存到本地: `docs/todoist-sync-api.md`
3. ✅ 创建迁移计划: `docs/sync-migration-plan.md`

### 文档链接

- **官方文档**: https://developer.todoist.com/sync/v9/
- **REST API v2**: https://developer.todoist.com/rest/v2/
- **开发者门户**: https://developer.todoist.com/

### API 版本对比

| 版本 | 端点 | 特性 |
|------|------|------|
| REST API v1 | `/api/v1/` | 当前使用 |
| REST API v2 | `/rest/v2/` | 新版 REST 端点 |
| **Sync API v9** | `/sync/v9/` | 完整功能，增量同步 |

### 迁移动机

1. **分区 (Sections) 支持**: Sync API 完整支持分区操作
2. **增量同步**: 减少不必要的数据传输
3. **批量操作**: 单次请求执行多个命令
4. **统一端点**: 所有资源通过 `/sync` 访问

### 相关文件

- `docs/todoist-sync-api.md` - 完整的 Sync API 文档
- `docs/sync-migration-plan.md` - 详细的迁移计划

### 关键 API 端点

```bash
# Sync 基础端点
POST https://api.todoist.com/sync/v9/sync

# 读取资源
resource_types: ["projects", "items", "sections", "filters", "labels"]

# 写入资源 (Commands)
item_add, item_update, item_delete, item_complete
section_add, section_update, section_delete
project_add, project_update, project_delete
```

### 下一步行动

1. 开始阶段 1: 基础架构重构
2. 创建 TodoistSyncClient 结构体
3. 迁移现有 API 方法到 Sync 端点

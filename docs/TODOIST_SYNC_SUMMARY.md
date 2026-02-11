# Todoist v1 Sync API 快速参考

## 文档信息
- **官方链接**: https://developer.todoist.com/api/v1
- **获取日期**: 2026-02-11
- **完整文档**: 请查看 `todoist-sync-api.md`

## 核心概念

### 1. Sync 端点基础
- **端点**: `POST https://api.todoist.com/api/v1/sync`
- **请求格式**: `application/x-www-form-urlencoded`
- **响应格式**: JSON

### 2. 支持的资源类型 (Resource Types)
- `labels` - 标签
- `projects` - 项目
- `items` - 任务
- `notes` - 备注
- `sections` - 区段
- `filters` - 过滤器
- `reminders` - 提醒
- `reminders_location` - 位置提醒
- `locations` - 位置
- `user` - 用户
- `live_notifications` - 实时通知
- `collaborators` - 协作者
- `user_settings` - 用户设置
- `notification_settings` - 通知设置
- `user_plan_limits` - 用户计划限制
- `completed_info` - 完成信息
- `stats` - 统计
- `workspaces` - 工作区
- `workspace_users` - 工作区用户
- `workspace_filters` - 工作区过滤器
- `view_options` - 视图选项
- `project_view_options_defaults` - 项目视图默认选项
- `role_actions` - 角色操作

**特殊用法**:
- 使用 `"all"` 获取所有资源类型
- 使用 `-` 前缀排除某些资源，例如 `"-projects"`

### 3. Full Sync vs Incremental Sync

#### Full Sync (完整同步)
- 首次同步时使用 `sync_token=*`
- 返回所有活动数据
- 返回 `full_sync: true`
- 包含 `full_sync_date_utc` 时间戳

#### Incremental Sync (增量同步)
- 使用上次同步返回的 `sync_token`
- 仅返回自上次同步以来更改的数据
- 更高效，减少数据传输
- 返回 `full_sync: false`

### 4. 命令 (Commands)

#### 常用命令类型:
- **item_add** - 添加任务
- **item_update** - 更新任务
- **item_delete** - 删除任务
- **item_complete** - 完成任务
- **item_uncomplete** - 取消完成任务
- **project_add** - 添加项目
- **project_update** - 更新项目
- **project_delete** - 删除项目
- **section_add** - 添加区段
- **section_update** - 更新区段
- **section_delete** - 删除区段
- **label_add** - 添加标签
- **label_update** - 更新标签
- **label_delete** - 删除标签

#### 命令限制:
- 每个请求最多 100 个命令
- 需要提供 `uuid` (命令唯一标识符)
- 支持 `temp_id` (临时资源 ID)

### 5. 临时 ID (Temporary IDs)

**用途**:
- 在创建新资源时使用临时 ID
- 格式: 任意唯一字符串
- 在响应中通过 `temp_id_mapping` 映射到真实 ID

**示例流程**:
1. 发送带有 `temp_id` 的命令
2. 收到包含 `temp_id_mapping` 的响应
3. `temp_id_mapping` 包含 `{临时ID: 真实ID}` 映射

### 6. Sections (区段) 操作

#### 获取 Sections:
- 通过 `resource_types=["sections"]` 获取所有区段
- 支持通过 REST API 搜索特定区段

#### Sections 命令:
- `section_add` - 添加区段
- `section_update` - 更新区段  
- `section_move` - 移动区段
- `section_reorder` - 重新排序区段
- `section_delete` - 删除区段
- `section_archive` - 归档区段
- `section_unarchive` - 取消归档区段

#### Sections 与任务:
- 任务可以属于某个区段 (`section_id`)
- 区段内的任务按顺序排列
- 移动任务到区段使用 `item_move` 命令

## 请求示例

### 完整同步请求
```bash
curl https://api.todoist.com/api/v1/sync \
  -H "Authorization: Bearer TOKEN" \
  -d 'sync_token=*' \
  -d 'resource_types=["all"]'
```

### 增量同步请求
```bash
curl https://api.todoist.com/api/v1/sync \
  -H "Authorization: Bearer TOKEN" \
  -d 'sync_token=上次返回的sync_token' \
  -d 'resource_types=["items", "projects"]'
```

### 写入命令请求
```bash
curl https://api.todoist.com/api/v1/sync \
  -H "Authorization: Bearer TOKEN" \
  -d 'commands=[{"type": "item_add", "temp_id": "uuid-123", "uuid": "uuid-456", "args": {"content": "Buy milk"}}]'
```

## 响应格式

### 读取响应
```json
{
  "sync_token": "新的sync_token",
  "full_sync": false,
  "items": [...],
  "projects": [...],
  "sections": [...],
  ...
}
```

### 写入响应
```json
{
  "sync_token": "新的sync_token",
  "temp_id_mapping": {
    "uuid-123": 123456789
  },
  "sync_status": {
    "uuid-456": "ok"
  }
}
```

## 重要注意事项

1. **v1 版本**: 这是当前唯一的 API 版本，包含了原 v9 Sync API 和 v2 REST API 的功能
2. **认证**: 使用 Bearer Token
3. **批量处理**: 支持在单个请求中读取和写入多个资源
4. **增量同步**: 建议在初始完整同步后使用增量同步以提高效率
5. **命令 UUID**: 每个命令应提供唯一 UUID 用于追踪
6. **临时 ID**: 创建资源时使用临时 ID 可确保客户端能正确映射引用

## 迁移自 v9

- 端点从 `/sync/v9/sync` 变更为 `/api/v1/sync`
- 对象命名从驼峰命名法变更为下划线命名法
- Sections 响应格式统一使用 Sync API 格式
- 更多信息请参考完整文档中的迁移指南

## 相关链接
- **完整文档**: `todoist-sync-api.md`
- **官方文档**: https://developer.todoist.com/api/v1
- **Python SDK**: https://doist.github.io/todoist-api-python
- **JavaScript SDK**: https://doist.github.io/todoist-api-typescript

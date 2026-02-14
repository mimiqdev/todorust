# Product Requirements Document: todorust-cli

**Version**: 1.0
**Date**: 2026-01-15
**Author**: Sarah (Product Owner)
**Quality Score**: 90/100

---

## Executive Summary

Todorust-cli是一个专为AI工具和脚本集成的Todoist CLI工具，旨在简化Todoist API的调用过程，提供结构化、标准化的JSON输出格式。该工具解决了在CLI环境中调用Todoist API的复杂性，特别是为AI工具提供易于解析的数据格式，使得自动化工作流（如Obsidian周报生成）能够轻松集成Todoist的任务管理功能。

通过支持完整的Todoist filter语法、精简的JSON输出和简单的配置管理，todorust-cli将成为开发者个人工具链中的重要组成部分，显著提升自动化效率。

---

## Problem Statement

**Current Situation**: 在CLI环境或AI工具中调用Todoist API需要处理复杂的HTTP请求、认证、数据解析和格式转换。特别是在自动化工作流中（如Obsidian插件或AI助手），需要快速、准确地获取和操作Todoist数据，但缺乏一个专门为此场景优化的CLI工具。

**Proposed Solution**: 创建一个基于Rust的CLI工具，提供简洁的命令行接口，支持完整的Todoist API调用，返回结构化的精简JSON格式，并自动处理认证和错误。

**Business Impact**: 提升自动化工作流的开发效率，降低集成Todoist API的技术门槛，使开发者能够在Obsidian、AI助手等环境中快速实现任务管理功能。

---

## Success Metrics

**Primary KPIs:**
- **功能完整性**: 所有核心API调用（获取任务、项目、filter列表、创建/更新任务）都能正常返回有效数据（100%成功率）
- **API响应率**: API调用成功率 ≥ 95%（排除网络和Todoist服务端错误）
- **使用便利性**: 工具安装后，首次成功调用时间 < 5分钟

**Validation**:
- 通过单元测试验证所有核心功能
- 在Obsidian workflow中完成周报生成场景的端到端测试
- 收集用户反馈并评估安装和使用的便利性

---

## User Personas

### Primary: 自动化工具开发者
- **Role**: 个人开发者，使用脚本和AI工具提升工作效率
- **Goals**:
  - 快速集成Todoist到个人工作流（Obsidian、AI助手等）
  - 获取结构化的任务数据用于自动化处理
  - 使用filter语法精确查询所需任务
- **Pain Points**:
  - Todoist API文档复杂，直接调用耗时
  - 需要手动处理认证、JSON解析、错误处理
  - 缺乏适合AI工具解析的标准化输出格式
- **Technical Level**: Advanced（熟悉命令行、脚本、JSON、API）

### Secondary: AI工具/脚本
- **Role**: 程序化调用工具（如Obsidian插件、AI助手、shell脚本）
- **Goals**:
  - 通过命令行快速获取Todoist数据
  - 解析标准化的JSON格式数据
  - 执行简单的任务操作（创建、更新、完成）
- **Pain Points**:
  - 需要稳定的API接口和返回格式
  - 期望简单的错误处理机制
  - 需要清晰、结构化的数据输出
- **Technical Level**: N/A（程序化调用）

---

## User Stories & Acceptance Criteria

### Story 1: 获取任务列表

**As a** 自动化工具开发者
**I want to** 使用filter查询特定条件的任务（如"WORK项目中本周完成的任务"）
**So that** 我可以将任务数据集成到我的工作流中（如生成周报）

**Acceptance Criteria:**
- [ ] 支持完整的Todoist filter语法（如 `project:WORK & completed within "7 days of today"`）
- [ ] 支持多种输出格式：JSON（默认）、Markdown checklist、Markdown structured
- [ ] 返回精简的JSON格式，包含：id, content, project_id, project_name, due_date, is_completed, created_at, order, priority, labels, tags
- [ ] 支持通过project_id、label、日期范围等参数过滤
- [ ] 支持分页查询
- [ ] 错误时返回标准HTTP状态码和简单的错误信息
- [ ] `--format` 参数支持全局设置和命令级别覆盖

### Story 2: 获取项目列表

**As a** 自动化工具开发者
**I want to** 获取所有Todoist项目
**So that** 我可以了解项目结构并用于后续的任务查询

**Acceptance Criteria:**
- [ ] 返回所有项目的精简JSON数组
- [ ] 包含字段：id, name, color, is_shared, is_favorite, url
- [ ] 错误时返回标准HTTP状态码和简单的错误信息

### Story 3: 获取自定义filter列表

**As a** 自动化工具开发者
**I want to** 获取我的Todoist自定义filter
**So that** 我可以快速使用预定义的查询条件

**Acceptance Criteria:**
- [ ] 返回所有自定义filter的精简JSON数组
- [ ] 包含字段：id, name, query（filter表达式）
- [ ] 错误时返回标准HTTP状态码和简单的错误信息

### Story 4: 创建任务

**As a** 自动化工具开发者
**I want to** 创建新的任务
**So that** 我可以通过脚本或AI工具快速添加任务

**Acceptance Criteria:**
- [ ] 支持必需参数：content（任务内容）
- [ ] 支持可选参数：project_id, due_date, priority, labels, tags
- [ ] 成功时返回创建的任务的精简JSON
- [ ] 错误时返回标准HTTP状态码和简单的错误信息

### Story 5: 更新任务状态

**As a** 自动化工具开发者
**I want to** 更新任务的状态（完成/重新打开）
**So that** 我可以通过脚本批量处理任务

**Acceptance Criteria:**
- [ ] 支持通过task_id标记任务为完成
- [ ] 支持通过task_id重新打开已完成的任务
- [ ] 成功时返回204 No Content状态码
- [ ] 错误时返回标准HTTP状态码和简单的错误信息

---

## Functional Requirements

### Core Features

**Feature 1: 任务查询**
- Description: 使用Todoist filter语法查询任务，返回多种格式输出（JSON/Markdown）
- User flow:
  1. 用户执行 `todorust tasks --filter="project:WORK & completed within \"7 days of today\"" --format checklist`
  2. 工具从配置文件读取API token
  3. 调用Todoist API v1的 `/api/v1/tasks` 端点
  4. 解析API响应，提取核心字段
  5. 关联project_id到project_name
  6. 根据--format参数格式化输出（默认JSON）
  7. 返回格式化结果
- Edge cases:
  - API返回空结果：返回空数组 `[]`（JSON）或空文本（Markdown）
  - 网络超时：返回HTTP 500错误
  - 认证失败：返回HTTP 401错误
- Error handling:
  - 网络错误：返回500状态码，错误信息包含简要说明
  - API错误：透传Todoist API的状态码和简单错误消息
  - 配置文件缺失：返回400状态码，提示用户配置API token
- Output formats:
  - `json`: 精简JSON数组（默认，用于程序化访问）
  - `checklist`: Markdown checklist格式（用于Obsidian笔记）
  - `structured`: 按项目分组的Markdown报告（用于周报）

**Feature 2: 项目查询**
- Description: 获取所有Todoist项目列表
- User flow:
  1. 用户执行 `todorust projects`
  2. 工具从配置文件读取API token
  3. 调用Todoist API的 `/api/v1/projects` 端点
  4. 解析API响应，提取核心字段
  5. 返回精简的JSON数组
- Edge cases: 同Feature 1
- Error handling: 同Feature 1

**Feature 3: Filter查询**
- Description: 获取用户自定义的filter列表
- User flow:
  1. 用户执行 `todorust filters`
  2. 工具从配置文件读取API token
  3. 调用Todoist Sync API的 `/api/v1/sync` 端点，resource_types=["filters"]
  4. 解析API响应，提取filter信息
  5. 返回精简的JSON数组
- Edge cases: 同Feature 1
- Error handling: 同Feature 1

**Feature 4: 任务创建**
- Description: 创建新的Todoist任务
- User flow:
  1. 用户执行 `todorust create --content="Buy milk" --project-id="123"`
  2. 工具从配置文件读取API token
  3. 调用Todoist API的 `/api/v1/tasks` 端点（POST）
  4. 解析API响应，提取创建的任务信息
  5. 返回精简的JSON对象
- Edge cases:
  - 缺少必需参数：返回400状态码，提示缺少content参数
  - project_id不存在：透传API错误（404）
- Error handling: 同Feature 1

**Feature 5: 任务更新**
- Description: 更新任务状态（完成/重新打开）
- User flow:
  1. 用户执行 `todorust complete --task-id="456"`
  2. 工具从配置文件读取API token
  3. 调用Todoist API的 `/api/v1/tasks/{task_id}/close` 端点（POST）
  4. 返回204 No Content状态码
- Edge cases:
  - task_id不存在：返回404状态码
  - 任务已完成：仍然返回成功（幂等操作）
- Error handling: 同Feature 1

### Out of Scope
- 删除任务或项目
- 标签管理（CRUD）
- Section管理
- 任务评论
- 提醒功能
- 用户信息查询
- 协作者管理
- 复杂的批量操作
- WebSocket实时同步
- OAuth认证流程（仅支持API token）

---

## Technical Constraints

### Performance
- 无特殊性能要求，API响应时间主要由Todoist服务决定
- 本地工具处理时间应在合理范围内（< 500ms，不包括网络传输）

### Security
- 认证方式：使用Todoist API token（个人token或OAuth token）
- Token存储：明文存储在用户本地配置文件 `~/.config/todorust/config.toml`
- 数据传输：使用HTTPS，支持CORS
- 不涉及敏感数据加密（Token由用户自行保管）

### Integration
- **Todoist API v1**: 使用最新的REST API v1（`https://api.todoist.com/api/v1`）
  - Tasks: `/api/v1/tasks`, `/api/v1/tasks/{id}/close`, `/api/v1/tasks/{id}/reopen`
  - Projects: `/api/v1/projects`
  - Filters: 通过Sync API `/api/v1/sync` 获取
- **Obsidian**: 通过命令行调用，JSON输出可直接被Obsidian插件或脚本解析
- **AI工具**: 提供标准化的JSON输出，易于LLM理解和处理

### Technology Stack
- **语言**: Rust
- **CLI框架**: clap或类似的命令行参数解析库
- **HTTP客户端**: reqwest或hyper
- **配置文件**: TOML格式（使用toml crate）
- **JSON处理**: serde_json
- **错误处理**: thiserror或anyhow
- **兼容性**: Linux, macOS, Windows（跨平台）
- **依赖管理**: Cargo

---

## MVP Scope & Phasing

### Phase 1: MVP (Required for Initial Launch)
- 获取任务列表（支持完整filter语法）
- 获取项目列表
- 获取自定义filter列表
- 创建任务
- 更新任务状态（完成/重新打开）
- 配置文件管理（TOML格式，API token）
- 多种输出格式支持（JSON、Markdown checklist、Markdown structured）
- 基础错误处理（标准HTTP状态码）

**MVP Definition**: 完成上述所有功能，能够在Obsidian workflow中实现"获取WORK项目中本周完成的任务并生成周报"的端到端场景。支持`--format`参数直接输出Markdown格式，无需额外解析。

### Phase 2: Enhancements (Post-Launch)
- 支持更新任务的更多属性（due_date, priority, labels等）
- 支持删除任务
- 支持标签管理
- 支持Section查询
- 支持批量操作（批量创建/更新任务）
- 支持更详细的错误信息和日志
- 支持多个配置文件（不同账户切换）
- 支持输出格式的自定义（Markdown等）

### Future Considerations
- 支持Todoist Sync API的增量同步
- 支持WebSocket实时更新
- 支持离线模式（本地缓存）
- 支持OAuth认证流程
- 支持Web UI或TUI（终端用户界面）
- 支持插件系统（自定义输出格式、过滤器）

---

## Risk Assessment

| Risk | Probability | Impact | Mitigation Strategy |
|------|------------|--------|---------------------|
| Todoist API变更或废弃 | Medium | High | 使用稳定的API v1，关注Todoist API更新通知，预留适配时间 |
| API速率限制 | High | Medium | 实现基本的速率限制检测，在配置文件中提供自定义延迟选项 |
| 用户配置错误（如无效token） | High | Low | 提供清晰的错误提示，在首次运行时引导用户配置 |
| 精简JSON格式无法满足所有场景 | Medium | Medium | MVP阶段使用固定格式，Phase 2支持自定义输出格式 |
| 跨平台兼容性问题 | Low | Medium | 使用跨平台的Rust库，在主流平台上测试 |

---

## Dependencies & Blockers

**Dependencies:**
- Todoist API v1服务稳定性：外部依赖，无法控制
- Rust生态系统依赖（clap, reqwest, serde等）：需要维护版本兼容性
- 用户网络连接：需要稳定的互联网连接

**Known Blockers:**
- 无已知阻塞项，可以立即开始开发

---

## Appendix

### Glossary
- **Filter**: Todoist的查询语法，用于按条件过滤任务（如 `project:WORK & priority:4`）
- **API Token**: Todoist的个人访问令牌，用于API认证
- **TOML**: Tom's Obvious, Minimal Language，一种配置文件格式
- **JSON**: JavaScript Object Notation，轻量级数据交换格式
- **MVP**: Minimum Viable Product，最小可行产品
- **CLI**: Command Line Interface，命令行接口

### References
- Todoist API v1文档: https://developer.todoist.com/api/v1/
- Todoist Filter语法文档: https://todoist.com/help/articles/introduction-to-filters-V98wIH
- Rust语言官方文档: https://www.rust-lang.org/
- clap CLI框架: https://github.com/clap-rs/clap
- reqwest HTTP客户端: https://github.com/seanmonstar/reqwest

---

*This PRD was created through interactive requirements gathering with quality scoring to ensure comprehensive coverage of business, functional, UX, and technical dimensions.*

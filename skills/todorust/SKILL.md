---
name: todorust
description: Manage Todoist tasks, projects, sections, filters, and labels via the `todorust` CLI. Use when a user asks to manage their Todoist data, including creating tasks, listing projects, completing tasks, or moving tasks between sections.
homepage: https://github.com/mimiqdev/todorust
metadata: {"gemini":{"emoji":"✅","requires":{"bins":["todorust"]}}}
---

# Todorust

Todoist CLI tool with optimized JSON output for AI agents.

## Setup

- Install: `cargo install --path .` (from project root)
- Configure: `todorust init --api-token YOUR_TOKEN`
- Format options: `--format json | checklist | structured`

## Commands

### Read Resources

```bash
# Get tasks (with optional filter)
todorust get tasks --filter "today"

# Get tasks with specific fields to save tokens
todorust get tasks --fields "id,content,priority"

# Limit results
todorust get tasks --limit 10

# Get all projects
todorust get projects

# Get all sections (optionally for a project)
todorust get sections --project-id "12345678"
```

### Batch Operations

Highly efficient for AI agents to perform multiple actions in one sync.

```bash
todorust batch '[
  {"type": "item_add", "args": {"content": "Task 1"}},
  {"type": "item_complete", "args": {"id": "12345"}}
]'
```

### Create Resources

Returns JSON with the new item ID.

```bash
# Basic task
todorust add task --title "Buy milk"

# Task with description, project, due date, and priority (1-4)
todorust add task --title "Review PR" --description "Check the sync logic" --project-id "222" --due-date "tomorrow" --priority 4

# Create a project
todorust add project --name "Side Project"
```

### Update/Move Resources

```bash
# Edit a task
todorust edit task --task-id "123" --title "New Title" --priority 3

# Move a task
todorust move task --task-id "123" --project-id "456" --section-id "789"

# Complete/Reopen
todorust complete task --task-id "123"
todorust reopen task --task-id "123"
```

## Filter Syntax (for `get tasks --filter`)

| Filter Type | Example |
|-------------|---------|
| Keyword     | `todorust get tasks --filter "milk"` (matches content or project) |
| Priority    | `todorust get tasks --filter "p:4"` (1-4) |
| Status      | `todorust get tasks --filter "is:completed"` or `"active"` |

## Output Formats

- `json`: Full JSON output (default). Mutations also return JSON.
- `checklist`: Markdown checklist (`- [ ] task (Project)`).
- `structured`: Markdown grouped by project with headings.

## Examples for Agents

### 1. Finding and Completing a Task
```bash
# 1. Search
todorust get tasks --filter "Buy milk" --fields "id,content"
# 2. Complete using ID from JSON
todorust complete task --task-id "ID_FROM_JSON"
```

### 2. Multi-action Sync
If a user wants to move and complete a task:
```bash
todorust batch '[
  {"type": "item_move", "args": {"id": "123", "project_id": "456"}},
  {"type": "item_complete", "args": {"id": "123"}}
]'
```

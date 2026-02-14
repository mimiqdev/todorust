# Todorust Usage Guide

## Core Commands

### Get Resources
```bash
# Basic retrieval
todorust get tasks --filter "today"
todorust get projects

# AI Optimizations
todorust get tasks --limit 10
todorust get tasks --fields "id,content"
```

### Add Resources
Returns JSON response with the new item's details.
```bash
todorust add task --title "Buy milk"
todorust add project --name "Side Project"
todorust add label --name "urgent"
todorust add filter --name "Today" --query "today"
```

### Manage Tasks
```bash
todorust complete task --task-id "123"
todorust reopen task --task-id "123"
todorust move task --task-id "123" --project-id "456"
todorust delete task --task-id "123"
```

### Batch Operations
Execute multiple Sync API commands in one request.
```bash
todorust batch '[
  {"type": "item_add", "args": {"content": "Task 1"}},
  {"type": "item_complete", "args": {"id": "123"}}
]'
```

### Shell Completion
```bash
# For zsh
todorust completion zsh > ~/.zfunc/_todorust
# For bash
todorust completion bash > /etc/bash_completion.d/todorust
```

## Filter Syntax Examples

The `--filter` flag supports keyword matching and some key-value patterns:

- **Keywords**: `Work`, `shopping` (matches content or project)
- **Priority**: `p:4`, `priority:1` (1-4)
- **Status**: `is:completed`, `active`, `incomplete`

## JSON Output Format

### Task
```json
{
  "id": "123",
  "content": "Task name",
  "description": "Details about the task",
  "project_id": "456",
  "project_name": "Work",
  "due_date": "2026-01-15",
  "is_completed": false,
  "created_at": "2026-01-10T10:00:00Z",
  "order": 1,
  "priority": 4,
  "labels": ["urgent"]
}
```

# Todorust Usage Guide

## Core Commands

### Get Resources
```bash
todorust get tasks --filter "today"
todorust get projects
todorust get filters
todorust get labels
todorust get sections --project-id "123"
```

### Add Resources
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

## Filter Syntax Examples

### By Project
```
project:Work
project:Personal
```

### By Date
```
due today
due tomorrow
due within "7 days of today"
completed within "7 days of today"
```

### By Priority
```
priority:4  # High
priority:3  # Normal
priority:2  # Low
priority:1  # None
```

### Combined Filters
```
project:Work & priority:4
project:Work & due within "7 days of today" & !completed
```

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

### Project
```json
{
  "id": "456",
  "name": "Work",
  "color": "blue",
  "is_shared": false,
  "is_favorite": true,
  "url": "https://todoist.com/showProject/456"
}
```

### Filter
```json
{
  "id": "789",
  "name": "This Week",
  "query": "due within \"7 days of today\""
}
```

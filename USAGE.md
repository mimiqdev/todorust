# Todorust Usage Guide

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

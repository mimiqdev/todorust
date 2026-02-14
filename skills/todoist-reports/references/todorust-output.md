# Todorust Output Format Reference

JSON output schemas for all todorust commands.

## Task Output

### Single Task Object

Returned by: `todorust get tasks` (in an array) or when adding a task.

```json
{
  "id": "123456789",
  "content": "Complete project proposal",
  "project_id": "987654321",
  "project_name": "Work",
  "due_date": "2026-01-15",
  "is_completed": false,
  "created_at": "2026-01-10T10:30:00Z",
  "order": 1,
  "priority": 4,
  "labels": ["urgent", "review"]
}
```

### Task Array

Returned by: `todorust get tasks`, `todorust get tasks --filter "..."`

```json
[
  {
    "id": "123",
    "content": "Task 1",
    "project_id": "456",
    "project_name": "Work",
    "due_date": "2026-01-15",
    "is_completed": true,
    "created_at": "2026-01-10T10:00:00Z",
    "order": 1,
    "priority": 4,
    "labels": ["urgent"]
  }
]
```

## Project Output

### Project Array

Returned by: `todorust get projects`

```json
[
  {
    "id": "456",
    "name": "Work",
    "color": "blue",
    "is_favorite": true
  }
]
```

## Parsing Examples for Agents

### Using jq

**Extract task names:**
```bash
todorust get tasks | jq -r '.[].content'
```

**Filter for completed tasks today (if AI is doing the filtering):**
```bash
todorust get tasks | jq '.[] | select(.is_completed == true)'
```

**Format as checklist:**
```bash
todorust get tasks | jq -r '.[] | "- [\(if .is_completed then "x" else " " end)] \(.content)"'
```

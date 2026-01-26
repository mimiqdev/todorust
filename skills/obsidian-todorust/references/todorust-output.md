# Todorust Output Format Reference

JSON output schemas for all todorust commands.

## Task Output

### Single Task Object

Returned by: `todorust create`, `todorust tasks` (array)

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

### Field Descriptions

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Unique task identifier |
| `content` | string | Task title/name |
| `description` | string/null | Task description/details |
| `project_id` | string/null | Project ID (null if no project) |
| `project_name` | string/null | Enriched project name (null if no project) |
| `due_date` | string/null | Due date in YYYY-MM-DD format (null if no date) |
| `is_completed` | boolean | Task completion status |
| `created_at` | string | ISO 8601 timestamp |
| `order` | number | Task ordering index |
| `priority` | number | 1=None, 2=Low, 3=Normal, 4=High |
| `labels` | array | List of label strings |

### Task Array

Returned by: `todorust tasks`, `todorust tasks --filter`

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
  },
  {
    "id": "124",
    "content": "Task 2",
    "project_id": "789",
    "project_name": "Personal",
    "due_date": null,
    "is_completed": true,
    "created_at": "2026-01-11T14:30:00Z",
    "order": 2,
    "priority": 2,
    "labels": []
  }
]
```

## Project Output

### Project Array

Returned by: `todorust projects`

```json
[
  {
    "id": "456",
    "name": "Work",
    "color": "blue",
    "is_shared": false,
    "is_favorite": true
  },
  {
    "id": "789",
    "name": "Personal",
    "color": "green",
    "is_shared": false,
    "is_favorite": false
  }
]
```

### Field Descriptions

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Unique project identifier |
| `name` | string | Project name |
| `color` | string | Project color identifier |
| `is_shared` | boolean | Whether project is shared |
| `is_favorite` | boolean | Whether project is favorited |

## Filter Output

### Filter Array

Returned by: `todorust filters`

```json
[
  {
    "id": "789",
    "name": "This Week",
    "query": "due within \"7 days of today\""
  },
  {
    "id": "790",
    "name": "Work High Priority",
    "query": "project:Work & priority:4"
  }
]
```

### Field Descriptions

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Unique filter identifier |
| `name` | string | Filter display name |
| `query` | string | Todoist filter query string |

## Parsing Examples

### Using jq

**Extract task names:**
```bash
todorust tasks | jq -r '.[].content'
```

**Extract completed tasks with project:**
```bash
todorust tasks --filter 'completed today' | \
  jq -r '.[] | "\(.content) (\(.project_name // "No Project"))"'
```

**Group by project:**
```bash
todorust tasks | jq 'group_by(.project_name) |
  map({project: .[0].project_name, count: length})'
```

**Count by priority:**
```bash
todorust tasks | jq 'group_by(.priority) |
  map({priority: .[0].priority, count: length})'
```

**Format as checklist:**
```bash
todorust tasks --filter 'completed today' | \
  jq -r '.[] | "- [\(if .is_completed then "x" else " " end)] \(.content)"'
```

### Using Python

```python
import json
import subprocess

# Get tasks
result = subprocess.run(
    ['todorust', 'tasks', '--filter', 'completed today'],
    capture_output=True,
    text=True
)

tasks = json.loads(result.stdout)

for task in tasks:
    print(f"- [x] {task['content']}")
    if task['project_name']:
        print(f"  Project: {task['project_name']}")
    if task['labels']:
        print(f"  Labels: {', '.join(task['labels'])}")
```

## Empty Results

All commands return empty arrays when no results:

```json
[]
```

## Error Output Format

Errors are printed to stderr, not JSON:

```
Error: Configuration not found.
Run: todorust init --api-token YOUR_TOKEN
```

Check exit code: `0` for success, non-zero for errors.

## Data Type Notes

- **All dates** are ISO 8601 or YYYY-MM-DD format
- **Priority values**: Always 1-4 (never null)
- **project_name**: Enriched field, may be null
- **labels**: Always array, may be empty `[]`
- **project_id**: May be null for tasks without projects
- **due_date**: May be null for tasks without due dates

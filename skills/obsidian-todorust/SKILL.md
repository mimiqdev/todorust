---
name: obsidian-todorust
description: Data retrieval skill for Todoist tasks and projects using todorust CLI. Provides structured JSON data from Todoist for report generation in Obsidian. Use when skills need to: (1) Fetch completed tasks by date range for daily/weekly reports, (2) Query tasks by project, priority, or custom filters, (3) Get project lists and metadata, (4) Parse todorust JSON output for integration with Obsidian workflows
---

# Obsidian Todoist Integration

Data retrieval skill for Todoist task and project information using todorust CLI. Use when other skills (daily notes, weekly reports, etc.) need to fetch structured task/project data from Todoist for report generation or analysis.

## Quick Start

### Check Installation

First verify todorust is installed and configured:

```bash
which todorust
todorust --help
```

If not installed, see [installation reference](references/todorust-setup.md).

### Basic Queries

**Get all active tasks:**
```bash
todorust tasks
```

**Get completed tasks today:**
```bash
todorust tasks --filter 'completed today'
```

**Get tasks completed this week:**
```bash
todorust tasks --filter 'completed within "7 days of today"'
```

**Get tasks for specific project:**
```bash
todorust tasks --filter 'project:Work'
```

**Get completed tasks for project this week:**
```bash
todorust tasks --filter 'project:Work & completed within "7 days of today"'
```

**Get all projects:**
```bash
todorust projects
```

### Common Filter Patterns

See [Todoist filter syntax](references/todoist-filters.md) for complete reference.

Quick reference:
- `completed today` - Tasks completed today
- `completed within "7 days of today"` - Completed in last 7 days
- `project:ProjectName` - Filter by project
- `priority:4` - High priority only
- Combine with `&` and `|` for complex filters

## Output Format

All todorust commands return JSON. See [output format reference](references/todorust-output.md) for complete schema.

**Task output includes:**
```json
{
  "id": "123",
  "content": "Task name",
  "project_id": "456",
  "project_name": "Work",
  "due_date": "2026-01-15",
  "is_completed": true,
  "priority": 4,
  "labels": ["urgent"]
}
```

**Project output includes:**
```json
{
  "id": "456",
  "name": "Work",
  "color": "blue",
  "is_shared": false,
  "is_favorite": true
}
```

## Usage in Other Skills

This skill provides raw data. Use it from other skills to:

1. **Call todorust** with appropriate filter
2. **Parse JSON output** using jq or native JSON parsing
3. **Transform data** for your report format

### Example: Daily Report

```bash
# Get today's completed tasks
TASKS=$(todorust tasks --filter 'completed today')

# Parse and format (example with jq)
echo "$TASKS" | jq -r '.[] | "- [x] \(.content) (Project: \(.project_name // "No Project"))"'
```

### Example: Weekly Report by Project

```bash
# Get this week's completed tasks
TASKS=$(todorust tasks --filter 'completed within "7 days of today"')

# Group by project
echo "$TASKS" | jq -r 'group_by(.project_name) | .[] |
  "## \(.[0].project_name // "No Project")\n" +
  (.[] | "- [x] \(.content)")'
```

### Example: With Date Range

For skills that have calculated start/end dates:

```bash
START_DATE="2026-01-10"
END_DATE="2026-01-16"

todorust tasks --filter "completed within \"$START_DATE to $END_DATE\""
```

## Error Handling

**Config not found:**
```
Error: Configuration not found.
Run: todorust init --api-token YOUR_TOKEN
```
→ User needs to initialize todorust first

**API errors:**
- Check token is valid
- Check network connectivity
- Verify filter syntax

**Empty results:**
- Normal if no tasks match filter
- Returns `[]` JSON array

## Integration Notes

**For daily note skills:**
- Use `completed today` filter
- Parse task list for checklist format
- Extract project_name for categorization

**For weekly review skills:**
- Use `completed within "7 days of today"` filter
- Or use custom date range if skill calculates week boundaries
- Group by project_name for structured report
- Use priority field for highlighting

**For custom date queries:**
- Todoist filter syntax supports date ranges
- Format: `completed within "START_DATE to END_DATE"`
- Dates in YYYY-MM-DD format

## Performance

- API calls take 1-3 seconds typically
- Cache results if needed for multiple reports
- todorust handles rate limiting internally

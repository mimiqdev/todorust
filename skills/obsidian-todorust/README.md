# Obsidian Todoist Integration Skill

This skill enables Obsidian + AI integrations (like Obsidian Copilot) to fetch Todoist tasks and projects using the todorust CLI tool.

## Purpose

This is a **data retrieval skill** - it provides structured task/project data from Todoist that other skills can use to generate daily notes, weekly reports, or other formatted output.

## What It Does

- Calls `todorust` CLI commands to fetch Todoist data
- Returns JSON output with tasks, projects, and filters
- Provides reference documentation for:
  - Todoist filter syntax
  - Todorust output formats
  - Installation and setup

## What It Doesn't Do

- ❌ Generate formatted reports (delegated to other skills)
- ❌ Create Obsidian notes directly (delegated to other skills)
- ❌ Handle report templates (delegated to other skills)

## Usage in Other Skills

Other skills should:

1. Call todorust with appropriate filters
2. Parse JSON output
3. Transform data for their report format

Example workflow for a daily note skill:
```bash
# Get today's completed tasks
TASKS=$(todorust tasks --filter 'completed today')

# Parse and format as needed
echo "$TASKS" | jq -r '.[] | "- [x] \(.content)"'
```

## Installation

1. Install todorust (see [references/todorust-setup.md](references/todorust-setup.md))
2. Configure with API token: `todorust init --api-token YOUR_TOKEN`
3. Test: `todorust projects`

## File Structure

```
obsidian-todorust/
├── SKILL.md                    # Main skill documentation
├── references/
│   ├── todorust-setup.md       # Installation guide
│   ├── todorust-output.md      # JSON output format reference
│   └── todoist-filters.md      # Filter syntax guide
├── scripts/
│   └── query_tasks.sh          # Helper script for common queries
└── assets/
    ├── example-daily-report.md
    └── example-weekly-report.md
```

## Common Queries

**Daily completed tasks:**
```bash
todorust tasks --filter 'completed today'
```

**Weekly completed tasks:**
```bash
todorust tasks --filter 'completed within "7 days of today"'
```

**Project-specific this week:**
```bash
todorust tasks --filter 'project:Work & completed within "7 days of today"'
```

**All projects:**
```bash
todorust projects
```

## Integration Examples

### Daily Notes Skill

Should:
1. Get current date
2. Call: `todorust tasks --filter 'completed today'`
3. Parse JSON for task list
4. Format as checklist in daily note template

### Weekly Review Skill

Should:
1. Calculate week start/end dates
2. Call: `todorust tasks --filter 'completed within "START to END"'`
3. Parse JSON and group by project
4. Format as structured weekly review

## Testing

Verify the skill works:

```bash
# Test todorust is accessible
which todorust

# Test basic queries
todorust projects
todorust tasks --filter 'completed today'

# Test JSON parsing
todorust tasks | jq '.'
```

## Support

- Todorust repo: `/home/tonyliu/Developer/projects/todoirust`
- Todoist API docs: https://developer.todoist.com/api/v1/
- Filter syntax: https://todoist.com/help/articles/205248802

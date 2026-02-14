# Todoist Reports Skill

This skill enables AI integrations to fetch and format Todoist tasks and projects using the `todorust` CLI tool, specifically optimized for note-taking applications like Obsidian.

## Purpose

This is a **data retrieval and formatting skill** - it provides structured data from Todoist that can be used to generate daily notes, weekly reports, or other formatted output.

## What It Does

- Calls `todorust` CLI commands to fetch Todoist data.
- Returns Markdown-ready output (checklists, structured lists) or JSON.
- Provides reference documentation for installation and usage.

## Usage in Other Skills

Other skills should:

1. Call `todorust` with appropriate filters and the desired format.
2. Use the output directly or parse the JSON for further transformation.

Example workflow for a daily note skill:
```bash
# Get today's tasks in checklist format
todorust get tasks --filter "today" --format checklist
```

## Installation

1. Install `todorust` (see [references/todorust-setup.md](references/todorust-setup.md))
2. Configure with API token: `todorust init --api-token YOUR_TOKEN`
3. Test: `todorust get projects`

## File Structure

```
todoist-reports/
├── SKILL.md                    # Main skill documentation
├── references/
│   ├── todorust-setup.md       # Installation guide
│   ├── todorust-output.md      # JSON output format reference
│   └── todoist-filters.md      # Filter syntax guide (reference only)
├── scripts/
│   └── query_tasks.sh          # Helper script for common queries
└── assets/
    ├── example-daily-report.md
    └── example-weekly-report.md
```

## Common Queries

**Today's tasks:**
```bash
todorust get tasks --filter "today"
```

**Work project tasks:**
```bash
todorust get tasks --filter "Work"
```

**All projects:**
```bash
todorust get projects
```

## Testing

Verify the skill works:

```bash
# Test todorust is accessible
which todorust

# Test basic queries
todorust get projects
todorust get tasks --filter "today"

# Test JSON parsing
todorust get tasks | jq '.'
```

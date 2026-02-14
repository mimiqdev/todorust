# Todoist Filter Syntax Reference

> **⚠️ CLI Support Note**: The `todorust` CLI currently supports simple keyword filtering on task content or project name via `--filter "keyword"`. The full Todoist filter syntax below is for your reference when viewing custom filters via `todorust get filters` or for manual use in the Todoist app.

## Date Filters

### Relative Dates

- `today` - Due today or completed today
- `tomorrow` - Due tomorrow
- `yesterday` - Due yesterday
- `completed today` - Completed today
- `completed within "7 days of today"` - Completed in last 7 days

### Date Ranges

```
due within "7 days of today"
due within "30 days of today"
completed within "7 days of today"
completed within "START_DATE to END_DATE"
```

**Format:** `YYYY-MM-DD`

Examples:
```
completed within "2026-01-10 to 2026-01-16"
due within "2026-01-15 to 2026-01-22"
```

### Specific Dates

```
due:2026-01-15
due before:2026-01-15
due after:2026-01-15
```

## Project Filters

```
project:ProjectName
project:Inbox
project:"Work Tasks"  # Use quotes for spaces
```

**Negation:**
```
!project:Inbox
no project
```

## Priority Filters

```
priority:4  # High (red)
priority:3  # Normal (yellow)
priority:2  # Low (blue)
priority:1  # None (gray)
```

**Negation:**
```
!priority:4
priority >= 3
```

## Label Filters

```
label:urgent
label:@work  # Labels with @ symbol
```

**Multiple labels:**
```
label:urgent & label:important
```

## Combining Filters

### AND (both conditions)

Use `&` or space:
```
project:Work & priority:4
project:Work & completed today
```

### OR (either condition)

Use `|`:
```
project:Work | project:Personal
priority:4 | urgent
```

### Negation

Use `!`:
```
!project:Inbox
!completed today
```

### Complex Examples

**High priority Work tasks completed this week:**
```
project:Work & priority:4 & completed within "7 days of today"
```

**All tasks except Inbox:**
```
!project:Inbox
```

**Urgent or high priority:**
```
label:urgent | priority:4
```

**Due today or overdue:**
```
due today | overdue
```

**Work or Personal projects, high priority:**
```
(project:Work | project:Personal) & priority:4
```

## Task State Filters

```
completed      # Shows all completed tasks
completed today
!completed     # Shows incomplete tasks
no due
overdue
```

## Common Patterns for Reports

### Daily Report
```
completed today
```

### Weekly Report
```
completed within "7 days of today"
```

### Custom Date Range
```
completed within "2026-01-10 to 2026-01-16"
```

### By Project This Week
```
project:Work & completed within "7 days of today"
```

### High Priority Completed
```
priority:4 & completed within "7 days of today"
```

### With Specific Label
```
label:@work & completed today
```

## Tips

1. **Use quotes** for filter values with spaces: `project:"Work Tasks"`
2. **Use parentheses** for complex logic: `(A | B) & C`
3. **Test filters** in Todoist web app first
4. **Case sensitive** for project names
5. **Date format** must be YYYY-MM-DD for ranges
6. **No limit** on filter complexity, but keep it readable

## More Resources

- Official Todoist filter syntax: https://todoist.com/help/articles/205248802
- Filter examples in Todoist app: https://todoist.com/help/articles/205248802

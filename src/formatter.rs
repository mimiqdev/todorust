/*!
 * # Output Formatting
 *
 * This module provides utilities for formatting task, project, and filter output
 * in various formats (JSON, Checklist, Structured).
 *
 * ## Supported Formats
 *
 * - **JSON**: Machine-readable JSON output
 * - **Checklist**: Simple checkbox format
 * - **Structured**: Hierarchical format with project groupings
 */

use crate::models::{Filter, Project, TaskOutput};
use crate::sync::{SyncFilter, SyncLabel, SyncSection};
use clap::ValueEnum;

#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Json,
    Checklist,
    Structured,
}

pub trait Formattable {
    fn format(&self, format: &OutputFormat) -> String;
}

impl Formattable for Vec<TaskOutput> {
    fn format(&self, format: &OutputFormat) -> String {
        match format {
            OutputFormat::Json => format_json(self),
            OutputFormat::Checklist => format_checklist(self),
            OutputFormat::Structured => format_structured(self),
        }
    }
}

fn format_json(tasks: &[TaskOutput]) -> String {
    serde_json::to_string_pretty(tasks).unwrap_or_default()
}

fn format_checklist(tasks: &[TaskOutput]) -> String {
    tasks
        .iter()
        .map(|task| {
            let checkbox = if task.is_completed { "[x]" } else { "[ ]" };
            if let Some(ref project) = task.project_name {
                format!("- {} {} ({})", checkbox, task.content, project)
            } else {
                format!("- {} {}", checkbox, task.content)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_structured(tasks: &[TaskOutput]) -> String {
    use std::collections::HashMap;

    let mut grouped: HashMap<&str, Vec<&TaskOutput>> = HashMap::new();

    for task in tasks {
        let project = task.project_name.as_deref().unwrap_or("Inbox");
        grouped.entry(project).or_default().push(task);
    }

    let mut projects: Vec<_> = grouped.into_iter().collect();
    projects.sort_by(|a, b| a.0.cmp(b.0));

    projects
        .iter()
        .map(|(project, tasks)| {
            let tasks_str = tasks
                .iter()
                .map(|task| {
                    let checkbox = if task.is_completed { "[x]" } else { "[ ]" };
                    if task.priority > 1 {
                        format!(
                            "- {} {} (Priority: {})",
                            checkbox, task.content, task.priority
                        )
                    } else {
                        format!("- {} {}", checkbox, task.content)
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            format!("## {}\n\n{}", project, tasks_str)
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

impl Formattable for Vec<Project> {
    fn format(&self, format: &OutputFormat) -> String {
        match format {
            OutputFormat::Json => format_json_projects(self),
            OutputFormat::Checklist => format_projects_checklist(self),
            OutputFormat::Structured => format_projects_structured(self),
        }
    }
}

fn format_json_projects(projects: &[Project]) -> String {
    serde_json::to_string_pretty(projects).unwrap_or_default()
}

fn format_projects_checklist(projects: &[Project]) -> String {
    projects
        .iter()
        .map(|project| {
            let indicator = if project.is_favorite { "⭐ " } else { "" };
            format!(
                "- [ ] {}{}{}",
                indicator,
                project.name,
                if project.is_shared { " (shared)" } else { "" }
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_projects_structured(projects: &[Project]) -> String {
    projects
        .iter()
        .map(|project| {
            let mut meta_parts = Vec::new();
            if project.is_favorite {
                meta_parts.push("⭐ Favorite");
            }
            if project.is_shared {
                meta_parts.push("👥 Shared");
            }
            let meta = if meta_parts.is_empty() {
                String::new()
            } else {
                format!("**Meta:** {} | ", meta_parts.join(" | "))
            };

            format!(
                "### {}\n\n**Color:** {}\n**ID:** {}\n{}",
                project.name, project.color, project.id, meta
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

impl Formattable for Vec<Filter> {
    fn format(&self, format: &OutputFormat) -> String {
        match format {
            OutputFormat::Json => format_json_filters(self),
            OutputFormat::Checklist => format_filters_checklist(self),
            OutputFormat::Structured => format_filters_structured(self),
        }
    }
}

fn format_json_filters(filters: &[Filter]) -> String {
    serde_json::to_string_pretty(filters).unwrap_or_default()
}

fn format_filters_checklist(filters: &[Filter]) -> String {
    filters
        .iter()
        .map(|filter| format!("- [ ] {} ({})", filter.name, filter.query))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_filters_structured(filters: &[Filter]) -> String {
    filters
        .iter()
        .map(|filter| {
            format!(
                "### {}\n\n**Filter:** `{}`\n**ID:** {}\n",
                filter.name, filter.query, filter.id
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

impl Formattable for Vec<SyncSection> {
    fn format(&self, format: &OutputFormat) -> String {
        match format {
            OutputFormat::Json => format_json_sections(self),
            OutputFormat::Checklist => format_sections_checklist(self),
            OutputFormat::Structured => format_sections_structured(self),
        }
    }
}

fn format_json_sections(sections: &[SyncSection]) -> String {
    serde_json::to_string_pretty(sections).unwrap_or_default()
}

fn format_sections_checklist(sections: &[SyncSection]) -> String {
    sections
        .iter()
        .map(|section| format!("- [ ] {} (Project: {})", section.name, section.project_id))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_sections_structured(sections: &[SyncSection]) -> String {
    use std::collections::HashMap;

    let mut grouped: HashMap<&str, Vec<&SyncSection>> = HashMap::new();

    for section in sections {
        grouped
            .entry(section.project_id.as_str())
            .or_default()
            .push(section);
    }

    let mut projects: Vec<_> = grouped.into_iter().collect();
    projects.sort_by(|a, b| a.0.cmp(b.0));

    projects
        .iter()
        .map(|(project_id, sections)| {
            let sections_str = sections
                .iter()
                .map(|s| format!("- [ ] {} (ID: {})", s.name, s.id))
                .collect::<Vec<_>>()
                .join("\n");
            format!("## Project: {}\n\n{}", project_id, sections_str)
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

impl Formattable for Vec<SyncFilter> {
    fn format(&self, format: &OutputFormat) -> String {
        match format {
            OutputFormat::Json => format_json_sync_filters(self),
            OutputFormat::Checklist => format_sync_filters_checklist(self),
            OutputFormat::Structured => format_sync_filters_structured(self),
        }
    }
}

fn format_json_sync_filters(filters: &[SyncFilter]) -> String {
    serde_json::to_string_pretty(filters).unwrap_or_default()
}

fn format_sync_filters_checklist(filters: &[SyncFilter]) -> String {
    filters
        .iter()
        .map(|filter| format!("- [ ] {} ({})", filter.name, filter.query))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_sync_filters_structured(filters: &[SyncFilter]) -> String {
    filters
        .iter()
        .map(|filter| {
            format!(
                "### {}\n\n**Filter:** `{}`\n**ID:** {}\n",
                filter.name, filter.query, filter.id
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

impl Formattable for Vec<SyncLabel> {
    fn format(&self, format: &OutputFormat) -> String {
        match format {
            OutputFormat::Json => format_json_sync_labels(self),
            OutputFormat::Checklist => format_sync_labels_checklist(self),
            OutputFormat::Structured => format_sync_labels_structured(self),
        }
    }
}

fn format_json_sync_labels(labels: &[SyncLabel]) -> String {
    serde_json::to_string_pretty(labels).unwrap_or_default()
}

fn format_sync_labels_checklist(labels: &[SyncLabel]) -> String {
    labels
        .iter()
        .map(|label| {
            let fav = if label.is_favorite { "⭐ " } else { "" };
            format!("- [ ] {}{} (Color: {})", fav, label.name, label.color)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_sync_labels_structured(labels: &[SyncLabel]) -> String {
    labels
        .iter()
        .map(|label| {
            let fav = if label.is_favorite {
                "**Favorite:** Yes\n"
            } else {
                ""
            };
            format!(
                "### {}\n\n**Color:** {}\n**ID:** {}\n{}",
                label.name, label.color, label.id, fav
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_tasks() -> Vec<TaskOutput> {
        vec![
            TaskOutput {
                id: "1".to_string(),
                content: "Task 1".to_string(),
                description: Some("Task 1 details".to_string()),
                project_name: Some("Work".to_string()),
                is_completed: true,
                priority: 4,
                labels: vec![],
                project_id: Some("p1".to_string()),
                due_date: None,
                created_at: "2026-01-10T10:00:00Z".to_string(),
                order: 1,
            },
            TaskOutput {
                id: "2".to_string(),
                content: "Task 2".to_string(),
                description: None,
                project_name: Some("Personal".to_string()),
                is_completed: false,
                priority: 2,
                labels: vec![],
                project_id: Some("p2".to_string()),
                due_date: None,
                created_at: "2026-01-11T10:00:00Z".to_string(),
                order: 2,
            },
        ]
    }

    #[test]
    fn test_format_checklist() {
        let tasks = mock_tasks();
        let output = tasks.format(&OutputFormat::Checklist);
        assert!(output.contains("- [x] Task 1 (Work)"));
        assert!(output.contains("- [ ] Task 2 (Personal)"));
    }

    #[test]
    fn test_format_structured() {
        let tasks = mock_tasks();
        let output = tasks.format(&OutputFormat::Structured);
        assert!(output.contains("## Personal"));
        assert!(output.contains("## Work"));
        assert!(output.contains("- [x] Task 1"));
        assert!(output.contains("(Priority: 4)"));
        assert!(output.contains("- [ ] Task 2"));
        assert!(output.contains("(Priority: 2)"));
    }

    #[test]
    fn test_format_json() {
        let tasks = mock_tasks();
        let output = tasks.format(&OutputFormat::Json);
        assert!(output.contains("\"content\""));
        assert!(output.contains("Task 1"));
        assert!(output.contains("Task 2"));
    }

    #[test]
    fn test_format_empty_tasks() {
        let tasks: Vec<TaskOutput> = vec![];
        let output = tasks.format(&OutputFormat::Checklist);
        assert_eq!(output, "");
    }

    #[test]
    fn test_format_checklist_no_project() {
        let tasks = vec![TaskOutput {
            id: "1".to_string(),
            content: "No Project Task".to_string(),
            description: None,
            project_name: None,
            is_completed: true,
            priority: 1,
            labels: vec![],
            project_id: None,
            due_date: None,
            created_at: "2026-01-10T10:00:00Z".to_string(),
            order: 1,
        }];
        let output = tasks.format(&OutputFormat::Checklist);
        assert!(output.contains("- [x] No Project Task"));
        assert!(!output.contains("("));
    }

    #[test]
    fn test_format_structured_inbox() {
        let tasks = vec![TaskOutput {
            id: "1".to_string(),
            content: "Inbox Task".to_string(),
            description: None,
            project_name: None,
            is_completed: false,
            priority: 1,
            labels: vec![],
            project_id: None,
            due_date: None,
            created_at: "2026-01-10T10:00:00Z".to_string(),
            order: 1,
        }];
        let output = tasks.format(&OutputFormat::Structured);
        assert!(output.contains("## Inbox"));
        assert!(output.contains("- [ ] Inbox Task"));
        // Priority 1 should not show
        assert!(!output.contains("(Priority:"));
    }

    #[test]
    fn test_format_projects_checklist() {
        let projects = vec![
            Project {
                id: "1".to_string(),
                name: "Work".to_string(),
                color: "blue".to_string(),
                is_shared: false,
                is_favorite: true,
            },
            Project {
                id: "2".to_string(),
                name: "Personal".to_string(),
                color: "green".to_string(),
                is_shared: true,
                is_favorite: false,
            },
        ];

        let output = projects.format(&OutputFormat::Checklist);
        assert!(output.contains("- [ ] ⭐ Work"));
        assert!(output.contains("- [ ] Personal (shared)"));
    }

    #[test]
    fn test_format_projects_structured() {
        let projects = vec![Project {
            id: "1".to_string(),
            name: "Work".to_string(),
            color: "blue".to_string(),
            is_shared: false,
            is_favorite: true,
        }];

        let output = projects.format(&OutputFormat::Structured);
        assert!(output.contains("### Work"));
        assert!(output.contains("**Color:** blue"));
        assert!(output.contains("⭐ Favorite"));
    }

    #[test]
    fn test_format_projects_json() {
        let projects = vec![Project {
            id: "1".to_string(),
            name: "Test".to_string(),
            color: "red".to_string(),
            is_shared: false,
            is_favorite: false,
        }];

        let output = projects.format(&OutputFormat::Json);
        assert!(output.contains("\"name\""));
        assert!(output.contains("Test"));
    }

    #[test]
    fn test_format_filters_checklist() {
        let filters = vec![
            Filter {
                id: "1".to_string(),
                name: "This Week".to_string(),
                query: "due within \"7 days of today\"".to_string(),
            },
            Filter {
                id: "2".to_string(),
                name: "Work High Priority".to_string(),
                query: "project:Work & priority:4".to_string(),
            },
        ];

        let output = filters.format(&OutputFormat::Checklist);
        assert!(output.contains("- [ ] This Week"));
        assert!(output.contains("(due within \"7 days of today\")"));
    }

    #[test]
    fn test_format_filters_structured() {
        let filters = vec![Filter {
            id: "1".to_string(),
            name: "Urgent Tasks".to_string(),
            query: "priority:4".to_string(),
        }];

        let output = filters.format(&OutputFormat::Structured);
        assert!(output.contains("### Urgent Tasks"));
        assert!(output.contains("**Filter:** `priority:4`"));
        assert!(output.contains("**ID:** 1"));
    }

    #[test]
    fn test_format_filters_json() {
        let filters = vec![Filter {
            id: "1".to_string(),
            name: "Test".to_string(),
            query: "project:Work".to_string(),
        }];

        let output = filters.format(&OutputFormat::Json);
        assert!(output.contains("\"name\""));
        assert!(output.contains("Test"));
    }

    // Tests for section formatting
    fn mock_sections() -> Vec<SyncSection> {
        vec![
            SyncSection {
                id: "s1".to_string(),
                project_id: "p1".to_string(),
                name: "Section 1".to_string(),
                order: 1,
                is_archived: false,
                is_deleted: false,
                created_at: "2026-01-10T10:00:00Z".to_string(),
                archived_at: None,
                is_collapsed: None,
            },
            SyncSection {
                id: "s2".to_string(),
                project_id: "p1".to_string(),
                name: "Section 2".to_string(),
                order: 2,
                is_archived: false,
                is_deleted: false,
                created_at: "2026-01-11T10:00:00Z".to_string(),
                archived_at: None,
                is_collapsed: None,
            },
            SyncSection {
                id: "s3".to_string(),
                project_id: "p2".to_string(),
                name: "Section 3".to_string(),
                order: 1,
                is_archived: false,
                is_deleted: false,
                created_at: "2026-01-12T10:00:00Z".to_string(),
                archived_at: None,
                is_collapsed: None,
            },
        ]
    }

    #[test]
    fn test_format_sections_json() {
        let sections = mock_sections();
        let output = sections.format(&OutputFormat::Json);
        assert!(output.contains("Section 1"));
        assert!(output.contains("Section 2"));
        assert!(output.contains("Section 3"));
    }

    #[test]
    fn test_format_sections_checklist() {
        let sections = mock_sections();
        let output = sections.format(&OutputFormat::Checklist);
        assert!(output.contains("- [ ] Section 1 (Project: p1)"));
        assert!(output.contains("- [ ] Section 2 (Project: p1)"));
        assert!(output.contains("- [ ] Section 3 (Project: p2)"));
    }

    #[test]
    fn test_format_sections_structured() {
        let sections = mock_sections();
        let output = sections.format(&OutputFormat::Structured);
        assert!(output.contains("## Project: p1"));
        assert!(output.contains("## Project: p2"));
        assert!(output.contains("Section 1"));
        assert!(output.contains("Section 3"));
    }

    #[test]
    fn test_format_empty_sections() {
        let sections: Vec<SyncSection> = vec![];
        let output = sections.format(&OutputFormat::Checklist);
        assert_eq!(output, "");
    }
}

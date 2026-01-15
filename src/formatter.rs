use clap::ValueEnum;
use crate::models::{TaskOutput, Project};

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
    tasks.iter()
        .map(|task| {
            let checkbox = if task.is_completed { "[x]" } else { "[ ]" };
            let project = task.project_name.as_ref()
                .map(|p| format!(" ({})", p))
                .unwrap_or_default();
            format!("- {} {}{}", checkbox, task.content, project)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_structured(tasks: &[TaskOutput]) -> String {
    let mut grouped: std::collections::HashMap<&str, Vec<&TaskOutput>> =
        std::collections::HashMap::new();

    for task in tasks {
        let project = task.project_name.as_deref().unwrap_or("Inbox");
        grouped.entry(project).or_insert_with(Vec::new).push(task);
    }

    let mut projects: Vec<_> = grouped.into_iter().collect();
    projects.sort_by(|a, b| a.0.cmp(&b.0));

    projects.iter()
        .map(|(project, tasks)| {
            let tasks_str = tasks.iter()
                .map(|task| {
                    let checkbox = if task.is_completed { "[x]" } else { "[ ]" };
                    let priority = if task.priority > 1 {
                        format!(" (Priority: {})", task.priority)
                    } else {
                        String::new()
                    };
                    format!("- {} {}{}", checkbox, task.content, priority)
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
    projects.iter()
        .map(|project| {
            let indicator = if project.is_favorite { "⭐ " } else { "" };
            format!("- [ ] {}{}{}", indicator, project.name,
                    if project.is_shared { " (shared)" } else { "" })
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_projects_structured(projects: &[Project]) -> String {
    projects.iter()
        .map(|project| {
            let meta = vec![
                if project.is_favorite { Some("⭐ Favorite".to_string()) } else { None },
                if project.is_shared { Some("👥 Shared".to_string()) } else { None },
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(" | ");

            format!(
                "### {}\n\n**Color:** {}\n**ID:** {}\n{}",
                project.name,
                project.color,
                project.id,
                if meta.is_empty() { String::new() } else { format!("**Meta:** {}\n", meta) }
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
}

/*!
 * # Sync API Commands
 *
 * This module provides the command structures and builder for the Todoist Sync API.
 *
 * ## Commands
 *
 * Commands are used to perform write operations through the Sync API.
 * The [`CommandBuilder`] provides a fluent API for constructing commands.
 *
 * ## Example
 *
 * ```ignore
 * use todorust::sync::commands::{CommandBuilder, ItemAddArgs};
 *
 * let mut builder = CommandBuilder::new();
 * builder
 *     .item_add(ItemAddArgs::new("Buy milk".to_string()))
 *     .item_add(ItemAddArgs::new("Clean house".to_string()));
 *
 * let commands = builder.build();
 * ```
 */

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::TodoError;

/// Sync API 命令结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default = "Command::generate_uuid")]
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_id: Option<String>,
    pub args: serde_json::Value,
}

impl Command {
    /// 生成命令 UUID
    pub fn generate_uuid() -> String {
        Uuid::new_v4().to_string()
    }

    /// 生成临时 ID
    pub fn generate_temp_id() -> String {
        Uuid::new_v4().to_string()
    }
}

/// Command 构建器
pub struct CommandBuilder {
    commands: Vec<Command>,
}

impl CommandBuilder {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// 添加 item_add 命令 - 创建任务
    pub fn item_add(mut self, args: ItemAddArgs) -> Self {
        self.commands.push(Command {
            type_: "item_add".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: Some(Command::generate_temp_id()),
            args: serde_json::to_value(args)
                .map_err(|e| TodoError::Serialize(e.to_string()))
                .unwrap(),
        });
        self
    }

    /// 添加 item_close 命令 - 完成任务
    pub fn item_close(mut self, id: &str) -> Self {
        self.commands.push(Command {
            type_: "item_close".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    /// 添加 item_reopen 命令 - 重新打开任务
    pub fn item_reopen(mut self, id: &str) -> Self {
        self.commands.push(Command {
            type_: "item_reopen".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    /// 添加 item_delete 命令 - 删除任务
    pub fn item_delete(mut self, id: &str) -> Self {
        self.commands.push(Command {
            type_: "item_delete".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    /// 添加 item_move 命令 - 移动任务
    pub fn item_move(mut self, id: &str, project_id: &str, section_id: Option<&str>) -> Self {
        let mut args = serde_json::json!({
            "id": id,
            "project_id": project_id
        });
        if let Some(sid) = section_id {
            args["section_id"] = serde_json::json!(sid);
        }
        self.commands.push(Command {
            type_: "item_move".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args,
        });
        self
    }

    /// 添加 project_add 命令 - 创建项目
    pub fn project_add(mut self, args: ProjectAddArgs) -> Self {
        self.commands.push(Command {
            type_: "project_add".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: Some(Command::generate_temp_id()),
            args: serde_json::to_value(args)
                .map_err(|e| TodoError::Serialize(e.to_string()))
                .unwrap(),
        });
        self
    }

    /// 添加 project_update 命令 - 更新项目
    pub fn project_update(
        mut self,
        id: &str,
        name: Option<&str>,
        color: Option<&str>,
        favorite: Option<bool>,
    ) -> Self {
        let mut args = serde_json::json!({ "id": id });
        if let Some(n) = name {
            args["name"] = serde_json::json!(n);
        }
        if let Some(c) = color {
            args["color"] = serde_json::json!(c);
        }
        if let Some(f) = favorite {
            args["favorite"] = serde_json::json!(f);
        }
        self.commands.push(Command {
            type_: "project_update".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args,
        });
        self
    }

    /// 添加 project_delete 命令 - 删除项目
    pub fn project_delete(mut self, id: &str) -> Self {
        self.commands.push(Command {
            type_: "project_delete".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    /// 添加 section_add 命令 - 创建分区
    pub fn section_add(mut self, args: SectionAddArgs) -> Self {
        self.commands.push(Command {
            type_: "section_add".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: Some(Command::generate_temp_id()),
            args: serde_json::to_value(args)
                .map_err(|e| TodoError::Serialize(e.to_string()))
                .unwrap(),
        });
        self
    }

    /// 添加 section_update 命令 - 更新分区
    pub fn section_update(mut self, id: &str, name: &str) -> Self {
        self.commands.push(Command {
            type_: "section_update".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "id": id, "name": name }),
        });
        self
    }

    /// 添加 section_delete 命令 - 删除分区
    pub fn section_delete(mut self, id: &str) -> Self {
        self.commands.push(Command {
            type_: "section_delete".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    /// 添加 section_archive 命令 - 归档分区
    pub fn section_archive(mut self, id: &str) -> Self {
        self.commands.push(Command {
            type_: "section_archive".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    /// 添加 section_unarchive 命令 - 取消归档分区
    pub fn section_unarchive(mut self, id: &str) -> Self {
        self.commands.push(Command {
            type_: "section_unarchive".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    /// 添加 section_move 命令 - 移动分区到项目
    pub fn section_move(mut self, id: &str, project_id: &str) -> Self {
        self.commands.push(Command {
            type_: "section_move".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({
                "id": id,
                "project_id": project_id
            }),
        });
        self
    }

    /// 添加 section_reorder 命令 - 批量重新排序分区
    pub fn section_reorder(mut self, sections: &[SectionOrderArgs]) -> Self {
        self.commands.push(Command {
            type_: "section_reorder".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "sections": sections }),
        });
        self
    }

    /// 添加 item_complete 命令 - 完成任务（标记为已完成）
    pub fn item_complete(mut self, id: &str) -> Self {
        self.commands.push(Command {
            type_: "item_complete".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    /// 添加 item_update 命令 - 更新任务
    pub fn item_update(mut self, args: ItemUpdateArgs) -> Self {
        self.commands.push(Command {
            type_: "item_update".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args: serde_json::to_value(args)
                .map_err(|e| TodoError::Serialize(e.to_string()))
                .unwrap(),
        });
        self
    }

    /// 添加 label_add 命令 - 创建标签
    pub fn label_add(mut self, args: LabelAddArgs) -> Self {
        self.commands.push(Command {
            type_: "label_add".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: Some(Command::generate_temp_id()),
            args: serde_json::to_value(args)
                .map_err(|e| TodoError::Serialize(e.to_string()))
                .unwrap(),
        });
        self
    }

    /// 添加 label_update 命令 - 更新标签
    pub fn label_update(mut self, id: &str, name: Option<&str>, color: Option<&str>) -> Self {
        let mut args = serde_json::json!({ "id": id });
        if let Some(n) = name {
            args["name"] = serde_json::json!(n);
        }
        if let Some(c) = color {
            args["color"] = serde_json::json!(c);
        }
        self.commands.push(Command {
            type_: "label_update".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args,
        });
        self
    }

    /// 添加 label_delete 命令 - 删除标签
    pub fn label_delete(mut self, id: &str) -> Self {
        self.commands.push(Command {
            type_: "label_delete".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    /// 添加 filter_update_orders 命令 - 更新过滤器顺序
    pub fn filter_update_orders(mut self, filters: &[FilterOrderArgs]) -> Self {
        self.commands.push(Command {
            type_: "filter_update_orders".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "filters": filters }),
        });
        self
    }

    /// 添加 filter_add 命令 - 创建过滤器
    pub fn filter_add(mut self, args: FilterAddArgs) -> Self {
        self.commands.push(Command {
            type_: "filter_add".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: Some(Command::generate_temp_id()),
            args: serde_json::to_value(args)
                .map_err(|e| TodoError::Serialize(e.to_string()))
                .unwrap(),
        });
        self
    }

    /// 添加 filter_update 命令 - 更新过滤器
    pub fn filter_update(
        mut self,
        id: &str,
        name: Option<&str>,
        query: Option<&str>,
        color: Option<&str>,
    ) -> Self {
        let mut args = serde_json::json!({ "id": id });
        if let Some(n) = name {
            args["name"] = serde_json::json!(n);
        }
        if let Some(q) = query {
            args["query"] = serde_json::json!(q);
        }
        if let Some(c) = color {
            args["color"] = serde_json::json!(c);
        }
        self.commands.push(Command {
            type_: "filter_update".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args,
        });
        self
    }

    /// 添加 filter_delete 命令 - 删除过滤器
    pub fn filter_delete(mut self, id: &str) -> Self {
        self.commands.push(Command {
            type_: "filter_delete".to_string(),
            uuid: Command::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    /// 构建命令列表
    pub fn build(self) -> Vec<Command> {
        self.commands
    }
}

impl Default for CommandBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// item_add 命令参数
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemAddArgs {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
}

impl ItemAddArgs {
    pub fn new(content: String) -> Self {
        Self {
            content,
            description: None,
            project_id: None,
            section_id: None,
            due_string: None,
            priority: None,
            labels: None,
        }
    }

    pub fn description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    pub fn project_id(mut self, project_id: Option<String>) -> Self {
        self.project_id = project_id;
        self
    }

    pub fn section_id(mut self, section_id: Option<String>) -> Self {
        self.section_id = section_id;
        self
    }

    pub fn due_string(mut self, due_string: Option<String>) -> Self {
        self.due_string = due_string;
        self
    }

    pub fn priority(mut self, priority: Option<u8>) -> Self {
        self.priority = priority;
        self
    }

    pub fn labels(mut self, labels: Option<Vec<String>>) -> Self {
        self.labels = labels;
        self
    }
}

/// project_add 命令参数
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectAddArgs {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,
}

impl ProjectAddArgs {
    pub fn new(name: String) -> Self {
        Self {
            name,
            color: None,
            favorite: None,
        }
    }

    pub fn color(mut self, color: Option<String>) -> Self {
        self.color = color;
        self
    }

    pub fn favorite(mut self, favorite: Option<bool>) -> Self {
        self.favorite = favorite;
        self
    }
}

/// section_add 命令参数
#[derive(Debug, Serialize, Deserialize)]
pub struct SectionAddArgs {
    pub name: String,
    pub project_id: String,
}

impl SectionAddArgs {
    pub fn new(name: String, project_id: String) -> Self {
        Self { name, project_id }
    }
}

/// item_update 命令参数
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemUpdateArgs {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_datetime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
}

impl ItemUpdateArgs {
    pub fn new(id: String) -> Self {
        Self {
            id,
            content: None,
            description: None,
            priority: None,
            due_string: None,
            due_datetime: None,
            due_lang: None,
            labels: None,
        }
    }

    pub fn content(mut self, content: Option<String>) -> Self {
        self.content = content;
        self
    }

    pub fn description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    pub fn priority(mut self, priority: Option<u8>) -> Self {
        self.priority = priority;
        self
    }

    pub fn due_string(mut self, due_string: Option<String>) -> Self {
        self.due_string = due_string;
        self
    }

    pub fn due_datetime(mut self, due_datetime: Option<String>) -> Self {
        self.due_datetime = due_datetime;
        self
    }

    pub fn due_lang(mut self, due_lang: Option<String>) -> Self {
        self.due_lang = due_lang;
        self
    }

    pub fn labels(mut self, labels: Option<Vec<String>>) -> Self {
        self.labels = labels;
        self
    }
}

/// label_add 命令参数
#[derive(Debug, Serialize, Deserialize)]
pub struct LabelAddArgs {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl LabelAddArgs {
    pub fn new(name: String) -> Self {
        Self { name, color: None }
    }

    pub fn color(mut self, color: Option<String>) -> Self {
        self.color = color;
        self
    }
}

/// filter_update_orders 命令参数
#[derive(Debug, Serialize, Deserialize)]
pub struct FilterOrderArgs {
    pub id: String,
    pub order: i64,
}

impl FilterOrderArgs {
    pub fn new(id: String, order: i64) -> Self {
        Self { id, order }
    }
}

/// filter_add 命令参数
#[derive(Debug, Serialize, Deserialize)]
pub struct FilterAddArgs {
    pub name: String,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl FilterAddArgs {
    pub fn new(name: String, query: String) -> Self {
        Self {
            name,
            query,
            color: None,
        }
    }

    pub fn color(mut self, color: Option<String>) -> Self {
        self.color = color;
        self
    }
}

/// section_reorder 命令参数
#[derive(Debug, Serialize, Deserialize)]
pub struct SectionOrderArgs {
    pub id: String,
    pub order: i64,
}

impl SectionOrderArgs {
    pub fn new(id: String, order: i64) -> Self {
        Self { id, order }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_builder_new() {
        let builder = CommandBuilder::new();
        assert!(builder.commands.is_empty());
    }

    #[test]
    fn test_item_add_command() {
        let commands = CommandBuilder::new()
            .item_add(ItemAddArgs::new("Test task".to_string()))
            .build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "item_add");
        assert!(!cmd.uuid.is_empty());
        assert!(cmd.temp_id.is_some());
    }

    #[test]
    fn test_item_close_command() {
        let commands = CommandBuilder::new().item_close("123").build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "item_close");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_multiple_commands() {
        let commands = CommandBuilder::new()
            .item_add(ItemAddArgs::new("Task 1".to_string()))
            .item_add(ItemAddArgs::new("Task 2".to_string()))
            .item_close("456")
            .build();
        assert_eq!(commands.len(), 3);
    }

    #[test]
    fn test_uuid_uniqueness() {
        let commands = CommandBuilder::new()
            .item_add(ItemAddArgs::new("Task 1".to_string()))
            .item_add(ItemAddArgs::new("Task 2".to_string()))
            .build();
        assert_ne!(commands[0].uuid, commands[1].uuid);
    }

    #[test]
    fn test_temp_id_uniqueness() {
        let commands = CommandBuilder::new()
            .item_add(ItemAddArgs::new("Task 1".to_string()))
            .item_add(ItemAddArgs::new("Task 2".to_string()))
            .build();
        assert_ne!(commands[0].temp_id, commands[1].temp_id);
    }

    #[test]
    fn test_item_complete_command() {
        let commands = CommandBuilder::new().item_complete("123").build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "item_complete");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_item_update_command() {
        let commands = CommandBuilder::new()
            .item_update(
                ItemUpdateArgs::new("123".to_string()).content(Some("Updated".to_string())),
            )
            .build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "item_update");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_label_add_command() {
        let commands = CommandBuilder::new()
            .label_add(LabelAddArgs::new("urgent".to_string()).color(Some("red".to_string())))
            .build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "label_add");
        assert!(cmd.temp_id.is_some());
    }

    #[test]
    fn test_label_delete_command() {
        let commands = CommandBuilder::new().label_delete("456").build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "label_delete");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_filter_update_orders_command() {
        let commands = CommandBuilder::new()
            .filter_update_orders(&[
                FilterOrderArgs::new("123".to_string(), 1),
                FilterOrderArgs::new("456".to_string(), 2),
            ])
            .build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "filter_update_orders");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_section_archive_command() {
        let commands = CommandBuilder::new().section_archive("123").build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "section_archive");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_section_unarchive_command() {
        let commands = CommandBuilder::new().section_unarchive("123").build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "section_unarchive");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_section_move_command() {
        let commands = CommandBuilder::new().section_move("123", "456").build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "section_move");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_section_reorder_command() {
        let commands = CommandBuilder::new()
            .section_reorder(&[
                SectionOrderArgs::new("123".to_string(), 1),
                SectionOrderArgs::new("456".to_string(), 2),
            ])
            .build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "section_reorder");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_project_add_command() {
        let commands = CommandBuilder::new()
            .project_add(ProjectAddArgs::new("New Project".to_string()))
            .build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "project_add");
        assert!(cmd.temp_id.is_some());
    }

    #[test]
    fn test_project_add_with_color_and_favorite() {
        let commands = CommandBuilder::new()
            .project_add(
                ProjectAddArgs::new("My Project".to_string())
                    .color(Some("blue".to_string()))
                    .favorite(Some(true)),
            )
            .build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "project_add");
        assert!(cmd.temp_id.is_some());
    }

    #[test]
    fn test_project_update_command() {
        let commands = CommandBuilder::new()
            .project_update("123", Some("Updated"), Some("red"), Some(false))
            .build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "project_update");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_project_delete_command() {
        let commands = CommandBuilder::new().project_delete("123").build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "project_delete");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_section_add_command() {
        let commands = CommandBuilder::new()
            .section_add(SectionAddArgs::new(
                "New Section".to_string(),
                "456".to_string(),
            ))
            .build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "section_add");
        assert!(cmd.temp_id.is_some());
    }

    #[test]
    fn test_section_update_command() {
        let commands = CommandBuilder::new()
            .section_update("123", "Updated Section")
            .build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "section_update");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_section_delete_command() {
        let commands = CommandBuilder::new().section_delete("123").build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "section_delete");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_item_reopen_command() {
        let commands = CommandBuilder::new().item_reopen("123").build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "item_reopen");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_item_move_command() {
        let commands = CommandBuilder::new().item_move("123", "456", None).build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "item_move");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_item_move_with_section() {
        let commands = CommandBuilder::new()
            .item_move("123", "456", Some("789"))
            .build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "item_move");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_item_delete_command() {
        let commands = CommandBuilder::new().item_delete("123").build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "item_delete");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_filter_add_command() {
        let commands = CommandBuilder::new()
            .filter_add(FilterAddArgs::new(
                "My Filter".to_string(),
                "today".to_string(),
            ))
            .build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "filter_add");
        assert!(cmd.temp_id.is_some());
    }

    #[test]
    fn test_filter_add_with_color() {
        let commands = CommandBuilder::new()
            .filter_add(
                FilterAddArgs::new("My Filter".to_string(), "overdue".to_string())
                    .color(Some("green".to_string())),
            )
            .build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "filter_add");
        assert!(cmd.temp_id.is_some());
    }

    #[test]
    fn test_filter_update_command() {
        let commands = CommandBuilder::new()
            .filter_update(
                "123",
                Some("Updated Filter"),
                Some("today | overdue"),
                Some("blue"),
            )
            .build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "filter_update");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_filter_delete_command() {
        let commands = CommandBuilder::new().filter_delete("123").build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "filter_delete");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_command_builder_default() {
        let builder = CommandBuilder::default();
        assert!(builder.build().is_empty());
    }

    #[test]
    fn test_item_add_args_builder_pattern() {
        let args = ItemAddArgs::new("New Task".to_string())
            .description(Some("Task description".to_string()))
            .project_id(Some("123".to_string()))
            .section_id(Some("456".to_string()))
            .due_string(Some("tomorrow".to_string()))
            .priority(Some(4))
            .labels(Some(vec!["label1".to_string(), "label2".to_string()]));

        let commands = CommandBuilder::new().item_add(args).build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "item_add");
        assert!(cmd.temp_id.is_some());
    }

    #[test]
    fn test_item_update_args_builder_pattern() {
        let args = ItemUpdateArgs::new("123".to_string())
            .content(Some("Updated content".to_string()))
            .description(Some("Updated description".to_string()))
            .priority(Some(3))
            .due_string(Some("next week".to_string()))
            .due_datetime(Some("2024-01-15T10:00:00".to_string()))
            .due_lang(Some("en".to_string()))
            .labels(Some(vec!["tag1".to_string()]));

        let commands = CommandBuilder::new().item_update(args).build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "item_update");
    }

    #[test]
    fn test_label_add_args_builder_pattern() {
        let args = LabelAddArgs::new("new_label".to_string()).color(Some("yellow".to_string()));
        let commands = CommandBuilder::new().label_add(args).build();
        assert_eq!(commands.len(), 1);
    }

    #[test]
    fn test_filter_add_args_builder_pattern() {
        let args = FilterAddArgs::new("Filter Name".to_string(), "p1 & @work".to_string())
            .color(Some("purple".to_string()));
        let commands = CommandBuilder::new().filter_add(args).build();
        assert_eq!(commands.len(), 1);
    }

    #[test]
    fn test_section_order_args() {
        let args = SectionOrderArgs::new("123".to_string(), 5);
        assert_eq!(args.id, "123");
        assert_eq!(args.order, 5);
    }

    #[test]
    fn test_filter_order_args() {
        let args = FilterOrderArgs::new("456".to_string(), 10);
        assert_eq!(args.id, "456");
        assert_eq!(args.order, 10);
    }

    #[test]
    fn test_command_serialize_to_json() {
        let commands = CommandBuilder::new()
            .item_add(ItemAddArgs::new("Test".to_string()))
            .build();

        let cmd = &commands[0];
        let json = serde_json::to_string(cmd).unwrap();
        assert!(json.contains("type"));
        assert!(json.contains("uuid"));
        assert!(json.contains("args"));
    }

    #[test]
    fn test_command_type_field() {
        let item_add = CommandBuilder::new()
            .item_add(ItemAddArgs::new("Task".to_string()))
            .build();
        assert_eq!(item_add[0].type_, "item_add");

        let item_close = CommandBuilder::new().item_close("123").build();
        assert_eq!(item_close[0].type_, "item_close");

        let project_add = CommandBuilder::new()
            .project_add(ProjectAddArgs::new("Project".to_string()))
            .build();
        assert_eq!(project_add[0].type_, "project_add");
    }

    #[test]
    fn test_empty_builder_build() {
        let commands = CommandBuilder::new().build();
        assert!(commands.is_empty());
    }

    #[test]
    fn test_command_deserialization() {
        let json = r#"{
            "type": "item_add",
            "args": {"content": "New Task"}
        }"#;
        let cmd: Command = serde_json::from_str(json).unwrap();
        assert_eq!(cmd.type_, "item_add");
        assert_eq!(cmd.args["content"], "New Task");
        // Check that UUID was generated by default
        assert!(!cmd.uuid.is_empty());
    }

    #[test]
    fn test_batch_deserialization() {
        let json = r#"[
            {"type": "item_add", "args": {"content": "Task 1"}},
            {"type": "item_complete", "args": {"id": "123"}}
        ]"#;
        let commands: Vec<Command> = serde_json::from_str(json).unwrap();
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].type_, "item_add");
        assert_eq!(commands[1].type_, "item_complete");
    }
}

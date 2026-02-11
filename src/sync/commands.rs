use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Sync API 命令结构
#[derive(Debug, Serialize)]
pub struct Command {
    #[serde(rename = "type")]
    pub type_: String,
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_id: Option<String>,
    pub args: serde_json::Value,
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

    /// 生成命令 UUID
    fn generate_uuid() -> String {
        Uuid::new_v4().to_string()
    }

    /// 生成临时 ID
    fn generate_temp_id() -> String {
        Uuid::new_v4().to_string()
    }

    /// 添加 item_add 命令 - 创建任务
    pub fn item_add(&mut self, args: ItemAddArgs) -> &mut Self {
        self.commands.push(Command {
            type_: "item_add".to_string(),
            uuid: Self::generate_uuid(),
            temp_id: Some(Self::generate_temp_id()),
            args: serde_json::to_value(args).unwrap(),
        });
        self
    }

    /// 添加 item_close 命令 - 完成任务
    pub fn item_close(&mut self, id: &str) -> &mut Self {
        self.commands.push(Command {
            type_: "item_close".to_string(),
            uuid: Self::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    /// 添加 item_reopen 命令 - 重新打开任务
    pub fn item_reopen(&mut self, id: &str) -> &mut Self {
        self.commands.push(Command {
            type_: "item_reopen".to_string(),
            uuid: Self::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    /// 添加 item_delete 命令 - 删除任务
    pub fn item_delete(&mut self, id: &str) -> &mut Self {
        self.commands.push(Command {
            type_: "item_delete".to_string(),
            uuid: Self::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    /// 添加 item_move 命令 - 移动任务
    pub fn item_move(&mut self, id: &str, project_id: &str, section_id: Option<&str>) -> &mut Self {
        let mut args = serde_json::json!({
            "id": id,
            "project_id": project_id
        });
        if let Some(sid) = section_id {
            args["section_id"] = serde_json::json!(sid);
        }
        self.commands.push(Command {
            type_: "item_move".to_string(),
            uuid: Self::generate_uuid(),
            temp_id: None,
            args,
        });
        self
    }

    /// 添加 project_add 命令 - 创建项目
    pub fn project_add(&mut self, args: ProjectAddArgs) -> &mut Self {
        self.commands.push(Command {
            type_: "project_add".to_string(),
            uuid: Self::generate_uuid(),
            temp_id: Some(Self::generate_temp_id()),
            args: serde_json::to_value(args).unwrap(),
        });
        self
    }

    /// 添加 project_update 命令 - 更新项目
    pub fn project_update(&mut self, id: &str, name: Option<&str>, color: Option<&str>, favorite: Option<bool>) -> &mut Self {
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
            uuid: Self::generate_uuid(),
            temp_id: None,
            args,
        });
        self
    }

    /// 添加 project_delete 命令 - 删除项目
    pub fn project_delete(&mut self, id: &str) -> &mut Self {
        self.commands.push(Command {
            type_: "project_delete".to_string(),
            uuid: Self::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "id": id }),
        });
        self
    }

    /// 添加 section_add 命令 - 创建分区
    pub fn section_add(&mut self, args: SectionAddArgs) -> &mut Self {
        self.commands.push(Command {
            type_: "section_add".to_string(),
            uuid: Self::generate_uuid(),
            temp_id: Some(Self::generate_temp_id()),
            args: serde_json::to_value(args).unwrap(),
        });
        self
    }

    /// 添加 section_update 命令 - 更新分区
    pub fn section_update(&mut self, id: &str, name: &str) -> &mut Self {
        self.commands.push(Command {
            type_: "section_update".to_string(),
            uuid: Self::generate_uuid(),
            temp_id: None,
            args: serde_json::json!({ "id": id, "name": name }),
        });
        self
    }

    /// 添加 section_delete 命令 - 删除分区
    pub fn section_delete(&mut self, id: &str) -> &mut Self {
        self.commands.push(Command {
            type_: "section_delete".to_string(),
            uuid: Self::generate_uuid(),
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
#[derive(Debug, Serialize)]
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
#[derive(Debug, Serialize)]
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
#[derive(Debug, Serialize)]
pub struct SectionAddArgs {
    pub name: String,
    pub project_id: String,
}

impl SectionAddArgs {
    pub fn new(name: String, project_id: String) -> Self {
        Self { name, project_id }
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
        let mut builder = CommandBuilder::new();
        builder.item_add(ItemAddArgs::new("Test task".to_string()));

        let commands = builder.build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "item_add");
        assert!(cmd.uuid.is_empty() == false);
        assert!(cmd.temp_id.is_some());
    }

    #[test]
    fn test_item_close_command() {
        let mut builder = CommandBuilder::new();
        builder.item_close("123");

        let commands = builder.build();
        assert_eq!(commands.len(), 1);

        let cmd = &commands[0];
        assert_eq!(cmd.type_, "item_close");
        assert!(cmd.temp_id.is_none());
    }

    #[test]
    fn test_multiple_commands() {
        let mut builder = CommandBuilder::new();
        builder
            .item_add(ItemAddArgs::new("Task 1".to_string()))
            .item_add(ItemAddArgs::new("Task 2".to_string()))
            .item_close("456");

        let commands = builder.build();
        assert_eq!(commands.len(), 3);
    }

    #[test]
    fn test_uuid_uniqueness() {
        let mut builder = CommandBuilder::new();
        builder
            .item_add(ItemAddArgs::new("Task 1".to_string()))
            .item_add(ItemAddArgs::new("Task 2".to_string()));

        let commands = builder.build();
        assert_ne!(commands[0].uuid, commands[1].uuid);
    }

    #[test]
    fn test_temp_id_uniqueness() {
        let mut builder = CommandBuilder::new();
        builder
            .item_add(ItemAddArgs::new("Task 1".to_string()))
            .item_add(ItemAddArgs::new("Task 2".to_string()));

        let commands = builder.build();
        assert_ne!(commands[0].temp_id, commands[1].temp_id);
    }
}

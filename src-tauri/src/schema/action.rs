use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Action {
    pub name: String,
    pub desc: String,
    pub wait: usize,
    pub retry: Option<usize>,
    pub timeout: Option<u64>,
    pub command: String,
    pub args: Option<Vec<String>>,
    pub typ: String,
    pub id: Option<String>,
}
// impl From<ActionRecord> for Action {
//     fn from(value: ActionRecord) -> Self {

//     }
// }

impl From<ActionRecord> for Action {
    // type Error = anyhow::Error;

    fn from(value: ActionRecord) -> Self {
        let mut action = Action {
            name: value.name,
            desc: value.desc,
            wait: value.wait,
            command: value.command,
            args: Some(value.args.split(",").map(|s| s.to_string()).collect()),
            typ: "".to_string(),
            id: Some(value.id),
            retry: value.retry,
            timeout: value.timeout,
        };
        match value.typ {
            ActionType::ExecCommand => action.typ = "exec_command".to_string(),
            ActionType::OpenDir => action.typ = "open_dir".to_string(),
            ActionType::OpenFile => action.typ = "open_file".to_string(),
            ActionType::OpenUrl => action.typ = "open_url".to_string(),
        }

        action
    }
}

#[derive(Deserialize, Debug)]
pub struct ActionRecord {
    pub id: String,
    pub typ: ActionType,
    pub name: String,
    pub wait: usize,
    pub timeout: Option<u64>,
    pub retry: Option<usize>,
    pub desc: String,
    pub command: String,
    pub args: String,
}

#[derive(Deserialize, Debug, Clone)]
#[repr(u8)]
pub enum ActionType {
    OpenDir = 0,
    OpenFile = 1,
    OpenUrl = 2,
    ExecCommand = 3,
}
impl From<ActionType> for u8 {
    fn from(action_type: ActionType) -> Self {
        action_type as u8
    }
}
impl TryFrom<&str> for ActionType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "open_dir" => Ok(ActionType::OpenDir),
            "open_file" => Ok(ActionType::OpenFile),
            "open_url" => Ok(ActionType::OpenUrl),
            "exec_command" => Ok(ActionType::ExecCommand),
            _ => Err(anyhow::anyhow!("无效的 ActionType 值: {}", value)),
        }
    }
}
impl From<ActionType> for String {
    fn from(action_type: ActionType) -> Self {
        match action_type {
            ActionType::OpenDir => "open_dir".to_string(),
            ActionType::OpenFile => "open_file".to_string(),
            ActionType::OpenUrl => "open_url".to_string(),
            ActionType::ExecCommand => "exec_command".to_string(),
        }
    }
}
impl TryFrom<u8> for ActionType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ActionType::OpenDir),
            1 => Ok(ActionType::OpenFile),
            2 => Ok(ActionType::OpenUrl),
            3 => Ok(ActionType::ExecCommand),
            _ => Err(anyhow::anyhow!("无效的 ActionType 值: {}", value)),
        }
    }
}

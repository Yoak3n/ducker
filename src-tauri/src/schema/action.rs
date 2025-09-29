use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Action {
    pub id: Option<String>,
    pub name: String,
    pub desc: String,
    pub wait: usize,
    #[serde(rename = "type")]
    pub typ: String,
    pub retry: Option<usize>,
    pub timeout: Option<u64>,
    pub command: String,
    pub args: Option<Vec<String>>,
}

impl From<ActionRecord> for Action {

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
        action.typ = value.typ.into();

        action
    }
}

#[derive(Deserialize, Debug)]
pub struct ActionRecord {
    pub id: String,
    #[serde(rename = "type")]
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
    Directory = 0,
    File = 1,
    Url = 2,
    Command = 3,
    Notice = 4,
    Group = 11,
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
            "directory" => Ok(ActionType::Directory),
            "file" => Ok(ActionType::File),
            "url" => Ok(ActionType::Url),
            "command" => Ok(ActionType::Command),
            "notice" => Ok(ActionType::Notice),
            "group" => Ok(ActionType::Group),
            _ => Err(anyhow::anyhow!("无效的 ActionType 值: {}", value)),
        }
    }
}
impl From<ActionType> for String {
    fn from(action_type: ActionType) -> Self {
        match action_type {
            ActionType::Directory => "directory".to_string(),
            ActionType::File => "file".to_string(), 
            ActionType::Url => "url".to_string(),
            ActionType::Command => "command".to_string(),
            ActionType::Notice => "notice".to_string(),
            ActionType::Group => "group".to_string(),
        }
    }
}
impl TryFrom<u8> for ActionType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ActionType::Directory),
            1 => Ok(ActionType::File),
            2 => Ok(ActionType::Url),
            3 => Ok(ActionType::Command),   
            4 => Ok(ActionType::Notice),
            11 => Ok(ActionType::Group),
            _ => Err(anyhow::anyhow!("无效的 ActionType 值: {}", value)),
        }
    }
}

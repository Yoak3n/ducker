
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowType {
    Main,
    Dashboard,
    Task,
    Action,
    Setting,
}

impl WindowType {
    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "main" => Some(WindowType::Main),
            "dashboard" => Some(WindowType::Dashboard),
            "task" => Some(WindowType::Task),
            "action" => Some(WindowType::Action),
            "setting" => Some(WindowType::Setting),
            _ => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            WindowType::Main => "main",
            WindowType::Dashboard => "dashboard",
            WindowType::Task => "task",
            WindowType::Action => "action",
            WindowType::Setting => "setting",
        }
    }

    pub fn url(&self) -> &'static str {
        match self {
            WindowType::Main => "/main",
            WindowType::Dashboard => "/",
            WindowType::Task => "/task/",
            WindowType::Action => "/action",
            WindowType::Setting => "/setting",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            WindowType::Main => "dida",
            WindowType::Dashboard => "dida",
            WindowType::Task => "dida",
            WindowType::Action => "dida",
            WindowType::Setting => "Setting",
        }
    }
}
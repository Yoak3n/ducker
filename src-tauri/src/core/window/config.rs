use crate::schema::WindowType;

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub window_type: WindowType,
    pub inner_size: (f64, f64),
    pub min_inner_size: (f64, f64),
    pub decorations: bool,
    pub transparent: bool,
    pub skip_taskbar: bool,
    pub shadow: bool,
    pub always_on_top: bool,
    pub maximizable: bool,
    pub focused: bool,
    pub center: bool,
}


impl WindowConfig {
    pub fn new(window_type: WindowType) -> Self {
        match window_type {
            WindowType::Main => Self {
                window_type,
                inner_size: (300.0, 480.0),
                min_inner_size: (300.0, 480.0),
                decorations: false,
                transparent: true,
                skip_taskbar: true,
                shadow: false,
                always_on_top: true,
                maximizable: false,
                focused: true,
                center: true,
            },
            WindowType::Dashboard => Self {
                window_type,
                inner_size: (1080.0, 900.0),
                min_inner_size: (620.0, 550.0),
                decorations: false,
                transparent: false,
                skip_taskbar: false,
                shadow: true,
                always_on_top: false,
                maximizable: true,
                focused: true,
                center: true,
            },
            WindowType::Task => Self {
                window_type,
                inner_size: (800.0, 620.0),
                min_inner_size: (800.0, 600.0),
                decorations: false,
                transparent: false,
                skip_taskbar: true,
                shadow: true,
                always_on_top: false,
                maximizable: false,
                focused: true,
                center: false,
            },
            WindowType::Action => Self {
                window_type,
                inner_size: (890.0, 700.0),
                min_inner_size: (620.0, 550.0),
                decorations: false,
                transparent: false,
                skip_taskbar: false,
                shadow: true,
                always_on_top: false,
                maximizable: true,
                focused: true,
                center: true,
            },
            WindowType::Setting => Self {
                window_type,
                inner_size: (890.0, 700.0),
                min_inner_size: (620.0, 550.0),
                decorations: false,
                transparent: false,
                skip_taskbar: false,
                shadow: true,
                always_on_top: false,
                maximizable: true,
                focused: true,
                center: true,
            },
        }
    }
}
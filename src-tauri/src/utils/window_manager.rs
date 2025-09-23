use crate::{
    core::{handle, tray},
    logging,
    utils::logging::Type,
    schema::WindowType,
};
use mouse_position::mouse_position::Mouse;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use tauri::{Listener, Manager, PhysicalPosition, WebviewWindow, Wry};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowOperationResult {
    /// 窗口已显示并获得焦点
    Shown,
    /// 窗口已隐藏
    Hidden,
    /// 创建了新窗口
    Created,
    /// 操作失败
    Failed,
    /// 无需操作
    NoAction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowState {
    /// 窗口可见且有焦点
    VisibleFocused,
    /// 窗口可见但无焦点
    // VisibleUnfocused,
    /// 窗口最小化
    Minimized,
    /// 窗口隐藏
    Hidden,
    /// 窗口不存在
    NotExist,
}


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

// 窗口操作防抖机制
static WINDOW_OPERATION_DEBOUNCE: OnceCell<Mutex<Instant>> = OnceCell::new();
static WINDOW_OPERATION_IN_PROGRESS: AtomicBool = AtomicBool::new(false);
const WINDOW_OPERATION_DEBOUNCE_MS: u64 = 500;

fn get_window_operation_debounce() -> &'static Mutex<Instant> {
    WINDOW_OPERATION_DEBOUNCE.get_or_init(|| Mutex::new(Instant::now() - Duration::from_secs(1)))
}

fn should_handle_window_operation() -> bool {
    if WINDOW_OPERATION_IN_PROGRESS.load(Ordering::Acquire) {
        log::warn!(target: "app", "[防抖] 窗口操作已在进行中，跳过重复调用");
        return false;
    }

    let debounce_lock = get_window_operation_debounce();
    let mut last_operation = debounce_lock.lock();
    let now = Instant::now();
    let elapsed = now.duration_since(*last_operation);

    log::debug!(target: "app", "[防抖] 检查窗口操作间隔: {}ms (需要>={}ms)", 
              elapsed.as_millis(), WINDOW_OPERATION_DEBOUNCE_MS);

    if elapsed >= Duration::from_millis(WINDOW_OPERATION_DEBOUNCE_MS) {
        *last_operation = now;
        WINDOW_OPERATION_IN_PROGRESS.store(true, Ordering::Release);
        log::info!(target: "app", "[防抖] 窗口操作被允许执行");
        true
    } else {
        log::warn!(target: "app", "[防抖] 窗口操作被防抖机制忽略，距离上次操作 {}ms < {}ms", 
                  elapsed.as_millis(), WINDOW_OPERATION_DEBOUNCE_MS);
        false
    }
}

fn finish_window_operation() {
    WINDOW_OPERATION_IN_PROGRESS.store(false, Ordering::Release);
}

#[derive(Clone)]
pub struct WindowManager {
    configs: HashMap<WindowType, WindowConfig>,
    states: Arc<Mutex<HashMap<WindowType, WindowState>>>,
}

impl WindowManager {
    fn new() -> Self {
        let mut configs = HashMap::new();
        configs.insert(WindowType::Main, WindowConfig::new(WindowType::Main));
        configs.insert(
            WindowType::Dashboard,
            WindowConfig::new(WindowType::Dashboard),
        );
        configs.insert(WindowType::Task, WindowConfig::new(WindowType::Task));
        configs.insert(WindowType::Action, WindowConfig::new(WindowType::Action));
        configs.insert(WindowType::Setting, WindowConfig::new(WindowType::Setting));
        let mut states = HashMap::new();
        states.insert(WindowType::Main, WindowState::NotExist);
        states.insert(WindowType::Dashboard, WindowState::NotExist);
        states.insert(WindowType::Task, WindowState::NotExist);
        states.insert(WindowType::Action, WindowState::NotExist);
        states.insert(WindowType::Setting, WindowState::NotExist);

        Self {
            configs,
            states: Arc::new(Mutex::new(states)),
        }
    }

    pub fn global() -> &'static Self {
        static INSTANCE: OnceCell<WindowManager> = OnceCell::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// 获取窗口实例
    pub fn get_window(&self, window_type: WindowType) -> Option<WebviewWindow<Wry>> {
        handle::Handle::global()
            .app_handle()
            .and_then(|app| app.get_webview_window(window_type.label()))
    }

    /// 获取窗口状态的引用，允许外部修改
    // pub fn get_states(&self) -> Arc<Mutex<HashMap<WindowType, WindowState>>> {
    //     Arc::clone(&self.states)
    // }

    /// 更新窗口状态
    pub fn update_window_state(&self, window_type: WindowType, state: WindowState) {
        self.states.lock().insert(window_type, state);
    }

    /// 获取缓存的窗口状态（不检查实际窗口状态）
    pub fn get_cached_window_state(&self, window_type: WindowType) -> WindowState {
        self.states
            .lock()
            .get(&window_type)
            .copied()
            .unwrap_or(WindowState::NotExist)
    }

    /// 创建窗口
    fn create_window(
        &self,
        window_type: WindowType,
        url: Option<&str>,
    ) -> Result<WebviewWindow<Wry>, tauri::Error> {
        let app_handle = handle::Handle::global()
            .app_handle()
            .ok_or_else(|| tauri::Error::FailedToReceiveMessage)?;

        let config = self
            .configs
            .get(&window_type)
            .ok_or_else(|| tauri::Error::FailedToReceiveMessage)?;

        // 检查是否已存在窗口
        if let Some(existing_window) = self.get_window(window_type) {
            logging!(
                info,
                Type::Window,
                true,
                "Found existing {} window, attempting to restore",
                window_type.label()
            );

            if existing_window.is_minimized().unwrap_or(false) {
                let _ = existing_window.unminimize();
            }
            let _ = existing_window.show();
            let _ = existing_window.set_focus();

            // 更新缓存状态为可见且有焦点
            self.update_window_state(window_type, WindowState::VisibleFocused);

            // 为恢复的窗口添加轻量模式监听器（如果还没有的话）
            crate::module::lightweight::add_window_listeners(window_type.label());

            return Ok(existing_window);
        }

        logging!(
            info,
            Type::Window,
            "Creating new {} window",
            window_type.label()
        );

        let mut builder = tauri::WebviewWindowBuilder::new(
            &app_handle,
            window_type.label().to_string(),
            tauri::WebviewUrl::App(
                url.map(|u| u.into())
                    .unwrap_or_else(|| window_type.url().into()),
            ),
        )
        .title(config.window_type.title())
        .inner_size(config.inner_size.0, config.inner_size.1)
        .min_inner_size(config.min_inner_size.0, config.min_inner_size.1)
        .decorations(config.decorations)
        .focused(config.focused)
        .skip_taskbar(config.skip_taskbar)
        .always_on_top(config.always_on_top)
        .maximizable(config.maximizable)
        .transparent(config.transparent)
        .shadow(config.shadow);

        if config.center {
            builder = builder.center();
            logging!(
                info,
                Type::Window,
                true,
                "{} window centered",
                window_type.label()
            );
        }
        let postion = if let Mouse::Position { x, y } = Mouse::get_mouse_position() {
            let half_width = config.inner_size.0 / 2.0;
            let window_width = config.inner_size.0;
            let window_height = config.inner_size.1;

            // 获取屏幕尺寸进行边界检测 - 支持多屏幕
            let (screen_width, screen_height, screen_x, screen_y) = if let Ok(monitors) =
                app_handle.available_monitors()
            {
                // 查找鼠标所在的显示器
                let mouse_x = x as f64;
                let mouse_y = y as f64;

                let mut found_monitor = None;
                for monitor in monitors {
                    let pos = monitor.position();
                    let size = monitor.size();
                    let monitor_x = pos.x as f64;
                    let monitor_y = pos.y as f64;
                    let monitor_width = size.width as f64;
                    let monitor_height = size.height as f64;

                    // 检查鼠标是否在当前显示器范围内
                    if mouse_x >= monitor_x
                        && mouse_x < monitor_x + monitor_width
                        && mouse_y >= monitor_y
                        && mouse_y < monitor_y + monitor_height
                    {
                        logging!(
                            info,
                            Type::Window,
                            true,
                            "检测到鼠标在显示器: {}x{} at ({}, {})",
                            monitor_width,
                            monitor_height,
                            monitor_x,
                            monitor_y
                        );
                        found_monitor = Some((monitor_width, monitor_height, monitor_x, monitor_y));
                        break;
                    }
                }

                // 如果找到了匹配的显示器，使用它；否则使用主显示器
                if let Some(monitor_info) = found_monitor {
                    monitor_info
                } else if let Some(primary) = app_handle.primary_monitor().ok().flatten() {
                    let pos = primary.position();
                    let size = primary.size();
                    (
                        size.width as f64,
                        size.height as f64,
                        pos.x as f64,
                        pos.y as f64,
                    )
                } else {
                    // 如果无法获取屏幕尺寸，使用默认值
                    (1920.0, 1080.0, 0.0, 0.0)
                }
            } else {
                // 如果无法获取屏幕尺寸，使用默认值
                (1920.0, 1080.0, 0.0, 0.0)
            };

            // 计算初始位置
            let initial_x = x as f64 - half_width;
            let initial_y = y as f64 - 20.0;

            // 进行屏幕边缘检测和调整，保留边距空间 - 考虑多屏幕偏移
            let margin = 10.0; // 保留10像素边距
            let adjusted_x = initial_x
                .max(screen_x + margin)
                .min(screen_x + screen_width - window_width - margin);
            let adjusted_y = initial_y
                .max(screen_y + margin)
                .min(screen_y + screen_height - window_height - margin);

            logging!(
                info,
                Type::Window,
                true,
                "Task窗口位置调整: 鼠标({}, {}) -> 初始({:.1}, {:.1}) -> 最终({:.1}, {:.1}), 显示器({:.0}x{:.0} at {:.0},{:.0})",
                x, y, initial_x, initial_y, adjusted_x, adjusted_y, screen_width, screen_height, screen_x, screen_y
            );
            (adjusted_x, adjusted_y)
        } else {
            (0.0, 0.0)
        };
        if let WindowType::Task = window_type {
            builder = builder.position(postion.0, postion.1);
        }

        // 添加浏览器参数
        #[cfg(target_os = "windows")]
        {
            builder = builder.additional_browser_args("--enable-features=msWebView2EnableDraggableRegions --disable-features=OverscrollHistoryNavigation,msExperimentalScrolling");
        }

        let window = builder.build()?;
        window.set_focus()?;

        logging!(
            info,
            Type::Window,
            true,
            "{} window created successfully",
            window_type.label()
        );

        // 为新创建的窗口添加轻量模式监听器
        match window_type {
            // 特殊处理主窗口
            WindowType::Main => {
                use crate::process::AsyncHandler;
                AsyncHandler::spawn(move || async move {
                    handle::Handle::global().mark_startup_completed();
                });
            }
            // 特殊处理任务窗口
            WindowType::Task => {
                window.set_position(PhysicalPosition::new(postion.0, postion.1))?;
                let window_clone = window.clone();
                let window_manager = self.clone();

                // 监听窗口失焦事件，但添加鼠标位置检查
                window.listen("tauri://blur", move |_event| {
                    let window_clone_inner = window_clone.clone();
                    let window_manager_inner = window_manager.clone();

                    // 检查鼠标是否在窗口区域外
                    if let Ok(window_position) = window_clone_inner.outer_position() {
                        if let Ok(window_size) = window_clone_inner.outer_size() {
                            if let Mouse::Position {
                                x: mouse_x,
                                y: mouse_y,
                            } = Mouse::get_mouse_position()
                            {
                                let window_x = window_position.x;
                                let window_y = window_position.y;
                                let window_width = window_size.width as i32;
                                let window_height = window_size.height as i32;

                                // 只有当鼠标在窗口外部时才关闭
                                if mouse_x < window_x
                                    || mouse_x > window_x + window_width
                                    || mouse_y < window_y
                                    || mouse_y > window_y + window_height
                                {
                                    logging!(
                                        info,
                                        Type::Window,
                                        "Task窗口失焦且鼠标在窗口外，自动关闭"
                                    );
                                    let _ = window_clone_inner.destroy();
                                    window_manager_inner.update_window_state(
                                        WindowType::Task,
                                        WindowState::NotExist,
                                    );
                                } else {
                                    logging!(
                                        info,
                                        Type::Window,
                                        "Task窗口失焦但鼠标仍在窗口内，不关闭"
                                    );
                                }
                            }
                        }
                    }
                });
            }
            _ => {}
        }

        Ok(window)
    }

    /// 激活窗口（取消最小化、显示、设置焦点）
    fn activate_window(
        &self,
        window: &WebviewWindow<Wry>,
        window_type: WindowType,
    ) -> WindowOperationResult {
        logging!(info, Type::Window, true, "开始激活窗口");

        let mut operations_successful = true;

        // 1. 如果窗口最小化，先取消最小化
        if window.is_minimized().unwrap_or(false) {
            logging!(info, Type::Window, true, "窗口已最小化，正在取消最小化");
            if let Err(e) = window.unminimize() {
                logging!(warn, Type::Window, true, "取消最小化失败: {}", e);
                operations_successful = false;
            }
        }

        // 2. 显示窗口
        if let Err(e) = window.show() {
            logging!(warn, Type::Window, true, "显示窗口失败: {}", e);
            operations_successful = false;
        }

        // 3. 设置焦点
        if let Err(e) = window.set_focus() {
            logging!(warn, Type::Window, true, "设置窗口焦点失败: {}", e);
            operations_successful = false;
        }

        // 4. 平台特定的激活策略
        #[cfg(target_os = "windows")]
        {
            // Windows 尝试额外的激活方法
            if let Err(e) = window.set_always_on_top(true) {
                logging!(
                    debug,
                    Type::Window,
                    true,
                    "设置置顶失败（非关键错误）: {}",
                    e
                );
            }
            // 立即取消置顶
            if let Err(e) = window.set_always_on_top(false) {
                logging!(
                    debug,
                    Type::Window,
                    true,
                    "取消置顶失败（非关键错误）: {}",
                    e
                );
            }
        }

        let result = if operations_successful {
            logging!(info, Type::Window, true, "窗口激活成功");
            WindowOperationResult::Shown
        } else {
            logging!(warn, Type::Window, true, "窗口激活部分失败");
            WindowOperationResult::Failed
        };

        // 更新缓存状态
        if result == WindowOperationResult::Shown {
            self.update_window_state(window_type, WindowState::VisibleFocused);
        }

        result
    }

    /// 显示窗口
    pub fn show_window(&self, window_type: WindowType, url: Option<&str>) -> WindowOperationResult {
        if !should_handle_window_operation() {
            return WindowOperationResult::NoAction;
        }
        let _guard = scopeguard::guard((), |_| {
            finish_window_operation();
        });

        logging!(
            info,
            Type::Window,
            true,
            "开始显示 {} 窗口",
            window_type.label()
        );
        let current_state = self.get_cached_window_state(window_type);

        let result = match current_state {
            WindowState::NotExist => {
                logging!(info, Type::Window, true, "窗口不存在，创建新窗口");
                match self.create_window(window_type, url) {
                    Ok(_) => {
                        logging!(info, Type::Window, true, "窗口创建成功");
                        std::thread::sleep(std::time::Duration::from_millis(10));
                        WindowOperationResult::Created
                    }
                    Err(e) => {
                        logging!(warn, Type::Window, true, "窗口创建失败: {}", e);
                        WindowOperationResult::Failed
                    }
                }
            }
            WindowState::VisibleFocused => {
                logging!(info, Type::Window, true, "窗口已经可见且有焦点，无需操作");
                WindowOperationResult::NoAction
            }
            WindowState::Minimized | WindowState::Hidden => {
                if let Some(window) = self.get_window(window_type) {
                    self.activate_window(&window, window_type)
                } else {
                    WindowOperationResult::Failed
                }
            }
        };

        // 更新缓存状态
        if matches!(
            result,
            WindowOperationResult::Created | WindowOperationResult::Shown
        ) {
            self.update_window_state(window_type, WindowState::VisibleFocused);
        }

        result
    }

    /// 隐藏窗口
    pub fn close_window(&self, window_type: WindowType) -> WindowOperationResult {
        logging!(
            info,
            Type::Window,
            true,
            "开始隐藏 {} 窗口",
            window_type.label()
        );

        let result = match self.get_window(window_type) {
            Some(window) => {
                let operation = window.close();
                match operation {
                    Ok(_) => {
                        logging!(info, Type::Window, true, "窗口已隐藏");
                        WindowOperationResult::Hidden
                    }
                    Err(e) => {
                        logging!(warn, Type::Window, true, "隐藏窗口失败: {}", e);
                        WindowOperationResult::Failed
                    }
                }
            }
            None => {
                logging!(info, Type::Window, true, "窗口不存在，无需隐藏");
                WindowOperationResult::NoAction
            }
        };

        // 更新缓存状态
        self.update_window_state(window_type, WindowState::Hidden);

        result
    }

    /// 切换窗口显示状态
    pub fn toggle_window(&self, window_type: WindowType) -> WindowOperationResult {
        if !should_handle_window_operation() {
            return WindowOperationResult::NoAction;
        }
        let _guard = scopeguard::guard((), |_| {
            finish_window_operation();
        });

        logging!(
            info,
            Type::Window,
            true,
            "开始切换 {} 窗口显示状态",
            window_type.label()
        );

        let current_state = self.get_cached_window_state(window_type);
        logging!(
            info,
            Type::Window,
            true,
            "当前窗口状态: {:?}",
            current_state
        );

        // 更新托盘菜单状态
        let update_tray = |visible: bool| {
            if matches!(window_type, WindowType::Main | WindowType::Dashboard) {
                tray::Tray::global().update_menu_visible(visible);
            }
        };

        let result = match current_state {
            WindowState::NotExist => {
                logging!(info, Type::Window, true, "窗口不存在，将创建新窗口");
                match self.create_window(window_type, None) {
                    Ok(_) => {
                        update_tray(true);
                        WindowOperationResult::Created
                    }
                    Err(_) => WindowOperationResult::Failed,
                }
            }
            WindowState::VisibleFocused => {
                logging!(info, Type::Window, true, "窗口可见，将隐藏窗口");
                update_tray(false);
                self.close_window(window_type)
            }
            WindowState::Minimized | WindowState::Hidden => {
                logging!(
                    info,
                    Type::Window,
                    true,
                    "窗口存在但被隐藏或最小化，将激活窗口"
                );
                if let Some(window) = self.get_window(window_type) {
                    update_tray(true);
                    self.activate_window(&window, window_type)
                } else {
                    logging!(warn, Type::Window, true, "无法获取窗口实例");
                    WindowOperationResult::Failed
                }
            }
        };

        // 更新缓存状态（注意：hide_window已经处理了隐藏状态的更新）
        match result {
            WindowOperationResult::Created => {
                self.update_window_state(window_type, WindowState::VisibleFocused);
            }
            WindowOperationResult::Shown => {
                self.update_window_state(window_type, WindowState::VisibleFocused);
            }
            // Hidden状态已在hide_window中处理
            _ => {}
        }

        result
    }

    fn minimized_window(&self, window_type: WindowType) -> bool {
        match self.get_window(window_type) {
            Some(window) => {
                if window.is_minimized().unwrap_or(false) {
                    return true;
                } else {
                    if let Err(e) = window.minimize() {
                        logging!(error, Type::Window, true, "窗口最小化失败: {:?}", e);
                        return false;
                    }
                    self.update_window_state(window_type, WindowState::Minimized);
                    return true;
                }
            }
            None => return false,
        }
    }

    /// 检查是否所有窗口都已关闭（隐藏或不存在）
    pub fn are_all_windows_closed(&self) -> bool {
        let all_window_types = [
            WindowType::Main,
            WindowType::Dashboard,
            WindowType::Task,
            WindowType::Action,
            WindowType::Setting,
        ];

        for window_type in &all_window_types {
            let label = window_type.label();
            let state = self.get_cached_window_state(*window_type);
            logging!(info, Type::Window, true, "window {} is {:?}", label, state);
            match state {
                WindowState::VisibleFocused
                | WindowState::Minimized => {
                    return false; // 有窗口仍然可见或最小化
                }
                WindowState::Hidden | WindowState::NotExist => {
                    // 窗口已隐藏或不存在，继续检查下一个
                }
            }
        }
        true // 所有窗口都已关闭
    }

    // /// 检查是否有任何窗口获得焦点
    // pub fn has_any_window_focused(&self) -> bool {
    //     let all_window_types = [
    //         WindowType::Main,
    //         WindowType::Dashboard,
    //         WindowType::Task,
    //         WindowType::Action,
    //         WindowType::Setting,
    //     ];

    //     for window_type in &all_window_types {
    //         if self.is_window_focused(*window_type) {
    //             return true;
    //         }
    //     }
    //     false
    // }
}

// 保持向后兼容的公共API
pub fn toggle_main_window() -> WindowOperationResult {
    WindowManager::global().toggle_window(WindowType::Main)
}

pub fn hide_main_window() -> WindowOperationResult {
    WindowManager::global().close_window(WindowType::Main)
}

pub fn create_main_window() {
    let _ = WindowManager::global().create_window(WindowType::Main, None);
    WindowManager::global().update_window_state(WindowType::Main, WindowState::VisibleFocused);
}

pub fn toggle_window_by_label(label: &str) -> WindowOperationResult {
    if let Some(window_type) = WindowType::from_label(label){
        WindowManager::global().toggle_window(window_type)
    } else {
        logging!(error, Type::Window, true, "Unknown window label: {}", label);
        WindowOperationResult::Failed
    }
}

pub fn show_window_by_label(label: &str, url: Option<&str>) {
    if let Some(window_type) = WindowType::from_label(label){
        let _ = WindowManager::global().show_window(window_type, url);
        if window_type == WindowType::Main {
            tray::Tray::global().update_menu_visible(true);
        }
    } else {
        logging!(error, Type::Window, true, "Unknown window label: {}", label);
    }
}

pub fn minimize_window_by_label(label: &str) -> bool {
    if let Some(window_type) = WindowType::from_label(label){
        WindowManager::global().minimized_window(window_type)
    } else {
        logging!(error, Type::Window, true, "Unknown window label: {}", label);
        false
    }
}

pub fn close_window_by_label(label: &str) {
    if let Some(window_type) = WindowType::from_label(label){
        WindowManager::global().close_window(window_type);
        if window_type == WindowType::Main {
            tray::Tray::global().update_menu_visible(false);
        }
    } else {
        logging!(error, Type::Window, true, "Unknown window label: {}", label);
    }
}

/// 检查是否所有窗口都已关闭
pub fn are_all_windows_closed() -> bool {
    WindowManager::global().are_all_windows_closed()
}

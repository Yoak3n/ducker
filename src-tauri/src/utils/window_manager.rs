use crate::{
    core::{handle, tray},
    logging,
    utils::logging::Type,
};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};
use tauri::{Manager, WebviewWindow, Wry};

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
    VisibleUnfocused,
    /// 窗口最小化
    Minimized,
    /// 窗口隐藏
    Hidden,
    /// 窗口不存在
    NotExist,
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

#[derive(Debug)]
pub enum WindowLabel {
    Main,
    Dashboard,
    Action,
    Setting,
}

pub fn toggle_main_window() {
    let app_handle = handle::Handle::global().app_handle().unwrap();
    if let Some(window) = app_handle.get_webview_window("main") {
        let visible = window.is_visible().unwrap_or(false);
        tray::Tray::global().update_menu_visible(!visible);
        if visible {
            window.close().unwrap();
        } else {
            window.show().unwrap();
            window.set_focus().unwrap();
        }
    } else {
        create_main_window();
        logging!(info, Type::Window, true, "Creating main window");
        tray::Tray::global().update_menu_visible(true);
    }
}

pub fn create_window_by_label(label: &WindowLabel) {
    match label {
        WindowLabel::Main => {
            logging!(info, Type::Window, true, "Creating main window");
            create_main_window();
        }
        WindowLabel::Dashboard => {
            logging!(info, Type::Window, true, "Creating dashboard window");
            create_dashboard_window();
        }
        WindowLabel::Action => {
            logging!(info, Type::Window, true, "Creating action window");
            create_action_window();
        }
        WindowLabel::Setting => {
            logging!(info, Type::Window, true, "Creating setting window");
            create_settings_window();
        }
    }
}

pub fn create_main_window() {
    logging!(info, Type::Window, true, "Creating window");
    let app_handle = handle::Handle::global().app_handle().unwrap();
    #[cfg(desktop)]
    {
        if let Some(window) = handle::Handle::global().get_window() {
            logging!(
                info,
                Type::Window,
                true,
                "Found existing window, attempting to restore visibility"
            );

            if window.is_minimized().unwrap_or(false) {
                logging!(
                    info,
                    Type::Window,
                    true,
                    "Window is minimized, restoring window state"
                );
                let _ = window.unminimize();
            }
            let _ = window.show();
            let _ = window.set_focus();
            return;
        }

        logging!(info, Type::Window, true, "Creating new application window");

        #[cfg(target_os = "windows")]
        let window = tauri::WebviewWindowBuilder::new(
                    &app_handle,
                    "main".to_string(),
                    tauri::WebviewUrl::App("/main".into()),
                )
                .title("dida")
                .inner_size(890.0, 700.0)
                .min_inner_size(620.0, 550.0)
                .decorations(false)
                .focused(true)
                .maximizable(true)
                .additional_browser_args("--enable-features=msWebView2EnableDraggableRegions --disable-features=OverscrollHistoryNavigation,msExperimentalScrolling")
                .transparent(true)
                .skip_taskbar(true)
                .shadow(false)
                .always_on_top(true)
                .center()
                .build();
        #[cfg(target_os = "linux")]
        let window = tauri::WebviewWindowBuilder::new(
            &app_handle,
            "main".to_string(),
            tauri::WebviewUrl::App("/main".into())
        )
        .title("dida")
        .focused(true)
        .center()
        .build();
        match window {
            Ok(w) => {
                use crate::process::AsyncHandler;
                w.set_focus().unwrap();
                logging!(info, Type::Window, true, "Window created successfully");
                // let app_handle_clone = app_handle.clone();
                AsyncHandler::spawn(move || async move {
                    // 处理启动完成
                    handle::Handle::global().mark_startup_completed();
                });
            }
            Err(e) => {
                logging!(
                    error,
                    Type::Window,
                    true,
                    "Failed to create window: {:?}",
                    e
                );
            }
        }
    }
    #[cfg(mobile)]
    {
        println!("android");
        logging!(info, Type::Window, true, "Creating new application window");
        let _ = tauri::WebviewWindowBuilder::new(
            &app_handle,
            "main".to_string(),
            tauri::WebviewUrl::App("index.html".into()),
        )
        .build();
    }
}

fn create_dashboard_window() {
    let app_handle = handle::Handle::global().app_handle().unwrap();
    if let Some(window) = app_handle.get_webview_window("dashboard") {
        logging!(
            info,
            Type::Window,
            true,
            "Found existing window, attempting to restore visibility"
        );

        if window.is_minimized().unwrap_or(false) {
            logging!(
                info,
                Type::Window,
                true,
                "Window is minimized, restoring window state"
            );
            let _ = window.unminimize();
        }
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }

    let window = tauri::WebviewWindowBuilder::new(
                    &app_handle,
                    "dashboard".to_string(),
                    tauri::WebviewUrl::App("/".into()),
                )
                .title("dida")
                .inner_size(1080.0, 900.0)
                .min_inner_size(620.0, 550.0)
                .decorations(false)
                .focused(true)
                .maximizable(true)
                .additional_browser_args("--enable-features=msWebView2EnableDraggableRegions --disable-features=OverscrollHistoryNavigation,msExperimentalScrolling")
                .transparent(false)
                .shadow(true)
                .center()
                .build();

    match window {
        Ok(w) => {
            w.set_focus().unwrap();
            logging!(info, Type::Window, true, "Window created successfully");
            // let app_handle_clone = app_handle.clone();
        }
        Err(e) => {
            logging!(
                error,
                Type::Window,
                true,
                "Failed to create window: {:?}",
                e
            );
        }
    }
}

fn create_action_window() {
    let app_handle = handle::Handle::global().app_handle().unwrap();
    if let Some(window) = app_handle.get_webview_window("action") {
        logging!(
            info,
            Type::Window,
            true,
            "Found existing window, attempting to restore visibility"
        );

        if window.is_minimized().unwrap_or(false) {
            logging!(
                info,
                Type::Window,
                true,
                "Window is minimized, restoring window state"
            );
            let _ = window.unminimize();
        }
    }
    let window = tauri::WebviewWindowBuilder::new
                    (&app_handle,
                    "action".to_string(),
                    tauri::WebviewUrl::App("/action".into()),
                )
                .title("dida")
                .inner_size(890.0, 700.0)
                .min_inner_size(620.0, 550.0)
                .decorations(false)
                .focused(true)
                .maximizable(true)
                .additional_browser_args("--enable-features=msWebView2EnableDraggableRegions --disable-features=OverscrollHistoryNavigation,msExperimentalScrolling")
                .transparent(false)
                .shadow(true)
                .center()
                .build();
    match window {
        Ok(w) => {
            w.set_focus().unwrap();
            logging!(info, Type::Window, true, "Window created successfully");
            // let app_handle_clone = app_handle.clone();
        }
        Err(e) => {
            logging!(
                error,
                Type::Window,
                true,
                "Failed to create window: {:?}",
                e
            );
        }
    }
}
pub fn create_settings_window() {
    let app_handle = handle::Handle::global().app_handle().unwrap();
    if let Some(window) = app_handle.get_webview_window("settings") {
        logging!(
            info,
            Type::Window,
            true,
            "Found existing window, attempting to restore visibility"
        );

        if window.is_minimized().unwrap_or(false) {
            logging!(
                info,
                Type::Window,
                true,
                "Window is minimized, restoring window state"
            );
            let _ = window.unminimize();
        }
    }
    let window = tauri::WebviewWindowBuilder::new(
                    &app_handle,
                    "settings".to_string(),
                    tauri::WebviewUrl::App("/settings".into()),
                )
                .title("dida")
                .inner_size(890.0, 700.0)
                .min_inner_size(620.0, 550.0)
                .decorations(false)
                .focused(true)
                .maximizable(true)
                .additional_browser_args("--enable-features=msWebView2EnableDraggableRegions --disable-features=OverscrollHistoryNavigation,msExperimentalScrolling")
                .transparent(false)
                .shadow(true)
                .center()
                .build();

    match window {
        Ok(w) => {
            w.set_focus().unwrap();
            logging!(info, Type::Window, true, "Window created successfully");
            // let app_handle_clone = app_handle.clone();
        }
        Err(e) => {
            logging!(
                error,
                Type::Window,
                true,
                "Failed to create window: {:?}",
                e
            );
        }
    }
}

pub struct WindowManager;

impl WindowManager {
    pub fn get_main_window_state() -> WindowState {
        match Self::get_main_window() {
            Some(window) => {
                let is_minimized = window.is_minimized().unwrap_or(false);
                let is_visible = window.is_visible().unwrap_or(false);
                let is_focused = window.is_focused().unwrap_or(false);

                if is_minimized {
                    return WindowState::Minimized;
                }
                if !is_visible {
                    return WindowState::Hidden;
                }

                if is_focused {
                    WindowState::VisibleFocused
                } else {
                    WindowState::VisibleUnfocused
                }
            }
            None => WindowState::NotExist,
        }
    }

    pub fn get_main_window() -> Option<WebviewWindow<Wry>> {
        handle::Handle::global()
            .app_handle()
            .and_then(|app| app.get_webview_window("main"))
    }

    pub fn show_main_window() -> WindowOperationResult {
        // 防抖检查
        if !should_handle_window_operation() {
            return WindowOperationResult::NoAction;
        }
        let _guard = scopeguard::guard((), |_| {
            finish_window_operation();
        });

        logging!(info, Type::Window, true, "开始智能显示主窗口");
        logging!(
            debug,
            Type::Window,
            true,
            "{}",
            Self::get_window_status_info()
        );

        let current_state = Self::get_main_window_state();

        match current_state {
            WindowState::NotExist => {
                logging!(info, Type::Window, true, "窗口不存在，创建新窗口");
                if Self::create_new_window() {
                    logging!(info, Type::Window, true, "窗口创建成功");
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    WindowOperationResult::Created
                } else {
                    logging!(warn, Type::Window, true, "窗口创建失败");
                    WindowOperationResult::Failed
                }
            }
            WindowState::VisibleFocused => {
                logging!(info, Type::Window, true, "窗口已经可见且有焦点，无需操作");
                WindowOperationResult::NoAction
            }
            WindowState::VisibleUnfocused | WindowState::Minimized | WindowState::Hidden => {
                if let Some(window) = Self::get_main_window() {
                    let state_after_check = Self::get_main_window_state();
                    if state_after_check == WindowState::VisibleFocused {
                        logging!(
                            info,
                            Type::Window,
                            true,
                            "窗口在检查期间已变为可见和有焦点状态"
                        );
                        return WindowOperationResult::NoAction;
                    }
                    Self::activate_window(&window)
                } else {
                    WindowOperationResult::Failed
                }
            }
        }
    }

    pub fn toggle_main_window() -> WindowOperationResult {
        // 防抖检查
        if !should_handle_window_operation() {
            return WindowOperationResult::NoAction;
        }
        let _guard = scopeguard::guard((), |_| {
            finish_window_operation();
        });

        logging!(info, Type::Window, true, "开始切换主窗口显示状态");

        let current_state = Self::get_main_window_state();
        logging!(
            info,
            Type::Window,
            true,
            "当前窗口状态: {:?} | 详细状态: {}",
            current_state,
            Self::get_window_status_info()
        );

        match current_state {
            WindowState::NotExist => {
                // 窗口不存在，创建新窗口
                logging!(info, Type::Window, true, "窗口不存在，将创建新窗口");
                // 由于已经有防抖保护，直接调用内部方法
                if Self::create_new_window() {
                    tray::Tray::global().update_menu_visible(true);
                    WindowOperationResult::Created
                } else {
                    WindowOperationResult::Failed
                }
            }
            WindowState::VisibleFocused | WindowState::VisibleUnfocused => {
                logging!(
                    info,
                    Type::Window,
                    true,
                    "窗口可见（焦点状态: {}），将隐藏窗口",
                    if current_state == WindowState::VisibleFocused {
                        "有焦点"
                    } else {
                        "无焦点"
                    }
                );
                tray::Tray::global().update_menu_visible(false);
                if let Some(window) = Self::get_main_window() {
                    match window.hide() {
                        Ok(_) => {
                            logging!(info, Type::Window, true, "窗口已成功隐藏");
                            WindowOperationResult::Hidden
                        }
                        Err(e) => {
                            logging!(warn, Type::Window, true, "隐藏窗口失败: {}", e);
                            WindowOperationResult::Failed
                        }
                    }
                } else {
                    logging!(warn, Type::Window, true, "无法获取窗口实例");
                    WindowOperationResult::Failed
                }
            }
            WindowState::Minimized | WindowState::Hidden => {
                logging!(
                    info,
                    Type::Window,
                    true,
                    "窗口存在但被隐藏或最小化，将激活窗口"
                );
                if let Some(window) = Self::get_main_window() {                    
                    tray::Tray::global().update_menu_visible(true);
                    Self::activate_window(&window)
                } else {
                    logging!(warn, Type::Window, true, "无法获取窗口实例");
                    WindowOperationResult::Failed
                }
            }
        }
    }
    /// 激活窗口（取消最小化、显示、设置焦点）
    fn activate_window(window: &WebviewWindow<Wry>) -> WindowOperationResult {
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
        #[cfg(target_os = "macos")]
        {
            logging!(info, Type::Window, true, "应用 macOS 特定的激活策略");
            AppHandleManager::global().set_activation_policy_regular();
        }

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

        if operations_successful {
            logging!(info, Type::Window, true, "窗口激活成功");
            WindowOperationResult::Shown
        } else {
            logging!(warn, Type::Window, true, "窗口激活部分失败");
            WindowOperationResult::Failed
        }
    }

    /// 隐藏主窗口
    pub fn hide_main_window() -> WindowOperationResult {
        logging!(info, Type::Window, true, "开始隐藏主窗口");

        match Self::get_main_window() {
            Some(window) => match window.hide() {
                Ok(_) => {
                    logging!(info, Type::Window, true, "窗口已隐藏");
                    WindowOperationResult::Hidden
                }
                Err(e) => {
                    logging!(warn, Type::Window, true, "隐藏窗口失败: {}", e);
                    WindowOperationResult::Failed
                }
            },
            None => {
                logging!(info, Type::Window, true, "窗口不存在，无需隐藏");
                WindowOperationResult::NoAction
            }
        }
    }

    /// 检查窗口是否可见
    pub fn is_main_window_visible() -> bool {
        Self::get_main_window()
            .map(|window| window.is_visible().unwrap_or(false))
            .unwrap_or(false)
    }

    /// 检查窗口是否有焦点
    pub fn is_main_window_focused() -> bool {
        Self::get_main_window()
            .map(|window| window.is_focused().unwrap_or(false))
            .unwrap_or(false)
    }

    /// 检查窗口是否最小化
    pub fn is_main_window_minimized() -> bool {
        Self::get_main_window()
            .map(|window| window.is_minimized().unwrap_or(false))
            .unwrap_or(false)
    }

    /// 创建新窗口,防抖避免重复调用
    fn create_new_window() -> bool {
        create_main_window();
        true
    }

    /// 获取详细的窗口状态信息
    pub fn get_window_status_info() -> String {
        let state = Self::get_main_window_state();
        let is_visible = Self::is_main_window_visible();
        let is_focused = Self::is_main_window_focused();
        let is_minimized = Self::is_main_window_minimized();

        format!(
            "窗口状态: {:?} | 可见: {} | 有焦点: {} | 最小化: {}",
            state, is_visible, is_focused, is_minimized
        )
    }
}

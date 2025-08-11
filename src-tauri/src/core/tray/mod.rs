use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

use anyhow::Result;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use tauri::{menu::CheckMenuItem, tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}};
use tauri::Wry;
use tauri::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    AppHandle,
};
// use super::handle;
use crate::{
    core::handle, // utils::logging::Type
    feat,
    logging,
    utils::{logging::Type, resolve, window_manager},
};

pub struct Tray {
    last_menu_update: Mutex<Option<Instant>>,
    menu_updating: AtomicBool,
}

impl Tray {
    pub fn global() -> &'static Tray {
        static TRAY: OnceCell<Tray> = OnceCell::new();

        #[cfg(target_os = "macos")]
        return TRAY.get_or_init(|| Tray {
            speed_rate: Arc::new(Mutex::new(None)),
            shutdown_tx: Arc::new(RwLock::new(None)),
            is_subscribed: Arc::new(RwLock::new(false)),
            rate_cache: Arc::new(Mutex::new(None)),
        });

        #[cfg(not(target_os = "macos"))]
        return TRAY.get_or_init(|| Tray {
            last_menu_update: Mutex::new(None),
            menu_updating: AtomicBool::new(false),
        });
    }

    pub fn init(&self) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            let mut speed_rate = self.speed_rate.lock();
            *speed_rate = Some(SpeedRate::new());
        }
        Ok(())
    }

    fn update_menu_internal(&self, app_handle: &AppHandle) -> Result<()> {
        match app_handle.tray_by_id("main") {
            Some(tray) => {
                let menu =
                    create_tray_menu(app_handle, handle::Handle::global().get_window_visible())?;
                tray.set_menu(Some(menu))?;
                log::debug!(target: "app", "托盘菜单更新成功");
            }
            None => {
                log::warn!(target: "app", "更新托盘菜单失败: 托盘不存在");
            }
        }
        Ok(())
    }

    pub fn update_menu(&self) -> Result<()> {
        const MIN_UPDATE_INTERVAL: Duration = Duration::from_millis(100);
        if self.menu_updating.load(Ordering::Acquire) {
            return Ok(());
        }
        let should_force_update = match std::thread::current().name() {
            Some("main") => true,
            _ => {
                let last_update = self.last_menu_update.lock();
                if let Some(last_time) = *last_update {
                    last_time.elapsed() >= MIN_UPDATE_INTERVAL
                } else {
                    true
                }
            }
        };

        if !should_force_update {
            return Ok(());
        }
        let app_handle = handle::Handle::global().app_handle().unwrap();
        self.menu_updating.store(true, Ordering::Release);
        let result = self.update_menu_internal(&app_handle);

        {
            let mut last_update = self.last_menu_update.lock();
            *last_update = Some(Instant::now());
        }
        self.menu_updating.store(false, Ordering::Release);

        result
    }

    pub fn update_tooltip(&self) -> Result<()> {
        let app_handle = handle::Handle::global().app_handle().unwrap();

        let tray = app_handle.tray_by_id("main").unwrap();
        tray.set_tooltip(Some("Dida"))?;
        Ok(())
    }

    pub fn update_part(&self) -> Result<()> {
        // self.update_menu()?;
        self.update_tooltip()?;
        self.update_icon(None)?;
        self.update_tray_display()?;
        Ok(())
    }

    pub fn update_icon(&self, _rate: Option<String>) -> Result<()> {
        Ok(())
    }
    pub fn update_tray_display(&self) -> Result<()> {
        let app_handle = handle::Handle::global().app_handle().unwrap();
        let _tray = app_handle.tray_by_id("main").unwrap();

        // 更新菜单
        self.update_menu()?;

        Ok(())
    }
    // pub fn update_all_states(&self) -> Result<()> {
    //     // 确保所有状态更新完成
    //     // self.update_menu()?;
    //     self.update_icon(None)?;
    //     self.update_tooltip()?;
    //     self.update_tray_display()?;
    //     Ok(())
    // }
    pub fn update_menu_visible(&self, visible: bool) {
        let app_handle = handle::Handle::global().app_handle().unwrap();
        let tray = app_handle.tray_by_id("main").unwrap();
        tray.set_menu(Some(create_tray_menu(&app_handle, visible).unwrap())).unwrap();
    }

    pub fn create_tray_from_handle(&self, app_handle: &AppHandle) -> Result<()> {
        log::info!(target: "app", "正在从AppHandle创建系统托盘");
        let mut builder = TrayIconBuilder::with_id("main")
            .icon(app_handle.default_window_icon().unwrap().clone())
            .icon_as_template(false);
        // 不明所以
        let tray_event = String::from("main_window");
        if tray_event.as_str() != "tray_menu" {
            builder = builder.show_menu_on_left_click(false);
        }
        let tray = builder.build(app_handle)?;
        tray.on_tray_icon_event(|_, event| {
            let tray_event = String::from("main_window");
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Down,
                    ..
                } => match tray_event.as_str() {
                    "main_window" => {
                        log::info!(target: "app", "Tray点击事件: 显示主窗口");
                    }
                    _ => {}
                },
                TrayIconEvent::DoubleClick {
                    button: MouseButton::Left,
                    ..
                } => window_manager::toggle_main_window(),
                _ => {}
            }
        });
        tray.on_menu_event(on_menu_event);
        log::info!(target: "app", "系统托盘创建成功");
        Ok(())
    }
}

fn create_tray_menu(app_handle: &AppHandle, visiable: bool) -> Result<Menu<Wry>> {
    let version = resolve::VERSION.get().unwrap();
    let show_item = &CheckMenuItem::with_id(app_handle, "open_window", "Live2D", true, visiable, Some("Show")).unwrap();
    let dashboard_item = &MenuItem::with_id(app_handle, "dashboard", "Dashboard", true, None::<&str>).unwrap();
    let separator = &PredefinedMenuItem::separator(app_handle).unwrap();
    let action_item = &MenuItem::with_id(app_handle, "action", "Action", true, None::<&str>).unwrap();
    let about_item = &MenuItem::with_id(app_handle, "about", format!("About v{version}"), true, None::<&str>).unwrap();
    let setting_item = &MenuItem::with_id(app_handle, "setting", "Setting", true, None::<&str>).unwrap();
    let quit =
        &MenuItem::with_id(app_handle, "quit", "Exit", true, Some("")).unwrap();

    let menu = tauri::menu::MenuBuilder::new(app_handle)
        .items(&[show_item, dashboard_item, action_item, about_item, setting_item, separator, quit])
        .build()
        .unwrap();
    logging!(info, Type::Tray, true, "Creating tray menu");
    Ok(menu)
}

fn on_menu_event(_: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "open_window" => window_manager::toggle_main_window(),
        "dashboard" => window_manager::create_window_by_label(&window_manager::WindowLabel::Dashboard),
        "action" => window_manager::create_window_by_label(&window_manager::WindowLabel::Action),
        // "about" => window_manager::create_window_by_label(&window_manager::WindowLabel::About),
        "setting" => window_manager::create_window_by_label(&window_manager::WindowLabel::Setting),

        "quit" => feat::quit(Some(0)),
        _ => {}
    }
    // if let Err(e) = Tray::global().update_all_states() {
    //     log::warn!(target: "app", "更新托盘状态失败: {}", e);
    // }
}

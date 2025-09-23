use crate::{logging, process::AsyncHandler, utils::logging::Type};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager, WebviewWindow};
use tauri_plugin_notification::NotificationExt;
/// 存储启动期间的错误消息
#[derive(Debug, Clone)]
struct ErrorMessage {
    status: String,
    message: String,
}

#[derive(Debug, Default, Clone)]
pub struct Handle {
    pub app_handle: Arc<RwLock<Option<AppHandle>>>,
    pub is_exiting: Arc<RwLock<bool>>,
    startup_errors: Arc<RwLock<Vec<ErrorMessage>>>,
    startup_completed: Arc<RwLock<bool>>,
}

impl Handle {
    pub fn global() -> &'static Handle {
        static HANDLE: OnceCell<Handle> = OnceCell::new();

        HANDLE.get_or_init(|| Handle {
            app_handle: Arc::new(RwLock::new(None)),
            is_exiting: Arc::new(RwLock::new(false)),
            startup_errors: Arc::new(RwLock::new(Vec::new())),
            startup_completed: Arc::new(RwLock::new(false)),
        })
    }
    pub fn init(&self, app_handle: AppHandle) {
        let mut handle = self.app_handle.write();
        *handle = Some(app_handle.clone());
    }

    pub fn is_exiting(&self) -> bool {
        *self.is_exiting.read()
    }
    pub fn set_is_exiting(&self) {
        let mut is_exiting = self.is_exiting.write();
        *is_exiting = true;
    }

    pub fn app_handle(&self) -> Option<AppHandle> {
        self.app_handle.read().clone()
    }

    pub fn get_window_by_label(&self, label: &str) -> Option<WebviewWindow> {
        let app_handle = self.app_handle().unwrap();
        let window: Option<WebviewWindow> = app_handle.get_webview_window(label);
        window
    }

    pub fn get_main_window(&self) -> Option<WebviewWindow> {
        let app_handle = self.app_handle().unwrap();
        let window: Option<WebviewWindow> = app_handle.get_webview_window("main");
        if window.is_none() {
            log::error!(target:"app", "main window not found");
            return None;
        }
        window
    }
    pub fn get_window_visible(&self) -> bool {
        let window = self.get_main_window();
        match window {
            // TODO 没有按照预想的立即返回false
            Some(window) => window.is_visible().unwrap_or(false),
            None => false,
        }
    }

    pub fn notice_message<S: Into<String>, M: Into<String>>(status: S, msg: M) {
        let handle = Self::global();
        let status_str = status.into();
        let msg_str = msg.into();
        if !*handle.startup_completed.read() {
            logging!(
                info,
                Type::Frontend,
                true,
                "启动过程中收到消息，加入消息队列: {} - {}",
                status_str,
                msg_str
            );

            // 将消息添加到启动错误队列
            let mut errors = handle.startup_errors.write();
            errors.push(ErrorMessage {
                status: status_str,
                message: msg_str,
            });
            return;
        }
        handle
            .app_handle()
            .unwrap()
            .notification()
            .builder()
            .title(&status_str)
            .body(&msg_str)
            .show()
            .unwrap();
    }
    pub fn mark_startup_completed(&self) {
        {
            let mut completed = self.startup_completed.write();
            *completed = true;
        }

        self.send_startup_errors();
    }
    fn send_startup_errors(&self) {
        let errors = {
            let mut errors = self.startup_errors.write();
            std::mem::take(&mut *errors)
        };

        if errors.is_empty() {
            return;
        }

        logging!(
            info,
            Type::Frontend,
            true,
            "发送{}条启动时累积的错误消息",
            errors.len()
        );

        // 等待2秒以确保前端已完全加载，延迟发送错误通知
        if let Some(window) = self.get_main_window() {
            let window_clone = window.clone();
            let errors_clone = errors.clone();

            AsyncHandler::spawn(move || async move {
                tokio::time::sleep(Duration::from_secs(2)).await;

                for error in errors_clone {
                    let _ =
                        window_clone.emit("ducker://notice-message", (error.status, error.message));
                    // 每条消息之间间隔500ms，避免消息堆积
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            });
        }
    }
}

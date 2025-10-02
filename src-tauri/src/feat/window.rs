use crate::{
    core::handle,
    get_app_handle
};

#[cfg(desktop)]
pub fn quit(code: Option<i32>) {
    // log::debug!(target: "app", "启动退出流程");
    println!("启动退出流程");

    // 获取应用句柄并设置退出标志
    let app_handle = get_app_handle!();
    handle::Handle::global().set_is_exiting();

    // 优先关闭窗口，提供立即反馈
    if let Some(window) = handle::Handle::global().get_main_window() {
        let _ = window.hide();
    }

    // 在单独线程中处理资源清理，避免阻塞主线程
    std::thread::spawn(move || {
        app_handle.exit(code.unwrap_or(0));
    });
}

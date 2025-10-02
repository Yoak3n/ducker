/// 数据库操作相关的宏定义
/// 
/// 这些宏简化了常见的数据库锁获取操作，减少重复代码

/// 获取应用程序句柄的宏
/// 
/// # 示例
/// ```rust
/// let app_handle = get_app_handle!();
/// ```
#[macro_export]
macro_rules! get_app_handle {
    () => {
        $crate::core::handle::Handle::global().app_handle().unwrap()
    };
}


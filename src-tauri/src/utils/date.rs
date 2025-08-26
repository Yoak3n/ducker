use super::logging::Type;
use crate::logging;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
pub fn to_datetime(timestamp: i64) -> DateTime<Local> {
    DateTime::from_timestamp(timestamp, 0)
        .unwrap()
        .with_timezone(&Local)
}

pub fn to_datetime_str(timestamp: i64) -> String {
    to_datetime(timestamp).to_string()
}

pub fn str_to_datetime(datetime_str: &str) -> DateTime<Local> {
    // 尝试多种时间格式解析

    // 首先尝试 RFC3339 格式
    if let Ok(datetime) = DateTime::parse_from_rfc3339(datetime_str) {
        return datetime.with_timezone(&Local);
    }

    // 尝试解析 "YYYY-MM-DD HH:MM:SS +HH:MM" 格式
    if let Ok(datetime) = DateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S %z") {
        return datetime.with_timezone(&Local);
    }

    // 尝试解析 "YYYY-MM-DD HH:MM:SS" 格式（假设为本地时间）
    if let Ok(naive_datetime) = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S") {
        return Local.from_local_datetime(&naive_datetime).unwrap();
    }

    // 如果所有格式都失败，记录错误并返回当前时间
    logging!(error, Type::System, true, "时间解析错误: {}", datetime_str);
    Local::now()
}

// pub fn to_timestamp(datetime: DateTime<Local>) -> i64 {
//     datetime.timestamp()
// }

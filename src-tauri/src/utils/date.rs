use super::logging::Type;
use crate::logging;
use chrono::{DateTime, Datelike, Local, NaiveDateTime, TimeZone, Timelike};
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

pub fn next_month(dt: DateTime<Local>) ->DateTime<Local> {
    let target_day = dt.day();
    let target_hour = dt.hour();
    let target_minute = dt.minute();
    let target_second = dt.second();
    
    // 计算下个月的基准日期（1号）
    let next_month_base = if dt.month() == 12 {
        dt.with_year(dt.year() + 1).unwrap().with_month(1).unwrap()
    } else {
        dt.with_month(dt.month() + 1).unwrap()
    }.with_day(1).unwrap();
    
    // 尝试设置目标日期
    if let Some(target_dt) = next_month_base.with_day(target_day) {
        return target_dt
            .with_hour(target_hour).unwrap()
            .with_minute(target_minute).unwrap()
            .with_second(target_second).unwrap()
    } else {
        // 如果目标日期在下个月不存在（如2月29日在非闰年），则返回这个月最后一天
        return next_month_base
            .with_day(next_month_base.num_days_in_month().into()).unwrap()
            .with_hour(target_hour).unwrap()
            .with_minute(target_minute).unwrap()
            .with_second(target_second).unwrap()
    }
}

pub fn calculate_next_period(
    current_period_timestamp: i64,
    interval: u8,
) -> i64 {
    let current_period = to_datetime(current_period_timestamp);
    let next_period = match interval {
        30 => next_month(current_period),
        1 | 7 => {
            to_datetime(current_period_timestamp + interval as i64 * 24 * 3600)
        }
        _ => Local::now(),
    };
    next_period.timestamp()
}

pub fn calculate_next_period_from_now(
    current_period_timestamp: i64,
    interval: u8,
) -> i64 {
    let now = Local::now().timestamp();
    let mut ret = current_period_timestamp;
    // 确保计算出的时间晚于当前时间，哪怕等于也不行
    while now >= ret {
        ret = calculate_next_period(ret, interval);
    }
    ret
}

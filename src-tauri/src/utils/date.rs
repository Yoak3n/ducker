use chrono::{DateTime,Local};

pub fn to_datetime(timestamp: i64) -> DateTime<Local> {
    DateTime::from_timestamp(timestamp, 0).unwrap().with_timezone(&Local)
}


pub fn to_datetime_str(timestamp: i64) -> String {
    to_datetime(timestamp).to_string()
}

pub fn str_to_datetime(datetime_str: &str) -> DateTime<Local> {
    DateTime::parse_from_str(datetime_str,"%Y-%m-%d %H:%M:%S").unwrap().with_timezone(&Local)
}

pub fn to_timestamp(datetime: DateTime<Local>) -> i64 {
    datetime.timestamp()
}




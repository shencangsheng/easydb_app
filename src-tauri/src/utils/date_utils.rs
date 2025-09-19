use chrono::{DateTime, Utc};

pub fn time_difference_from_now(input_time: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(input_time);

    if duration.num_milliseconds() < 1000 {
        format!("{}ms", duration.num_milliseconds())
    } else if duration.num_seconds() < 60 {
        format!("{}s", duration.num_seconds())
    } else if duration.num_minutes() < 60 {
        format!("{}m", duration.num_minutes())
    } else {
        format!("{}h", duration.num_hours())
    }
}
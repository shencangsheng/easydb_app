use chrono::{DateTime, Utc};

pub fn time_difference_from_now(input_time: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(input_time);

    let total_ms = duration.num_milliseconds();
    let mut ms = total_ms;

    let hours = ms / 3_600_000;
    ms %= 3_600_000;

    let minutes = ms / 60_000;
    ms %= 60_000;

    let seconds = ms / 1_000;
    ms %= 1_000;

    let mut parts = Vec::new();
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if minutes > 0 {
        parts.push(format!("{}m", minutes));
    }
    if seconds > 0 {
        parts.push(format!("{}s", seconds));
    }
    if ms > 0 {
        parts.push(format!("{}ms", ms));
    }

    if parts.is_empty() {
        "0ms".to_string()
    } else {
        parts.join(" ")
    }
}
use chrono::{DateTime, Datelike, Month, Utc};

pub fn time_pretty(millis: i64) -> String {
    let time =
        chrono::DateTime::from_timestamp_millis(millis).expect("invalid timestamp");
    pretty(time)
}

fn pretty(time: DateTime<Utc>) -> String {
    format!(
        "{} {}, {}",
        time.day(),
        Month::try_from(u8::try_from(time.month()).unwrap())
            .unwrap()
            .name(),
        time.year()
    )
}

pub fn timeline_time(millis: i64) -> String {
    let time =
        chrono::DateTime::from_timestamp_millis(millis).expect("invalid timestamp");
    let now = Utc::now();
    let duration = now.signed_duration_since(time);
    if duration.num_milliseconds() < 0 {
        return pretty(time);
    }
    let hours = duration.num_hours();
    if hours > 3*24 {
        return pretty(time);
    }
    if hours > 24 {
        return format!("{}d ago", duration.num_days());
    }
    if hours < 1 {
        let mins = duration.num_minutes();
        if mins < 1 {
            let secs = duration.num_seconds();
            return format!("{}s ago", secs);
        }
        return format!("{}m ago", mins);
    }
    format!("{}h ago", hours)
}
use chrono::{DateTime, Utc};

// Convert a file size to a human-readable string
pub fn human_readable_size(size: u64) -> String {
    if size >= 1 << 30 {
        format!("{:.2} GB", size as f64 / (1 << 30) as f64)
    } else if size >= 1 << 20 {
        format!("{:.2} MB", size as f64 / (1 << 20) as f64)
    } else if size >= 1 << 10 {
        format!("{:.2} KB", size as f64 / (1 << 10) as f64)
    } else {
        format!("{} B", size)
    }
}

// Format a SystemTime into a human-readable string
pub fn human_readable_time(time: std::time::SystemTime) -> String {
    let datetime: DateTime<Utc> = time.into();
    datetime.format("%d/%m/%Y %H:%M:%S").to_string()
}

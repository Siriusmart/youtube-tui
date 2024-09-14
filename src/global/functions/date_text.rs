use chrono::{DateTime, Datelike};

/// Convert timestamp into `DD/MM/YYYY`
pub fn date_text(timestamp: u64) -> String {
    let date = DateTime::from_timestamp(timestamp as i64, 0)
        .unwrap()
        .date_naive();
    format!("{}/{}/{}", date.day(), date.month(), date.year())
}

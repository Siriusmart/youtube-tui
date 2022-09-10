use chrono::{Datelike, NaiveDateTime};

// Convert timestamp into DD/MM/YYYY
pub fn date_text(timestamp: u64) -> String {
    let date = NaiveDateTime::from_timestamp(timestamp as i64, 0).date();
    format!("{}/{}/{}", date.day(), date.month(), date.year())
}

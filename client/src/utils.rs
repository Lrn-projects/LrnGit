use std::
    env
;

use chrono::{DateTime, NaiveDateTime, Utc};

pub fn change_wkdir(dir: &str) {
    env::set_current_dir(dir).expect("Failed to change directory");
}

// convert a timestamp to readable datetime
pub fn timestamp_to_datetime(timestamp: i64) -> String {
    // Create a NaiveDateTime from the timestamp
    #[allow(deprecated)]
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);

    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);

    // Format the datetime how you want
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}


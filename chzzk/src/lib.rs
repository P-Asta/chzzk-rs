pub mod chat;
mod client;
mod data;
pub mod error;
pub mod r#macro;

pub use client::ChzzkClient;
pub use data::*;

use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use serde::Deserialize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) struct Response<T> {
    pub code: usize,
    pub message: Option<String>,
    pub content: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChzzkDateTime(SystemTime);

impl<'de> Deserialize<'de> for ChzzkDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let naive_datetime = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
            .expect("Failed to parse datetime");
        let datetime_fixed: DateTime<FixedOffset> = FixedOffset::east_opt(9 * 3600)
            .unwrap()
            .from_local_datetime(&naive_datetime)
            .unwrap();
        let datetime_utc: DateTime<Utc> = DateTime::<Utc>::from(datetime_fixed);

        // Convert DateTime<Utc> to SystemTime
        Ok((UNIX_EPOCH + Duration::from_secs(datetime_utc.timestamp() as u64)).into())
    }
}

impl From<SystemTime> for ChzzkDateTime {
    fn from(value: SystemTime) -> Self {
        ChzzkDateTime(value)
    }
}

impl From<ChzzkDateTime> for SystemTime {
    fn from(value: ChzzkDateTime) -> Self {
        value.0
    }
}

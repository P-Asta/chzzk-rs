pub mod chat;
mod client;
mod data;
pub mod error;
pub mod r#macro;
mod request_builder;

pub use client::ChzzkClient;
pub use data::*;

use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use serde::Deserialize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};


/// Chzzk Datetime type in form of `YYYY-MM-DD HH:mm:ss`
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ChzzkTimestamp(u64);

impl From<SystemTime> for ChzzkTimestamp {
    fn from(value: SystemTime) -> Self {
        ChzzkTimestamp(value.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64)
    }
}

impl From<ChzzkTimestamp> for SystemTime {
    fn from(value: ChzzkTimestamp) -> Self {
        UNIX_EPOCH + Duration::from_millis(value.0)
    }
}

#[cfg(feature = "chzzk_debug")]
macro_rules! debug_println {
    ($($arg:tt)*) => (println!($($arg)*));
}

#[cfg(not(feature = "chzzk_debug"))]
macro_rules! debug_println {
    ($($arg:tt)*) => {};
}

pub(crate) use debug_println;

#[cfg(feature = "chzzk_debug")]
macro_rules! debug_print {
    ($($arg:tt)*) => (print!($($arg)*));
}

#[cfg(not(feature = "chzzk_debug"))]
macro_rules! debug_print {
    ($($arg:tt)*) => {};
}

pub(crate) use debug_print;

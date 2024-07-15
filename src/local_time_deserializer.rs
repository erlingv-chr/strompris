use std::ops::Sub;

use chrono::{DateTime, Duration, FixedOffset, NaiveDateTime};
use serde::{self, Deserialize, Deserializer};

const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%z";

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    // Get the timezone offset by finding the substring following "+"
    let tz_offset_start = s.find('+').unwrap() + 1;
    let tz_offset: i32 = s.get(tz_offset_start..tz_offset_start + 2).unwrap().parse().unwrap();

    let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
    let hour = 3600;
    let tz = FixedOffset::east_opt(tz_offset * hour).unwrap();

    // Subtract the offset because parsing ignores timezone
    let offset_delta = Duration::hours(tz_offset as i64);
    Ok(DateTime::<FixedOffset>::from_naive_utc_and_offset(dt, tz).sub(offset_delta))
}

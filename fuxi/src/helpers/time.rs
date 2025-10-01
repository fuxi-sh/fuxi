use crate::{
    helpers::constants::{FMT_MS, FMT_MS_CPT, OFFSET},
    types::alias::Time,
};
use anyhow::{Result, anyhow, bail};
use chrono::{NaiveDateTime, TimeZone, Utc};
use pyo3::pyfunction;

#[inline]
#[pyfunction]
pub fn millis_to_time(value: i64) -> Result<Time> {
    OFFSET
        .timestamp_millis_opt(value)
        .single()
        .ok_or(anyhow!("无效的时间戳: {value}"))
}

#[inline]
#[pyfunction]
pub fn nanos_to_time(value: i64) -> Time {
    OFFSET.timestamp_nanos(value)
}

#[pyfunction]
pub fn str_to_time(value: &str) -> Result<Time> {
    let len = value.len();
    let naive = if value.contains("-") {
        let ds = match len {
            7 => format!("{value}-01 00:00:00.000"),
            10 => format!("{value} 00:00:00.000"),
            13 => format!("{value}:00:00.000"),
            16 => format!("{value}:00.000"),
            19 => format!("{value}.000"),
            23 => value.to_owned(),
            _ => bail!("无效时间字符串: {value}"),
        };
        NaiveDateTime::parse_from_str(&ds, FMT_MS)?
    } else {
        let ds = match len {
            4 => format!("{value}0101000000000"),
            6 => format!("{value}01000000000"),
            8 => format!("{value}000000000"),
            10 => format!("{value}0000000"),
            12 => format!("{value}00000"),
            14 => format!("{value}000"),
            17 => value.to_owned(),
            _ => bail!("无效时间字符串: {value}"),
        };
        NaiveDateTime::parse_from_str(&ds, FMT_MS_CPT)?
    };

    OFFSET
        .from_local_datetime(&naive)
        .single()
        .ok_or(anyhow!("无效时间字符串: {value}"))
}

#[pyfunction]
pub fn time_to_str(value: Time, fmt: &str) -> String {
    value.format(fmt).to_string()
}

#[allow(dead_code)]
#[inline]
pub fn now() -> Time {
    Utc::now().with_timezone(&OFFSET)
}

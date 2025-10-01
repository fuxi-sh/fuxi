use crate::helpers::constants::TIME_OFFSET;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use indexmap::IndexMap;
use parking_lot::RwLock;
use pyo3::{Py, PyAny, types::PyFunction};
use pyo3_polars::PyDataFrame;
use rust_decimal::Decimal;
use std::sync::Arc;

pub type Price = Decimal;

pub type Size = Decimal;

pub type Time = DateTime<Tz>;

#[inline]
pub fn default_time() -> Time {
    DateTime::<Utc>::default().with_timezone(&TIME_OFFSET)
}

pub type Safe<T> = Arc<RwLock<T>>;

#[inline]
pub fn new_safe<T>(data: T) -> Safe<T> {
    Arc::new(RwLock::new(data))
}

pub type Map<K, V> = IndexMap<K, V>;

pub type SafeMap<K, V> = Safe<Map<K, V>>;

pub type SharedPyObj<T> = Arc<Py<T>>;

pub type SharedPyFunc = SharedPyObj<PyFunction>;

pub type SharedPyParams = SharedPyObj<PyAny>;

pub type SharedPyCandles = SharedPyObj<PyDataFrame>;

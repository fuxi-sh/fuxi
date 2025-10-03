use crate::types::{
    alias::{Time, default_time},
    base::{LogLevel, Mode, Volume},
    market::SymbolMap,
};
use anyhow::Result;
use fuxi_macros::model;
use pyo3::{
    Bound, pymethods,
    types::{PyTuple, PyTupleMethods},
};
use std::fmt::Arguments;

#[model(python)]
pub struct Context {
    pub mode: Mode,
    pub time: Time,
    pub spot: Volume,
    pub swap: Volume,
    pub symbols: SymbolMap,
    log_level: (LogLevel, LogLevel),
}

impl Context {
    pub fn new(mode: Mode, log_level: (LogLevel, LogLevel)) -> Self {
        Self::from(ContextData {
            mode,
            log_level,
            time: default_time(),
            spot: Default::default(),
            swap: Default::default(),
            symbols: Default::default(),
        })
    }

    fn log(&self, engine: bool, level: LogLevel, msg: Arguments) {
        let curr_level = if engine {
            self.log_level().0
        } else {
            self.log_level().1
        };
        if level < curr_level {
            return;
        }

        crate::helpers::log::print(format_args!(
            "{} {}{}{} - {}\n",
            Self::FMT_S,
            match *self.mode() {
                Mode::Backtest => "📊",
                Mode::Sandbox => "🧪",
                Mode::Mainnet => "🚀",
            },
            if engine { "🐲" } else { "🐺" },
            match level {
                LogLevel::Trace => "🔗",
                LogLevel::Debug => "🔍",
                LogLevel::Info => "📝",
                LogLevel::Warn => "🚨",
                LogLevel::Error => "💥",
            },
            msg,
        ));
    }

    #[inline]
    pub fn show_log(&self, level: LogLevel, msg: Arguments) {
        self.log(true, level, msg);
    }
}

#[pymethods]
impl Context {
    #[classattr]
    const FMT_MS: &'static str = crate::helpers::constants::FMT_MS;

    #[classattr]
    const FMT_MS_CPT: &'static str = crate::helpers::constants::FMT_MS_CPT;

    #[classattr]
    const FMT_S: &'static str = crate::helpers::constants::FMT_S;

    #[classattr]
    const FMT_S_CPT: &'static str = crate::helpers::constants::FMT_S_CPT;

    #[pyo3(name = "show_log", signature = (level, *args))]
    fn show_strategy_log(&self, level: LogLevel, args: &Bound<'_, PyTuple>) {
        self.log(
            false,
            level,
            format_args!(
                "{}",
                args.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        );
    }

    #[staticmethod]
    #[pyo3(signature = (millis))]
    fn millis_to_time(millis: i64) -> Result<Time> {
        crate::helpers::time::millis_to_time(millis)
    }

    #[staticmethod]
    #[pyo3(signature = (nanos))]
    fn nanos_to_time(nanos: i64) -> Time {
        crate::helpers::time::nanos_to_time(nanos)
    }

    #[staticmethod]
    #[pyo3(signature = (s))]
    fn str_to_time(s: &str) -> Result<Time> {
        crate::helpers::time::str_to_time(s)
    }

    #[staticmethod]
    #[pyo3(signature = (t, fmt))]
    fn time_to_str(t: Time, fmt: &str) -> String {
        crate::helpers::time::time_to_str(t, fmt)
    }

    #[staticmethod]
    fn new_id() -> String {
        crate::helpers::id::new()
    }
}

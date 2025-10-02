use crate::{
    helpers::constants::{FMT_STY_1, FMT_STY_2},
    runtime::Runtime,
    types::{
        alias::{Time, default_time},
        base::{Codes, LogLevel, Mode, Volume},
        market::SymbolMap,
    },
};
use anyhow::Result;
use fuxi_macros::model;
use pyo3::{
    Bound, PyResult, pymethods,
    types::{PyTuple, PyTupleMethods},
};
use pyo3_polars::PyDataFrame;
use std::fmt::Arguments;

#[model(python, abs)]
pub struct Context {
    runtime: Option<Box<dyn Runtime + Send + Sync + 'static>>,
    log_level: (LogLevel, LogLevel),
    pub time: Time,
    pub spot: Volume,
    pub swap: Volume,
    pub symbols: SymbolMap,
}

impl Context {
    pub fn log(&self, engine: bool, level: LogLevel, msg: Arguments) {
        let curr_level = if engine {
            self.log_level().0
        } else {
            self.log_level().1
        };
        if level < curr_level {
            return;
        }

        let mode = self.runtime().as_ref().unwrap().mode();

        crate::helpers::log::print(format_args!(
            "{} {}{}{} - {}\n",
            self.time().format(match mode {
                Mode::Backtest => FMT_STY_1,
                _ => FMT_STY_2,
            }),
            match mode {
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
    pub fn engine_log(&self, level: LogLevel, msg: Arguments) {
        self.log(true, level, msg);
    }
}

#[pymethods]
impl Context {
    #[new]
    #[pyo3(signature = (log_level=(LogLevel::Info, LogLevel::Info)))]
    fn new(log_level: (LogLevel, LogLevel)) -> Self {
        Self::from(ContextData {
            runtime: None,
            log_level,
            time: default_time(),
            spot: Default::default(),
            swap: Default::default(),
            symbols: Default::default(),
        })
    }
}

#[pymethods]
impl Context {
    #[pyo3(signature = (*args))]
    fn trace(&self, args: &Bound<'_, PyTuple>) {
        self.log(
            false,
            LogLevel::Trace,
            format_args!(
                "{}",
                args.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        );
    }

    #[pyo3(signature = (*args))]
    fn debug(&self, args: &Bound<'_, PyTuple>) {
        self.log(
            false,
            LogLevel::Debug,
            format_args!(
                "{}",
                args.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        );
    }

    #[pyo3(signature = (*args))]
    fn info(&self, args: &Bound<'_, PyTuple>) {
        self.log(
            false,
            LogLevel::Info,
            format_args!(
                "{}",
                args.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        );
    }

    #[pyo3(signature = (*args))]
    fn warn(&self, args: &Bound<'_, PyTuple>) {
        self.log(
            false,
            LogLevel::Warn,
            format_args!(
                "{}",
                args.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        );
    }

    #[pyo3(signature = (*args))]
    fn error(&self, args: &Bound<'_, PyTuple>) {
        self.log(
            false,
            LogLevel::Error,
            format_args!(
                "{}",
                args.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        );
    }
}

#[pymethods]
impl Context {
    fn millis_to_time(&self, millis: i64) -> Result<Time> {
        crate::helpers::time::millis_to_time(millis)
    }

    fn nanos_to_time(&self, nanos: i64) -> Time {
        crate::helpers::time::nanos_to_time(nanos)
    }

    fn str_to_time(&self, s: &str) -> Result<Time> {
        crate::helpers::time::str_to_time(s)
    }

    fn time_to_str(&self, t: Time, fmt: &str) -> String {
        crate::helpers::time::time_to_str(t, fmt)
    }

    fn new_id(&self) -> String {
        crate::helpers::id::new()
    }

    #[getter(FMT_MS)]
    fn fmt_ms(&self) -> &'static str {
        crate::helpers::constants::FMT_MS
    }

    #[getter(FMT_MS_CPT)]
    fn fmt_ms_cpt(&self) -> &'static str {
        crate::helpers::constants::FMT_MS_CPT
    }

    #[getter(FMT_S)]
    fn fmt_s(&self) -> &'static str {
        crate::helpers::constants::FMT_S
    }

    #[getter(FMT_S_CPT)]
    fn fmt_s_cpt(&self) -> &'static str {
        crate::helpers::constants::FMT_S_CPT
    }
}

#[pymethods]
impl Context {
    fn on_start(&self) -> PyResult<()> {
        Ok(())
    }

    fn on_stop(&self) -> PyResult<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn on_history_candle(&self, code: Codes, candles: PyDataFrame) -> PyResult<()> {
        Ok(())
    }

    fn on_tick(&self) -> PyResult<()> {
        Ok(())
    }

    fn on_position(&self) -> PyResult<()> {
        Ok(())
    }

    fn on_order(&self) -> PyResult<()> {
        Ok(())
    }

    fn on_cash(&self) -> PyResult<()> {
        Ok(())
    }
}

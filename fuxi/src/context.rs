use crate::{
    helpers::constants::FMT_S,
    types::{
        alias::{Time, default_time},
        base::{LogLevel, Mode, Volume},
        market::SymbolMap,
    },
};
use anyhow::Result;
use fuxi_macros::model;
use pyo3::{
    Bound, PyResult,
    exceptions::PyNotImplementedError,
    pymethods,
    types::{PyTuple, PyTupleMethods},
};
use std::fmt::Arguments;

#[model(python, abs)]
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
        crate::helpers::log::print(format_args!(
            "{} | {}{}{} | ==> {}\n",
            self.time().format(FMT_S),
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
    pub fn engine_log(&self, level: LogLevel, msg: Arguments) {
        self.log(true, level, msg);
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
    fn launcher(&self) -> PyResult<()> {
        Err(PyNotImplementedError::new_err("子类必须实现`launcher`方法"))
    }

    fn on_start(&self) -> PyResult<()> {
        Ok(())
    }

    fn on_stop(&self) -> PyResult<()> {
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

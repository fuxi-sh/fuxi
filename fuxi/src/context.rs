use crate::{
    runtime::Runtime,
    types::{
        alias::{Price, Size, Time, default_time},
        base::{Codes, Direction, LogLevel, Method, Mode, Side, Volume},
        market::SymbolMap,
        order::Order,
    },
};
use anyhow::Result;
use fuxi_macros::model;
use pyo3::{
    Bound, pymethods,
    types::{PyTuple, PyTupleMethods},
};
use std::{fmt::Arguments, sync::Arc};

#[model(python)]
pub struct Context {
    pub mode: Mode,
    pub time: Time,
    pub spot: Volume,
    pub swap: Volume,
    pub symbols: SymbolMap,
    runtime: Option<Arc<dyn Runtime>>,
    log_level: (LogLevel, LogLevel),
}

impl Default for Context {
    fn default() -> Self {
        Self::from(ContextData {
            mode: Mode::Backtest,
            runtime: None,
            log_level: (LogLevel::Info, LogLevel::Info),
            time: default_time(),
            spot: Default::default(),
            swap: Default::default(),
            symbols: Default::default(),
        })
    }
}

impl Context {
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
            self.time().format(crate::helpers::constants::FMT_S),
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
    #[pyo3(name = "show_log", signature = (level, *args))]
    fn _show_log(&self, level: LogLevel, args: &Bound<'_, PyTuple>) {
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

    #[pyo3(name = "set_log_level", signature = (engine, strategy))]
    fn _set_log_level(&self, engine: LogLevel, strategy: LogLevel) {
        self.set_log_level((engine, strategy));
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

    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (code, method, direction, side, size, price, remark=None))]
    fn place_order(
        &self,
        code: Codes,
        method: Method,
        direction: Direction,
        side: Side,
        size: Size,
        price: Price,
        remark: Option<String>,
    ) -> Result<Order> {
        let runtime = self.runtime().clone().unwrap();
        runtime.place_order(code, method, direction, side, size, price, remark)
    }

    #[pyo3(signature = (code, id))]
    fn cancel_order(&self, code: Codes, id: &str) -> Result<()> {
        let runtime = self.runtime().clone().unwrap();
        runtime.cancel_order(code, id)
    }
}

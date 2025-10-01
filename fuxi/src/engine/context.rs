use crate::{
    engine::traits::SharedRuntime,
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
    Bound, pymethods,
    types::{PyTuple, PyTupleMethods},
};
use std::fmt::Arguments;

#[model(python)]
pub struct Context {
    runtime: SharedRuntime,
    log_level: (LogLevel, LogLevel),
    pub time: Time,
    pub spot: Volume,
    pub swap: Volume,
    pub symbols: SymbolMap,
}

impl Context {
    pub fn run(runtime: SharedRuntime, log_level: (LogLevel, LogLevel)) -> Result<()> {
        let fuxi = Self::from(ContextData {
            runtime: runtime.clone(),
            log_level,
            time: default_time(),
            spot: Default::default(),
            swap: Default::default(),
            symbols: Default::default(),
        });

        runtime.run(fuxi.clone())?;

        Ok(())
    }

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
            "{} | {} | {} | {} | ==> {}\n",
            self.time().format(FMT_S),
            match self.runtime().mode() {
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

use crate::{
    engine::traits::SharedRuntime,
    helpers::constants::FMT_S,
    types::{
        alias::{Time, default_time},
        base::{LogLevel, Mode, SymbolCode, Volume},
        symbol::SymbolMap,
    },
};
use anyhow::{Result, anyhow};
use fuxi_macros::model;
use pyo3::{
    Bound, pymethods,
    types::{PyTuple, PyTupleMethods},
};
use pyo3_polars::PyExpr;
use std::{fmt::Arguments, time::Instant};

#[model(python)]
pub struct Context {
    runtime: SharedRuntime,
    pub time: Time,
    pub spot: Volume,
    pub swap: Volume,
    pub symbols: SymbolMap,
}

impl Context {
    pub fn run(runtime: SharedRuntime) -> Result<()> {
        let start_time = Instant::now();

        let fuxi = Self::from(ContextData {
            runtime: runtime.clone(),
            time: default_time(),
            spot: Default::default(),
            swap: Default::default(),
            symbols: Default::default(),
        });

        runtime.run(fuxi.clone())?;

        fuxi.engine_log(
            LogLevel::Info,
            format_args!(
                "运行时长: {}",
                humantime::format_duration(start_time.elapsed())
            ),
        );

        Ok(())
    }

    pub fn log(&self, engine: bool, level: LogLevel, msg: Arguments) {
        let curr_level = if engine {
            *self.config().log().fuxi_level()
        } else {
            *self.config().log().strategy_level()
        };
        if level < curr_level {
            return;
        }
        crate::helpers::log::print(format_args!(
            "{} | {} | {} | {} | ==> {}\n",
            self.time().format(FMT_S),
            match self.runtime().mode() {
                Mode::Backtest => "回测",
                Mode::Sandbox => "沙盒",
                Mode::Mainnet => "实盘",
            },
            if engine { "伏羲" } else { "策略" },
            match level {
                LogLevel::Trace => "链路",
                LogLevel::Debug => "调试",
                LogLevel::Info => "信息",
                LogLevel::Warn => "警告",
                LogLevel::Error => "错误",
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
    #[pyo3(signature = (code, size=None))]
    fn candles(&self, code: SymbolCode, size: Option<usize>) -> Result<PyCandles> {
        self.runtime().candles(self.clone(), code, size)
    }

    #[pyo3(signature = (code, expr))]
    fn with_candles_column(&self, code: SymbolCode, expr: PyExpr) -> Result<()> {
        self.symbols()
            .maps()
            .get(&code)
            .ok_or(anyhow!("交易对不存在: {code}"))?
            .candles()
            .exprs_mut()
            .push(expr.0);
        Ok(())
    }
}

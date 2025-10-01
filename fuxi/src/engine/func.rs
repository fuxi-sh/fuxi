use crate::{
    engine::{backtest::Backtest, context::Context},
    types::{
        alias::{Size, Time},
        base::{Codes, LogLevel},
    },
};
use pyo3::{Py, Python, pyfunction, types::PyDict};
use rust_decimal::dec;
use std::sync::Arc;

#[allow(clippy::too_many_arguments)]
#[pyfunction]
#[pyo3(signature = (strategy, params, codes, begin, end, spot = dec!(1000), swap = dec!(1000), history_size = 5000, engine_log_level=LogLevel::Info, strategy_log_level=LogLevel::Info))]
pub fn run_backtest(
    py: Python,
    strategy: &str,
    params: Py<PyDict>,
    codes: Vec<Codes>,
    begin: Time,
    end: Time,
    spot: Size,
    swap: Size,
    history_size: usize,
    engine_log_level: LogLevel,
    strategy_log_level: LogLevel,
) {
    crate::helpers::log::init(Some(1024));
    py.allow_threads(|| {
        if let Err(err) = Context::run(
            Arc::new(Backtest {
                begin,
                end,
                spot,
                swap,
                history_size,
            }),
            config,
        ) {
            crate::helpers::log::print(format_args!("运行回测失败: {err}\n"));
        }
    });
    crate::helpers::log::flush().ok();
}

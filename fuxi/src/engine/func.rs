use crate::{
    engine::{backtest::Backtest, config::Config, context::Context},
    types::alias::{Size, Time},
};
use pyo3::{Python, pyfunction};
use rust_decimal::dec;
use std::sync::Arc;

#[allow(clippy::too_many_arguments)]
#[pyfunction]
#[pyo3(signature = (config, begin, end, spot = dec!(1000), swap = dec!(1000), history_size = 5000))]
pub fn run_backtest(
    py: Python,
    config: Config,
    begin: Time,
    end: Time,
    spot: Size,
    swap: Size,
    history_size: usize,
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

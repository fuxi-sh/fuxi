use crate::{
    engine::{strategy::PythonStrategy, traits::SharedStrategy},
    types::{
        alias::{Size, StrategyParams},
        base::{LogLevel, SymbolCode},
    },
};
use anyhow::Result;
use fuxi_macros::model;
use pyo3::{Python, pymethods};
use rust_decimal::dec;

#[model]
pub struct SymbolConfig {
    pub(crate) code: SymbolCode,
    pub(crate) taker: Size,
    pub(crate) maker: Size,
    pub(crate) lever: Size,
}

#[model]
pub struct LogConfig {
    pub(crate) fuxi_level: LogLevel,
    pub(crate) strategy_level: LogLevel,
}

#[model(python)]
pub struct Config {
    pub(crate) strategy: SharedStrategy,
    pub(crate) params: StrategyParams,
    pub(crate) symbols: Vec<SymbolConfig>,
    pub(crate) log: LogConfig,
}

#[pymethods]
impl Config {
    #[new]
    #[pyo3(signature = (strategy, params))]
    fn new(py: Python, strategy: &str, params: StrategyParams) -> Result<Self> {
        let strategy = PythonStrategy::new(py, strategy, true)?;
        Ok(ConfigData {
            strategy,
            params,
            symbols: Default::default(),
            log: LogConfigData {
                fuxi_level: LogLevel::Info,
                strategy_level: LogLevel::Info,
            }
            .into(),
        }
        .into())
    }

    #[pyo3(signature = (level))]
    fn fuxi_log_level(&self, level: LogLevel) -> Self {
        self.log().set_fuxi_level(level);
        self.clone()
    }

    #[pyo3(signature = (level))]
    fn strategy_log_level(&self, level: LogLevel) -> Self {
        self.log().set_strategy_level(level);
        self.clone()
    }

    #[pyo3(signature = (code, taker=dec!(0.005), maker=dec!(0.005), lever=dec!(1)))]
    fn add_symbol(&self, code: SymbolCode, taker: Size, maker: Size, lever: Size) -> Self {
        self.symbols_mut().push(
            SymbolConfigData {
                code,
                taker,
                maker,
                lever,
            }
            .into(),
        );
        self.clone()
    }
}

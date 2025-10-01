use crate::{
    engine::context::Context,
    types::{
        alias::{Price, Size},
        base::{RunMode, SymbolCode, TradeAction, TradeMethod, TradeSide},
    },
};
use anyhow::Result;
use pyo3::{Py, PyAny};
use std::sync::Arc;

pub type SharedRuntime = Arc<dyn Runtime + Send + Sync>;
pub type SharedStrategy = Arc<dyn Strategy + Send + Sync>;

pub trait Runtime: Send + Sync {
    fn mode(&self) -> RunMode;

    fn run(&self, ctx: Context) -> Result<()>;

    #[allow(clippy::too_many_arguments)]
    fn order_place(
        &self,
        ctx: Context,
        code: SymbolCode,
        method: TradeMethod,
        side: TradeSide,
        action: TradeAction,
        size: Size,
        price: Price,
    ) -> Result<String>;

    fn order_cancel(&self, ctx: Context, code: SymbolCode, id: &str) -> Result<()>;

    fn candles(&self, ctx: Context, code: SymbolCode, size: Option<usize>) -> Result<Py<PyAny>>;
}

pub trait Strategy: Send + Sync {
    fn on_init(&self, ctx: Context) -> Result<()>;
    fn on_stop(&self, ctx: Context) -> Result<()>;
    fn on_candle(&self, ctx: Context) -> Result<()>;
    fn on_funding_rate(&self, ctx: Context) -> Result<()>;
    fn on_position(&self, ctx: Context) -> Result<()>;
    fn on_order(&self, ctx: Context) -> Result<()>;
    fn on_cash(&self, ctx: Context) -> Result<()>;
}

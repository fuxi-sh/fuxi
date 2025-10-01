use crate::{
    engine::context::Context,
    types::{
        alias::{Price, Size},
        base::{Codes, Diretion, Method, Mode, Side},
    },
};
use anyhow::Result;
use std::sync::Arc;

pub type SharedRuntime = Arc<dyn Runtime + Send + Sync>;
pub type SharedStrategy = Arc<dyn Strategy + Send + Sync>;

pub trait Runtime: Send + Sync {
    fn mode(&self) -> Mode;

    fn run(&self, context: Context) -> Result<()>;

    #[allow(clippy::too_many_arguments)]
    fn order_place(
        &self,
        context: Context,
        code: Codes,
        method: Method,
        side: Diretion,
        action: Side,
        size: Size,
        price: Price,
    ) -> Result<String>;

    fn order_cancel(&self, context: Context, code: Codes, id: &str) -> Result<()>;
}

pub trait Strategy: Send + Sync {
    fn on_init(&self, context: Context) -> Result<()>;
    fn on_stop(&self, context: Context) -> Result<()>;
    fn on_candle(&self, context: Context) -> Result<()>;
    fn on_funding_rate(&self, context: Context) -> Result<()>;
    fn on_position(&self, context: Context) -> Result<()>;
    fn on_order(&self, context: Context) -> Result<()>;
    fn on_cash(&self, context: Context) -> Result<()>;
}

use crate::{
    engine::context::Context,
    types::{
        alias::{Price, Size},
        base::{Codes, Direction, Method, Mode, Side},
    },
};
use anyhow::Result;
use std::sync::Arc;

pub type SharedRuntime = Arc<dyn Runtime + Send + Sync>;
pub type SharedStrategy = Arc<dyn Strategy + Send + Sync>;

pub trait Runtime: Send + Sync {
    #[allow(clippy::too_many_arguments)]
    fn place_order(
        &self,
        context: Context,
        code: Codes,
        method: Method,
        direction: Direction,
        action: Side,
        size: Size,
        price: Price,
    ) -> Result<String>;

    fn cancel_order(&self, context: Context, code: Codes, id: &str) -> Result<()>;
}

pub trait Strategy: Send + Sync {
    fn on_start(&self, context: Context) -> Result<()>;
    fn on_stop(&self, context: Context) -> Result<()>;
    fn on_tick(&self, context: Context) -> Result<()>;
    fn on_position(&self, context: Context) -> Result<()>;
    fn on_order(&self, context: Context) -> Result<()>;
    fn on_cash(&self, context: Context) -> Result<()>;
}

use crate::types::alias::Size;
use fuxi_macros::{define_coins_with_codes, model};
use polars::{frame::DataFrame, prelude::Expr};

#[model(python)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[model(python)]
pub enum RunMode {
    Backtest,
    Sandbox,
    Mainnet,
}

#[model(python)]
pub enum Market {
    Spot,
    Swap,
}

#[model(python)]
pub enum TradeMethod {
    Market,
    Limit,
}

#[model(python)]
pub enum TradeSide {
    Long,
    Short,
}

#[model(python)]
pub enum TradeAction {
    Buy,
    Sell,
}

#[model(python)]
pub enum OrderStatus {
    New,
    Submitting,
    Rejected,
    Pending,
    Completed,
    Cancelling,
    Cancelled,
}

#[model(python)]
pub enum CandlePeriod {
    Min,
    Min3,
    Min5,
    Min15,
    Min30,
    Hour,
    Hour2,
    Hour4,
    Hour8,
    Hour12,
    Day,
    Day3,
    Week,
    Month,
}

define_coins_with_codes!();

#[model(python)]
pub struct Volume {
    pub total: Size,
    pub avail: Size,
    pub frozen: Size,
}

impl Default for Volume {
    fn default() -> Self {
        VolumeData {
            total: Default::default(),
            avail: Default::default(),
            frozen: Default::default(),
        }
        .into()
    }
}

#[model(python)]
pub struct Pnl {
    pub realized: Size,
    pub unrealized: Size,
}

impl Default for Pnl {
    fn default() -> Self {
        PnlData {
            realized: Default::default(),
            unrealized: Default::default(),
        }
        .into()
    }
}

#[model]
#[derive(Default)]
pub struct MyDataFrame {
    dataframe: DataFrame,
    exprs: Vec<Expr>,
}

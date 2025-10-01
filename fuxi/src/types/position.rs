use crate::types::{
    alias::{Price, Size},
    base::{Pnl, SymbolCode, TradeSide, Volume},
    order::OrderMap,
};
use fuxi_macros::model;

#[model(python)]
pub struct SidePosition {
    pub code: SymbolCode,
    pub side: TradeSide,
    pub size: Volume,
    pub price: Price,
    pub pnl: Pnl,
}

impl SidePosition {
    pub fn new(code: SymbolCode, side: TradeSide) -> Self {
        SidePositionData {
            code,
            side,
            size: Default::default(),
            price: Default::default(),
            pnl: Default::default(),
        }
        .into()
    }
}

#[model(python)]
pub struct Position {
    pub code: SymbolCode,
    pub margin: Volume,
    pub pnl: Pnl,
    pub long: SidePosition,
    pub short: SidePosition,
    pub lever: Size,
    pub orders: OrderMap,
}

impl Position {
    pub fn new(code: SymbolCode, lever: Size) -> Self {
        PositionData {
            code,
            margin: Default::default(),
            pnl: Default::default(),
            long: SidePosition::new(code, TradeSide::Long),
            short: SidePosition::new(code, TradeSide::Short),
            lever,
            orders: Default::default(),
        }
        .into()
    }
}

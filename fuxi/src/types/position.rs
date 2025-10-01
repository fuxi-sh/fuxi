use crate::types::{
    alias::{Price, Size},
    base::{Codes, Pnl, Diretion, Volume},
    order::OrderMap,
};
use fuxi_macros::model;

#[model(python)]
pub struct SidePosition {
    pub code: Codes,
    pub side: Diretion,
    pub size: Volume,
    pub price: Price,
    pub pnl: Pnl,
}

impl SidePosition {
    pub fn new(code: Codes, side: Diretion) -> Self {
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
    pub code: Codes,
    pub margin: Volume,
    pub pnl: Pnl,
    pub long: SidePosition,
    pub short: SidePosition,
    pub lever: Size,
    pub orders: OrderMap,
}

impl Position {
    pub fn new(code: Codes, lever: Size) -> Self {
        PositionData {
            code,
            margin: Default::default(),
            pnl: Default::default(),
            long: SidePosition::new(code, Diretion::Long),
            short: SidePosition::new(code, Diretion::Short),
            lever,
            orders: Default::default(),
        }
        .into()
    }
}

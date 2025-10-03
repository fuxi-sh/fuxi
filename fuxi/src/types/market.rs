use crate::types::{
    alias::{Size, Time, default_time},
    base::Codes,
    position::Position,
};
use fuxi_macros::{define_map, model};

#[model(python)]
pub struct Candle {
    pub code: Codes,
    pub time: Time,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[model(python)]
pub struct FundingRate {
    pub code: Codes,
    pub time: Time,
    pub rate: f64,
    pub next_time: Time,
    pub min: f64,
    pub max: f64,
    pub update_time: Time,
}

impl FundingRate {
    pub fn new(code: Codes) -> Self {
        FundingRateData {
            code,
            time: default_time(),
            rate: Default::default(),
            next_time: default_time(),
            min: Default::default(),
            max: Default::default(),
            update_time: default_time(),
        }
        .into()
    }
}

#[model(python)]
pub struct Symbol {
    pub code: Codes,
    pub taker: Size,
    pub maker: Size,
    pub position: Position,
}

define_map!(pub SymbolMap is Codes to Symbol);

impl Symbol {
    pub fn new(code: Codes, taker: Size, maker: Size, lever: Size) -> Self {
        Self::from(SymbolData {
            code,
            taker,
            maker,
            position: Position::new(code, lever),
        })
    }
}

mod backtest;
mod context;
mod engine;
mod helpers;
mod types;

use crate::{
    backtest::Backtest,
    context::Context,
    types::{
        base::{
            Codes, Coins, Direction, Interval, LogLevel, Market, Method, Mode, OrderStatus, Pnl,
            Side, Volume,
        },
        market::{Candle, FundingRate, Symbol},
        order::Order,
        position::{Position, SidePosition},
    },
};
use pyo3::{
    Bound, PyResult, pymodule,
    types::{PyModule, PyModuleMethods},
};

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<LogLevel>()?;
    m.add_class::<Mode>()?;
    m.add_class::<Market>()?;
    m.add_class::<Method>()?;
    m.add_class::<Direction>()?;
    m.add_class::<Side>()?;
    m.add_class::<OrderStatus>()?;
    m.add_class::<Interval>()?;
    m.add_class::<Coins>()?;
    m.add_class::<Codes>()?;
    m.add_class::<Volume>()?;
    m.add_class::<Pnl>()?;
    m.add_class::<Order>()?;
    m.add_class::<SidePosition>()?;
    m.add_class::<Position>()?;
    m.add_class::<Candle>()?;
    m.add_class::<FundingRate>()?;
    m.add_class::<Symbol>()?;
    m.add_class::<Context>()?;
    m.add_class::<Backtest>()?;
    Ok(())
}

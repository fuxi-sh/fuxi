mod engine;
mod helpers;
mod types;

use crate::{
    helpers::{
        constants::{TIME_FMT_MS, TIME_FMT_MS_CPT, TIME_FMT_S, TIME_FMT_S_CPT},
        id::new as new_id,
        time::{millis_to_time, nanos_to_time, str_to_time, time_to_str},
    },
    types::{
        base::{
            CandlePeriod, Codes, Coins, LogLevel, Market, OrderStatus, Pnl, RunMode, TradeAction,
            TradeMethod, TradeSide, Volume,
        },
        order::Order,
        position::{Position, SidePosition},
        symbol::{FundingRate, Symbol},
    },
};
use pyo3::{
    Bound, PyResult, pymodule,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction,
};

#[pymodule]
fn _sdk(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<LogLevel>()?;
    m.add_class::<RunMode>()?;
    m.add_class::<Market>()?;
    m.add_class::<TradeMethod>()?;
    m.add_class::<TradeSide>()?;
    m.add_class::<TradeAction>()?;
    m.add_class::<OrderStatus>()?;
    m.add_class::<CandlePeriod>()?;
    m.add_class::<Coins>()?;
    m.add_class::<Codes>()?;
    m.add_class::<Volume>()?;
    m.add_class::<Pnl>()?;
    m.add_class::<Order>()?;
    m.add_class::<SidePosition>()?;
    m.add_class::<Position>()?;
    m.add_class::<FundingRate>()?;
    m.add_class::<Symbol>()?;

    m.add("TIME_FMT_MS", TIME_FMT_MS)?;
    m.add("TIME_FMT_MS_CPT", TIME_FMT_MS_CPT)?;
    m.add("TIME_FMT_S", TIME_FMT_S)?;
    m.add("TIME_FMT_S_CPT", TIME_FMT_S_CPT)?;

    m.add_function(wrap_pyfunction!(millis_to_time, m)?)?;
    m.add_function(wrap_pyfunction!(nanos_to_time, m)?)?;
    m.add_function(wrap_pyfunction!(str_to_time, m)?)?;
    m.add_function(wrap_pyfunction!(time_to_str, m)?)?;
    m.add_function(wrap_pyfunction!(new_id, m)?)?;

    Ok(())
}

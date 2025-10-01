mod engine;
mod helpers;
mod types;

use crate::{
    helpers::{
        constants::{FMT_MS, FMT_MS_CPT, FMT_S, FMT_S_CPT},
        id::new as new_id,
        time::{millis_to_time, nanos_to_time, str_to_time, time_to_str},
    },
    types::{
        base::{
            Codes, Coins, Diretion, Interval, LogLevel, Market, Method, Mode, OrderStatus, Pnl,
            Side, Volume,
        },
        order::Order,
        position::{Position, SidePosition},
        symbol::{Candle, FundingRate, Symbol},
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
    m.add_class::<Mode>()?;
    m.add_class::<Market>()?;
    m.add_class::<Method>()?;
    m.add_class::<Diretion>()?;
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

    m.add("FMT_MS", FMT_MS)?;
    m.add("FMT_MS_CPT", FMT_MS_CPT)?;
    m.add("FMT_S", FMT_S)?;
    m.add("FMT_S_CPT", FMT_S_CPT)?;

    m.add_function(wrap_pyfunction!(millis_to_time, m)?)?;
    m.add_function(wrap_pyfunction!(nanos_to_time, m)?)?;
    m.add_function(wrap_pyfunction!(str_to_time, m)?)?;
    m.add_function(wrap_pyfunction!(time_to_str, m)?)?;
    m.add_function(wrap_pyfunction!(new_id, m)?)?;

    Ok(())
}

use crate::{
    context::Context,
    helpers::constants::{FMT_STY_1, FMT_STY_2},
    runtime::Runtime,
    types::{
        alias::{Time, default_time},
        base::{Codes, LogLevel, Mode, Volume},
        market::SymbolMap,
    },
};
use anyhow::Result;
use fuxi_macros::model;
use pyo3::{
    Bound, Py, PyResult, Python, pymethods,
    types::{PyTuple, PyTupleMethods},
};
use pyo3_polars::PyDataFrame;
use std::fmt::Arguments;

#[model(python)]
pub struct DataProvider {
    context: Context,
    pub dataframe: Py<PyDataFrame>,
}

#[pymethods]
impl DataProvider {
    #[new]
    #[pyo3(signature = (context))]
    fn new(py: Python, context: Context) -> Result<Self> {
        todo!()
    }
}

use crate::{
    engine::{context::Context, traits::Strategy},
    types::alias::SharedPyFunc,
};
use anyhow::Result;
use pyo3::{
    Py, Python,
    types::{PyAnyMethods, PyFunction, PyModule},
};
use std::{ffi::CString, sync::Arc};

pub struct PythonStrategy {
    on_init: SharedPyFunc,
    on_stop: SharedPyFunc,
    on_candle: SharedPyFunc,
    on_funding_rate: SharedPyFunc,
    on_position: SharedPyFunc,
    on_order: SharedPyFunc,
    on_cash: SharedPyFunc,
}

impl PythonStrategy {
    pub fn new(py: Python, strategy: &str, is_backtest: bool) -> Result<Arc<Self>> {
        let code = std::fs::read_to_string(strategy)?;
        let module = PyModule::from_code(
            py,
            CString::new(code)?.as_c_str(),
            CString::new(if is_backtest { strategy } else { "strategy.py" })?.as_c_str(),
            CString::new(crate::helpers::id::new())?.as_c_str(),
        )?;
        let on_init = Arc::new(module.getattr("on_init")?.extract::<Py<PyFunction>>()?);
        let on_stop = Arc::new(module.getattr("on_stop")?.extract::<Py<PyFunction>>()?);
        let on_candle = Arc::new(module.getattr("on_candle")?.extract::<Py<PyFunction>>()?);
        let on_funding_rate = Arc::new(
            module
                .getattr("on_funding_rate")?
                .extract::<Py<PyFunction>>()?,
        );
        let on_position = Arc::new(module.getattr("on_position")?.extract::<Py<PyFunction>>()?);
        let on_order = Arc::new(module.getattr("on_order")?.extract::<Py<PyFunction>>()?);
        let on_cash = Arc::new(module.getattr("on_cash")?.extract::<Py<PyFunction>>()?);

        Ok(Arc::new(Self {
            on_init,
            on_stop,
            on_candle,
            on_funding_rate,
            on_position,
            on_order,
            on_cash,
        }))
    }
}

impl Strategy for PythonStrategy {
    #[inline]
    fn on_init(&self, fuxi: Context) -> Result<()> {
        Python::with_gil(|py| self.on_init.call1(py, (fuxi,)))?;
        Ok(())
    }

    #[inline]
    fn on_stop(&self, fuxi: Context) -> Result<()> {
        Python::with_gil(|py| self.on_stop.call1(py, (fuxi,)))?;
        Ok(())
    }

    #[inline]
    fn on_candle(&self, fuxi: Context) -> Result<()> {
        Python::with_gil(|py| self.on_candle.call1(py, (fuxi,)))?;
        Ok(())
    }

    #[inline]
    fn on_funding_rate(&self, fuxi: Context) -> Result<()> {
        Python::with_gil(|py| self.on_funding_rate.call1(py, (fuxi,)))?;
        Ok(())
    }

    #[inline]
    fn on_position(&self, fuxi: Context) -> Result<()> {
        Python::with_gil(|py| self.on_position.call1(py, (fuxi,)))?;
        Ok(())
    }

    #[inline]
    fn on_order(&self, fuxi: Context) -> Result<()> {
        Python::with_gil(|py| self.on_order.call1(py, (fuxi,)))?;
        Ok(())
    }

    #[inline]
    fn on_cash(&self, fuxi: Context) -> Result<()> {
        Python::with_gil(|py| self.on_cash.call1(py, (fuxi,)))?;
        Ok(())
    }
}

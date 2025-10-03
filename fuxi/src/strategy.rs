use crate::{backtest::Backtest, context::Context, types::base::Codes};
use anyhow::Result;
use pyo3::{Bound, Py, PyAny, Python, types::PyAnyMethods};
use pyo3_polars::PyDataFrame;
use std::sync::Arc;

pub struct Strategy {
    on_inject_context: Py<PyAny>,
    on_inject_backtest: Py<PyAny>,
    on_start: Py<PyAny>,
    on_stop: Py<PyAny>,
    on_history_candle: Py<PyAny>,
    on_candle: Py<PyAny>,
    on_timer: Py<PyAny>,
    on_position: Py<PyAny>,
    on_order: Py<PyAny>,
    on_cash: Py<PyAny>,
}

impl Strategy {
    pub fn new(instance: &Bound<PyAny>) -> Result<Arc<Self>> {
        let on_inject_context = instance.getattr("_on_inject_context")?.unbind();
        let on_inject_backtest = instance.getattr("_on_inject_backtest")?.unbind();
        let on_start = instance.getattr("_on_start")?.unbind();
        let on_stop = instance.getattr("_on_stop")?.unbind();
        let on_history_candle = instance.getattr("_on_history_candle")?.unbind();
        let on_candle = instance.getattr("_on_candle")?.unbind();
        let on_timer = instance.getattr("_on_timer")?.unbind();
        let on_position = instance.getattr("_on_position")?.unbind();
        let on_order = instance.getattr("_on_order")?.unbind();
        let on_cash = instance.getattr("_on_cash")?.unbind();

        Ok(Arc::new(Self {
            on_inject_context,
            on_inject_backtest,
            on_start,
            on_stop,
            on_history_candle,
            on_candle,
            on_timer,
            on_position,
            on_order,
            on_cash,
        }))
    }

    #[inline]
    pub fn on_inject_context(&self, context: Context) -> Result<()> {
        Python::with_gil(|py| self.on_inject_context.call1(py, (context,)))?;
        Ok(())
    }

    #[inline]
    pub fn on_inject_backtest(&self, backtest: Backtest) -> Result<()> {
        Python::with_gil(|py| self.on_inject_backtest.call1(py, (backtest,)))?;
        Ok(())
    }

    #[inline]
    pub fn on_start(&self) -> Result<()> {
        Python::with_gil(|py| self.on_start.call0(py))?;
        Ok(())
    }

    #[inline]
    pub fn on_stop(&self) -> Result<()> {
        Python::with_gil(|py| self.on_stop.call0(py))?;
        Ok(())
    }

    #[inline]
    pub fn on_history_candle(&self, code: Codes, candles: PyDataFrame) -> Result<()> {
        Python::with_gil(|py| self.on_history_candle.call1(py, (code, candles)))?;
        Ok(())
    }
    #[inline]
    pub fn on_candle(&self) -> Result<()> {
        Python::with_gil(|py| self.on_candle.call0(py))?;
        Ok(())
    }

    #[inline]
    pub fn on_timer(&self) -> Result<()> {
        Python::with_gil(|py| self.on_timer.call0(py))?;
        Ok(())
    }

    #[inline]
    pub fn on_position(&self) -> Result<()> {
        Python::with_gil(|py| self.on_position.call0(py))?;
        Ok(())
    }

    #[inline]
    pub fn on_order(&self) -> Result<()> {
        Python::with_gil(|py| self.on_order.call0(py))?;
        Ok(())
    }

    #[inline]
    pub fn on_cash(&self) -> Result<()> {
        Python::with_gil(|py| self.on_cash.call0(py))?;
        Ok(())
    }
}

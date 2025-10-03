use crate::{
    context::Context,
    types::{base::Codes, market::Candle},
};
use anyhow::Result;
use pyo3::{Bound, Py, PyAny, Python, types::PyAnyMethods};
use std::sync::Arc;

pub struct Strategy {
    on_inject_context: Py<PyAny>,
    on_start: Py<PyAny>,
    on_stop: Py<PyAny>,
    on_history_tick: Py<PyAny>,
    on_tick: Py<PyAny>,
    on_position: Py<PyAny>,
    on_order: Py<PyAny>,
    on_cash: Py<PyAny>,
}

impl Strategy {
    pub fn new(instance: &Bound<PyAny>) -> Result<Arc<Self>> {
        let on_inject_context = instance.getattr("on_inject_context")?.unbind();
        let on_start = instance.getattr("on_start")?.unbind();
        let on_stop = instance.getattr("on_stop")?.unbind();
        let on_history_tick = instance.getattr("on_history_tick")?.unbind();
        let on_tick = instance.getattr("on_tick")?.unbind();
        let on_position = instance.getattr("on_position")?.unbind();
        let on_order = instance.getattr("on_order")?.unbind();
        let on_cash = instance.getattr("on_cash")?.unbind();

        Ok(Arc::new(Self {
            on_inject_context,
            on_start,
            on_stop,
            on_history_tick,
            on_tick,
            on_position,
            on_order,
            on_cash,
        }))
    }

    #[inline]
    pub fn on_inject_context(&self, context: Context) -> Result<()> {
        Python::attach(|py| self.on_inject_context.call1(py, (context,)))?;
        Ok(())
    }

    #[inline]
    pub fn on_start(&self) -> Result<()> {
        Python::attach(|py| self.on_start.call0(py))?;
        Ok(())
    }

    #[inline]
    pub fn on_stop(&self) -> Result<()> {
        Python::attach(|py| self.on_stop.call0(py))?;
        Ok(())
    }

    #[inline]
    pub fn on_history_tick(&self, code: Codes, candles: Vec<Candle>) -> Result<()> {
        Python::attach(|py| self.on_history_tick.call1(py, (code, candles)))?;
        Ok(())
    }
    #[inline]
    pub fn on_tick(&self) -> Result<()> {
        Python::attach(|py| self.on_tick.call0(py))?;
        Ok(())
    }

    #[inline]
    pub fn on_position(&self) -> Result<()> {
        Python::attach(|py| self.on_position.call0(py))?;
        Ok(())
    }

    #[inline]
    pub fn on_order(&self) -> Result<()> {
        Python::attach(|py| self.on_order.call0(py))?;
        Ok(())
    }

    #[inline]
    pub fn on_cash(&self) -> Result<()> {
        Python::attach(|py| self.on_cash.call0(py))?;
        Ok(())
    }
}

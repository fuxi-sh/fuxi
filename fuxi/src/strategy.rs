use anyhow::Result;
use pyo3::{
    Bound, Py, PyAny, Python,
    types::{PyAnyMethods, PyFunction},
};
use std::sync::Arc;

pub struct Strategy {
    on_start: Py<PyFunction>,
    on_stop: Py<PyFunction>,
    on_tick: Py<PyFunction>,
    on_position: Py<PyFunction>,
    on_order: Py<PyFunction>,
    on_cash: Py<PyFunction>,
}

impl Strategy {
    pub fn new(instance: &Bound<PyAny>) -> Result<Arc<Self>> {
        let on_start = instance.getattr("on_start")?.extract()?;
        let on_stop = instance.getattr("on_stop")?.extract()?;
        let on_tick = instance.getattr("on_tick")?.extract()?;
        let on_position = instance.getattr("on_position")?.extract()?;
        let on_order = instance.getattr("on_order")?.extract()?;
        let on_cash = instance.getattr("on_cash")?.extract()?;

        Ok(Arc::new(Self {
            on_start,
            on_stop,
            on_tick,
            on_position,
            on_order,
            on_cash,
        }))
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

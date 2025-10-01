use crate::engine::{context::Context, traits::Strategy};
use anyhow::Result;
use pyo3::{
    Py, Python,
    types::{PyAnyMethods, PyFunction, PyModule},
};
use std::{ffi::CString, sync::Arc};

pub struct PythonStrategy {
    on_start: Arc<Py<PyFunction>>,
    on_stop: Arc<Py<PyFunction>>,
    on_tick: Arc<Py<PyFunction>>,
    on_position: Arc<Py<PyFunction>>,
    on_order: Arc<Py<PyFunction>>,
    on_cash: Arc<Py<PyFunction>>,
}

impl PythonStrategy {
    pub fn new(py: Python, name: &str, code: &str) -> Result<Arc<Self>> {
        let module = PyModule::from_code(
            py,
            CString::new(code)?.as_c_str(),
            CString::new(name)?.as_c_str(),
            CString::new(crate::helpers::id::new())?.as_c_str(),
        )?;
        let on_start = Arc::new(module.getattr("on_start")?.extract::<Py<PyFunction>>()?);
        let on_stop = Arc::new(module.getattr("on_stop")?.extract::<Py<PyFunction>>()?);
        let on_tick = Arc::new(module.getattr("on_tick")?.extract::<Py<PyFunction>>()?);
        let on_position = Arc::new(module.getattr("on_position")?.extract::<Py<PyFunction>>()?);
        let on_order = Arc::new(module.getattr("on_order")?.extract::<Py<PyFunction>>()?);
        let on_cash = Arc::new(module.getattr("on_cash")?.extract::<Py<PyFunction>>()?);

        Ok(Arc::new(Self {
            on_start,
            on_stop,
            on_tick,
            on_position,
            on_order,
            on_cash,
        }))
    }
}

impl Strategy for PythonStrategy {
    #[inline]
    fn on_start(&self, fuxi: Context) -> Result<()> {
        Python::attach(|py| self.on_start.call1(py, (fuxi,)))?;
        Ok(())
    }

    #[inline]
    fn on_stop(&self, fuxi: Context) -> Result<()> {
        Python::attach(|py| self.on_stop.call1(py, (fuxi,)))?;
        Ok(())
    }

    #[inline]
    fn on_tick(&self, fuxi: Context) -> Result<()> {
        Python::attach(|py| self.on_tick.call1(py, (fuxi,)))?;
        Ok(())
    }

    #[inline]
    fn on_position(&self, fuxi: Context) -> Result<()> {
        Python::attach(|py| self.on_position.call1(py, (fuxi,)))?;
        Ok(())
    }

    #[inline]
    fn on_order(&self, fuxi: Context) -> Result<()> {
        Python::attach(|py| self.on_order.call1(py, (fuxi,)))?;
        Ok(())
    }

    #[inline]
    fn on_cash(&self, fuxi: Context) -> Result<()> {
        Python::attach(|py| self.on_cash.call1(py, (fuxi,)))?;
        Ok(())
    }
}

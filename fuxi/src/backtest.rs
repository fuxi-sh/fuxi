use crate::{
    context::Context,
    runtime::Runtime,
    strategy::Strategy,
    types::{
        alias::{Price, Size, Time},
        base::{Codes, Direction, Method, Mode, Side},
        market::Symbol,
    },
};
use anyhow::{Result, ensure};
use fuxi_macros::model;
use pyo3::{Bound, PyAny, pymethods, types::PyAnyMethods};
use rust_decimal::{Decimal, dec};
use std::sync::Arc;

#[model(python)]
pub struct Backtest {
    context: Context,
    strategy: Arc<Strategy>,
    begin: Time,
    end: Time,
    history_size: usize,
}

#[pymethods]
impl Backtest {
    #[new]
    #[pyo3(signature = (strategy, begin, end, symbols, spot = dec!(1000), swap = dec!(1000), history_size=5000))]
    fn new(
        strategy: &Bound<PyAny>,
        begin: &str,
        end: &str,
        symbols: Vec<(Codes, Size, Size, Size)>,
        spot: Size,
        swap: Size,
        history_size: usize,
    ) -> Result<Self> {
        ensure!(
            strategy.is_instance_of::<Context>(),
            "策略必须继承自`Context`"
        );
        let context = strategy.extract::<Context>()?;

        let strategy = Strategy::new(strategy)?;

        for (code, taker, maker, lever) in symbols {
            ensure!(
                !context.symbols().maps().contains_key(&code),
                "重复交易对: {code}",
            );
            ensure!(
                lever.fract().is_zero(),
                "杠杆倍率不能有小数: 交易对={code}, 杠杆倍率={lever}",
            );

            context
                .symbols()
                .maps_mut()
                .insert(code, Symbol::new(code, taker, maker, lever));
        }

        ensure!(
            !(spot.is_zero() && swap.is_zero()),
            "现货&合约的资金不能同时为空"
        );
        ensure!(spot >= Decimal::ZERO, "现货资金不能小于0");
        ensure!(swap >= Decimal::ZERO, "合约资金不能小于0");

        context.spot().set_total(spot);
        context.spot().set_avail(spot);
        context.swap().set_total(swap);
        context.swap().set_avail(swap);

        let begin = crate::helpers::time::str_to_time(begin)?;
        let end = crate::helpers::time::str_to_time(end)?;
        ensure!(begin < end, "开始时间不能大于结束时间: {begin} - {end}");

        context.set_time(begin);

        let backtest = Backtest::from(BacktestData {
            context: context.clone(),
            strategy,
            begin,
            end,
            history_size,
        });

        context.set_runtime(Some(Box::new(backtest.clone())));

        Ok(backtest)
    }

    #[pyo3(name = "run")]
    fn _run(&self) -> Result<()> {
        crate::helpers::log::init(Some(1024));
        self.run()?;
        crate::helpers::log::flush()?;
        Ok(())
    }
}

impl Runtime for Backtest {
    #[inline]
    fn mode(&self) -> Mode {
        Mode::Backtest
    }

    fn run(&self) -> Result<()> {
        todo!()
    }

    fn place_order(
        &self,
        code: Codes,
        method: Method,
        direction: Direction,
        side: Side,
        size: Size,
        price: Price,
    ) -> Result<String> {
        todo!()
    }

    fn cancel_order(&self, code: Codes, id: &str) -> Result<()> {
        todo!()
    }
}

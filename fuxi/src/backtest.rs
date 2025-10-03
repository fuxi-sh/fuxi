use crate::{
    context::Context,
    history_data,
    runtime::Runtime,
    strategy::Strategy,
    types::{
        alias::{Price, Size, Time},
        base::{Codes, Direction, LogLevel, Market, Method, Side},
        market::Symbol,
    },
};
use anyhow::{Result, ensure};
use chrono::Duration;
use fuxi_macros::model;
use pyo3::{Bound, PyAny, pymethods};
use pyo3_polars::PyDataFrame;
use rust_decimal::{Decimal, dec};
use std::{sync::Arc, time::Instant};

#[model(python)]
pub struct Backtest {
    context: Context,
    strategy: Arc<Strategy>,
    begin: Time,
    end: Time,
    history_size: usize,
    force_sync_data: bool,
}

#[pymethods]
impl Backtest {
    #[allow(clippy::too_many_arguments)]
    #[new]
    #[pyo3(signature = (strategy, begin, end, symbols, spot = dec!(1000), swap = dec!(1000), history_size=5000, force_sync_data=false))]
    fn new(
        strategy: &Bound<PyAny>,
        begin: &str,
        end: &str,
        symbols: Vec<(Codes, Size, Size, Size)>,
        spot: Size,
        swap: Size,
        history_size: usize,
        force_sync_data: bool,
    ) -> Result<Self> {
        let strategy = Strategy::new(strategy)?;

        let context = Context::default();

        let begin = crate::helpers::time::str_to_time(begin)?;
        let end = crate::helpers::time::str_to_time(end)?;
        ensure!(begin < end, "开始时间不能大于结束时间: {begin} - {end}");
        context.set_time(begin);

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

        ensure!(history_size > 0, "历史数据长度错误: {history_size}");

        let backtest = Backtest::from(BacktestData {
            context: context.clone(),
            strategy: strategy.clone(),
            begin,
            end,
            history_size,
            force_sync_data,
        });

        backtest
            .context()
            .set_runtime(Some(Box::new(backtest.clone())));

        strategy.on_inject_context(context.clone())?;

        Ok(backtest)
    }

    fn launche(&self) -> Result<()> {
        crate::helpers::log::init(Some(1024));
        self.run()?;
        crate::helpers::log::flush()?;
        Ok(())
    }
}

impl Runtime for Backtest {
    fn run(&self) -> Result<()> {
        let strategy = self.strategy().clone();

        strategy.on_start()?;

        let codes = self
            .context()
            .symbols()
            .maps()
            .keys()
            .cloned()
            .collect::<Vec<_>>();

        history_data::download(self.context().clone(), &codes, *self.force_sync_data())?;

        self.load_candles(&codes)?;

        let mut now = *self.begin();
        let end = *self.end();

        while now < end {
            self.context().set_time(now);
            now += Duration::minutes(1);
        }

        Ok(())
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

impl Backtest {
    fn load_candles(&self, codes: &[Codes]) -> Result<()> {
        use polars::prelude::*;

        let dir = std::env::current_dir()?.join("data");
        let spot_dir = dir.join("spot");
        let swap_dir = dir.join("swap");

        let strategy = self.strategy().clone();

        for code in codes {
            let start_time = Instant::now();

            let file_path = match code.market() {
                Market::Spot => spot_dir.join(format!(
                    "{}.feather",
                    code.code().replace("/", "_").replace(":", "_")
                )),
                Market::Swap => swap_dir.join(format!(
                    "{}.feather",
                    code.code().replace("/", "_").replace(":", "_")
                )),
            };

            let mut df = LazyFrame::scan_ipc(
                PlPathRef::from_local_path(file_path.as_path()).into_owned(),
                Default::default(),
            )?
            .collect()?;

            if df.should_rechunk() {
                df.rechunk_mut();
            }

            let elapsed = start_time.elapsed();

            self.context().show_log(
                LogLevel::Debug,
                format_args!(
                    "加载数据完成 交易对: {code}, 耗时: {}, 数据: {df}",
                    humantime::format_duration(elapsed),
                ),
            );

            strategy.on_history_candle(*code, PyDataFrame(df.clone()))?;

            self.context()
                .symbols()
                .maps()
                .get(code)
                .unwrap()
                .set_candles(df);
        }

        Ok(())
    }
}

use crate::{
    context::Context,
    history_data,
    runtime::Runtime,
    strategy::Strategy,
    types::{
        alias::{Price, Size, Time},
        base::{Codes, Direction, LogLevel, Market, Method, Mode, Side},
        market::Symbol,
    },
};
use anyhow::{Result, ensure};
use fuxi_macros::model;
use pyo3::{Bound, PyAny, pymethods, types::PyAnyMethods};
use pyo3_polars::PyDataFrame;
use rust_decimal::{Decimal, dec};
use std::sync::Arc;

#[model(python)]
pub struct Backtest {
    context: Context,
    strategy: Arc<Strategy>,
    begin: Time,
    end: Time,
    history_size: usize,
    sync_data: bool,
}

#[pymethods]
impl Backtest {
    #[allow(clippy::too_many_arguments)]
    #[new]
    #[pyo3(signature = (strategy, begin, end, symbols, spot = dec!(1000), swap = dec!(1000), history_size=5000, sync_data=false))]
    fn new(
        strategy: &Bound<PyAny>,
        begin: &str,
        end: &str,
        symbols: Vec<(Codes, Size, Size, Size)>,
        spot: Size,
        swap: Size,
        history_size: usize,
        sync_data: bool,
    ) -> Result<Self> {
        ensure!(
            strategy.is_instance_of::<Context>(),
            "策略必须继承自: Context"
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
                "杠杆倍率不能有小数 交易对: {code}, 杠杆倍率: {lever}",
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

        let begin = crate::helpers::time::str_to_time(begin)?;
        let end = crate::helpers::time::str_to_time(end)?;
        ensure!(
            begin < end,
            "开始时间不能大于结束时间: 开始时间: {begin}, 结束时间: {end}"
        );

        context.set_time(begin);

        let backtest = Backtest::from(BacktestData {
            context: context.clone(),
            strategy,
            begin,
            end,
            history_size,
            sync_data,
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
        let codes = self
            .context()
            .symbols()
            .maps()
            .keys()
            .cloned()
            .collect::<Vec<_>>();

        history_data::download(self.context().clone(), &codes, *self.sync_data())?;

        self.load_data(&codes)?;

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
    fn load_data(&self, codes: &[Codes]) -> Result<()> {
        use polars::prelude::*;

        let dir = std::env::current_dir()?.join("data");
        let spot_dir = dir.join("spot");
        let swap_dir = dir.join("swap");

        let time_range = date_range(
            "time".into(),
            (*self.begin() - chrono::Duration::minutes(*self.history_size() as i64)).naive_utc(),
            self.end().naive_utc(),
            Duration::parse("1m"),
            ClosedWindow::Both,
            TimeUnit::Nanoseconds,
            Some(&chrono_tz::Asia::Shanghai),
        )?
        .into_column();
        let time_range = DataFrame::new(vec![time_range])?.lazy();

        let strategy = self.strategy().clone();

        for code in codes {
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

            let df = LazyFrame::scan_ipc(
                PlPathRef::from_local_path(file_path.as_path()).into_owned(),
                Default::default(),
            )?;

            let mut df = time_range
                .clone()
                .left_join(df, col("time"), col("time"))
                .collect()?;

            if df.should_rechunk() {
                df.rechunk_mut();
            }

            self.context().engine_log(
                LogLevel::Debug,
                format_args!("加载数据完成 交易对: {code}, {df}"),
            );

            let history_df = df.slice(
                (*self.context().time() - *self.begin()).num_minutes(),
                *self.history_size(),
            );

            strategy.on_history_candle(*code, PyDataFrame(history_df))?;

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

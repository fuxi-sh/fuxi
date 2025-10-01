use crate::{
    engine::{context::Context, traits::Runtime},
    types::{
        alias::{Price, Size, Time},
        base::{Diretion, LogLevel, Method, Mode, Side, SymbolCode},
    },
};
use anyhow::{Result, anyhow, ensure};
use chrono::{Duration, DurationRound};
use num_format::ToFormattedString;
use polars::{
    df,
    frame::DataFrame,
    prelude::{IntoColumn, IntoLazy, LazyFrame, PlPathRef, TimeUnit, TimeZone, col, lit},
    time::{ClosedWindow, date_range},
};
use pyo3::{Py, PyAny};
use pyo3_polars::PyDataFrame;
use rust_decimal::dec;
use std::time::Instant;

pub struct Backtest {
    pub begin: Time,
    pub end: Time,
    pub spot: Size,
    pub swap: Size,
    pub history_size: usize,
}

impl Runtime for Backtest {
    #[inline]
    fn mode(&self) -> Mode {
        Mode::Backtest
    }

    fn run(&self, fuxi: Context) -> Result<()> {
        let start_time = Instant::now();

        ensure!(
            self.history_size > 0,
            "历史数据长度错误: {}",
            self.history_size
        );

        let begin = self.begin.duration_trunc(Duration::minutes(1))?;
        let end = self.end.duration_trunc(Duration::minutes(1))?;
        ensure!(begin < end, "时间错误: 开始时间={begin}, 结束时间={end}");

        ensure!(self.spot >= dec!(0), "现货资金错误: {}", self.spot);
        ensure!(self.swap >= dec!(0), "合约资金错误: {}", self.swap);
        fuxi.spot().set_total(self.spot);
        fuxi.spot().set_avail(self.spot);
        fuxi.swap().set_total(self.swap);
        fuxi.swap().set_avail(self.swap);

        fuxi.set_time(begin);

        let strategy = fuxi.config().strategy().clone();
        let codes = fuxi
            .config()
            .symbols()
            .iter()
            .map(|s| *s.code())
            .collect::<Vec<_>>();

        fuxi.engine_log(LogLevel::Debug, format_args!("初始化策略事件"));
        strategy.on_init(fuxi.clone())?;

        let curr_dir = std::env::current_dir()?;
        let filter_time = date_range(
            "time".into(),
            (begin - Duration::minutes(self.history_size as i64)).naive_utc(),
            end.naive_utc(),
            polars::prelude::Duration::parse("1m"),
            ClosedWindow::Both,
            TimeUnit::Nanoseconds,
            Some(&chrono_tz::Asia::Shanghai),
        )?
        .into_column();
        let time_range = DataFrame::new(vec![filter_time])?.lazy();

        for code in &codes {
            fuxi.engine_log(LogLevel::Debug, format_args!("{code}: k线加载中...",));

            let candle_start_time = Instant::now();
            let path = curr_dir.join("data").join(format!(
                "{}.feather",
                code.code().replace("/", "_").replace(":", "_"),
            ));

            let mut df = LazyFrame::scan_ipc(
                PlPathRef::from_local_path(path.as_path()).into_owned(),
                Default::default(),
            )?;
            df = df.rename(["date"], ["time"], true);
            df = df.with_column(
                col("time")
                    .dt()
                    .convert_time_zone(TimeZone::from_chrono(&chrono_tz::Asia::Shanghai)),
            );
            df = df.with_column(lit(true).alias("finished"));

            df = df.with_columns(
                fuxi.symbols()
                    .maps()
                    .get(code)
                    .unwrap()
                    .candles()
                    .exprs()
                    .clone(),
            );

            let mut df = time_range
                .clone()
                .left_join(df, col("time"), col("time"))
                .collect()?;

            if df.should_rechunk() {
                fuxi.engine_log(LogLevel::Debug, format_args!("{code}: 合并k线数据"));
                df.rechunk_mut();
            }

            fuxi.engine_log(
                LogLevel::Debug,
                format_args!(
                    "{code}: k线加载完成, 数量({}), 耗时: {}",
                    df.height().to_formatted_string(&num_format::Locale::en),
                    humantime::format_duration(candle_start_time.elapsed())
                ),
            );

            fuxi.symbols()
                .maps_mut()
                .get_mut(code)
                .unwrap()
                .candles()
                .set_dataframe(df);
        }

        fuxi.engine_log(LogLevel::Debug, format_args!("开始回放行情"));

        let mut now = begin;

        while now <= end {
            fuxi.set_time(now);

            // for code in &codes {
            //     let symbol = fuxi.symbols().maps().get(code).unwrap().clone();
            //     let position = symbol.position();
            //     let orders = position.orders();
            //     let df = symbol
            //         .candles()
            //         .dataframe()
            //         .slice((now - begin).num_minutes() + self.history_size as i64, 1);

            //     let open = match df.column("open")?.f64()?.get(0) {
            //         Some(open) => open,
            //         None => continue,
            //     };

            //     for order in orders.maps_mut().values_mut() {
            //         match *order.status() {
            //             OrderStatus::New => {
            //                 order.set_status(OrderStatus::Submitting);
            //             }
            //             OrderStatus::Submitting | OrderStatus::Pending => {}
            //             OrderStatus::Cancelling => {}
            //             OrderStatus::Rejected | OrderStatus::Completed | OrderStatus::Cancelled => {
            //             }
            //         }
            //     }
            // }

            fuxi.engine_log(LogLevel::Trace, format_args!("k线更新事件"));
            strategy.on_candle(fuxi.clone())?;

            now += Duration::minutes(1);
        }

        fuxi.engine_log(LogLevel::Debug, format_args!("停止策略事件"));
        strategy.on_stop(fuxi.clone())?;

        let elapsed = humantime::format_duration(start_time.elapsed()).to_string();

        let report = df![
            "名称"=>["回测耗时"],
            "值"=>[elapsed],
        ]?;

        fuxi.engine_log(LogLevel::Info, format_args!("回测报告: {report}"));

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn order_place(
        &self,
        fuxi: Context,
        code: SymbolCode,
        method: Method,
        side: Diretion,
        action: Side,
        size: Size,
        price: Price,
    ) -> Result<String> {
        todo!()
    }

    fn order_cancel(&self, fuxi: Context, code: SymbolCode, id: &str) -> Result<()> {
        todo!()
    }

    fn candles(&self, fuxi: Context, code: SymbolCode, size: Option<usize>) -> Result<Py<PyAny>> {
        let df = fuxi
            .symbols()
            .maps()
            .get(&code)
            .ok_or(anyhow!("交易对不存在: {code}"))?
            .candles()
            .dataframe()
            .slice((*fuxi.time() - self.begin).num_minutes(), self.history_size);
        let df = match size {
            Some(size) => df.tail(Some(size)),
            None => df,
        };
        Ok(PyDataFrame(df))
    }
}

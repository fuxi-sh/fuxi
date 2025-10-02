use crate::{
    context::Context,
    types::{
        alias::{Size, Time},
        base::{Codes, LogLevel, Mode},
        market::Symbol,
    },
};
use anyhow::{Result, ensure};
use fuxi_macros::model;
use pyo3::pymethods;
use rust_decimal::{Decimal, dec};

#[model(python, abs, ext=Context)]
pub struct Backtest {
    begin: Time,
    end: Time,
    history_size: usize,
    context: Context,
}

#[pymethods]
impl Backtest {
    #[new]
    #[pyo3(signature = (begin, end, symbols, spot = dec!(1000), swap = dec!(1000), history_size=5000, log_level=(LogLevel::Info, LogLevel::Info)))]
    fn new(
        begin: &str,
        end: &str,
        symbols: Vec<(Codes, Size, Size, Size)>,
        spot: Size,
        swap: Size,
        history_size: usize,
        log_level: (LogLevel, LogLevel),
    ) -> Result<(Self, Context)> {
        crate::helpers::log::init(Some(1024));
        let context = Context::new(Mode::Backtest, log_level);

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
            begin,
            end,
            history_size,
            context: context.clone(),
        });
        Ok((backtest, context))
    }
}

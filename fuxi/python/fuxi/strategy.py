from abc import ABC
from typing import Dict
from pandas import DataFrame
from ._core import Context, Codes, Mode, Volume, Symbol, LogLevel, Backtest
from .indicator import Indicator
import polars as pl
from polars import DataFrame
from datetime import datetime, timedelta


class Strategy(ABC):
    def __init__(self):
        self._candles = {}
        self._indicators = {}

    def _on_inject_context(self, context: Context):
        self._context = context

    def _on_inject_backtest(self, backtest: Backtest):
        self._backtest = backtest

    def _on_start(self):
        pass

    def _on_stop(self):
        pass

    def _on_history_candle(self, code: Codes, candles: DataFrame):
        if self.mode == Mode.Backtest:
            self._candles[code] = (
                pl.select(
                    pl.datetime_range(
                        self._backtest.begin - timedelta(minutes=self._backtest.history_size),
                        self._backtest.end,
                        interval="1m",
                        closed="both",
                        time_unit="ns",
                        time_zone="Asia/Shanghai",
                    ).alias("time")
                )
                .lazy()
                .join(candles.lazy(), on="time", how="left")
                .collect()
            ).rechunk()
        else:
            self._candles[code] = candles.rechunk()
        for key in self._indicators:
            self._indicators[key]._on_candles(self, self._candles[code])

    def _on_candle(self, code: Codes, candles: DataFrame):
        self._candles[code] = (
            pl.concat(
                [self._candles[code], candles],
                how="horizontal",
            )
            .unique(
                subset=["time"],
                keep="last",
                maintain_order=True,
            )
            .rechunk()
        )
        for key in self._indicators:
            self._indicators[key]._on_candles(self, self._candles[code])

    def _on_timer(self):
        pass

    def _on_position(self):
        pass

    def _on_order(self):
        pass

    def _on_cash(self):
        pass

    @property
    def mode(self) -> Mode:
        return self._context.mode

    @property
    def time(self) -> datetime:
        return self._context.time

    @property
    def spot(self) -> Volume:
        return self._context.spot

    @property
    def swap(self) -> Volume:
        return self._context.swap

    @property
    def symbols(self) -> Dict[Codes, Symbol]:
        return self._context.symbols

    def trace_log(self, *args):
        self._context.show_log(LogLevel.Trace, *args)

    def debug_log(self, *args):
        self._context.show_log(LogLevel.Debug, *args)

    def info_log(self, *args):
        self._context.show_log(LogLevel.Info, *args)

    def warn_log(self, *args):
        self._context.show_log(LogLevel.Warn, *args)

    def error_log(self, *args):
        self._context.show_log(LogLevel.Error, *args)

    @staticmethod
    def millis_to_time(millis: int) -> datetime:
        return Context.millis_to_time(millis)

    @staticmethod
    def nanos_to_time(nanos: int) -> datetime:
        return Context.nanos_to_time(nanos)

    @staticmethod
    def str_to_time(s: str) -> datetime:
        return Context.str_to_time(s)

    @staticmethod
    def time_to_str(t: datetime, fmt: str) -> str:
        return Context.time_to_str(t, fmt)

    @staticmethod
    def new_id() -> str:
        return Context.new_id()

    def set_log_level(self, engine: LogLevel, strategy: LogLevel):
        self._context.set_log_level(engine, strategy)

    def add_indicator(self, name: str, indicator: Indicator):
        self._indicators[name] = indicator

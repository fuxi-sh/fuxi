from abc import ABC, abstractmethod
from typing import Dict, Optional
from ._core import Context, Codes, Mode, Volume, Symbol, LogLevel
import polars as pl
from datetime import datetime


class AbsStrategy(ABC):
    _context: Context

    @abstractmethod
    def on_start(self):
        pass

    @abstractmethod
    def on_stop(self):
        pass

    @abstractmethod
    def on_history_candle(self, code: Codes, candles: pl.DataFrame):
        pass

    @abstractmethod
    def on_candle(self, code: Codes, candles: pl.DataFrame):
        pass

    @abstractmethod
    def on_timer(self):
        pass

    @abstractmethod
    def on_position(self):
        pass

    @abstractmethod
    def on_order(self):
        pass

    @abstractmethod
    def on_cash(self):
        pass

    def _on_inject_context(self, context: Context):
        self._context = context

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


class Strategy(AbsStrategy):
    _candles: Dict[Codes, pl.DataFrame]

    def __init__(self):
        self._candles = {}

    def on_start(self):
        pass

    def on_stop(self):
        pass

    def on_history_candle(self, code: Codes, candles: pl.DataFrame):
        self._candles[code] = candles.rechunk()

    def on_candle(self, code: Codes, candles: pl.DataFrame):
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

    def on_timer(self):
        pass

    def on_position(self):
        pass

    def on_order(self):
        pass

    def on_cash(self):
        pass

    def get_candles(self, code: Codes) -> pl.DataFrame:
        if self.mode == Mode.Backtest:
            return self._candles[code].filter(pl.col("time") < self.time)
        else:
            return self._candles[code]

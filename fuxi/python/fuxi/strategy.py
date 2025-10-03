from abc import ABC, abstractmethod
from typing import Dict
from ._core import Context, Codes, Mode, Volume, Symbol, LogLevel
from polars import DataFrame
from datetime import datetime


class Strategy(ABC):
    _context: Context

    def on_inject_context(self, context: Context):
        self._context = context

    @abstractmethod
    def on_start(self):
        pass

    @abstractmethod
    def on_stop(self):
        pass

    @abstractmethod
    def on_history_candle(self, code: Codes, candles: DataFrame):
        pass

    @abstractmethod
    def on_candle(self, code: Codes, candles: DataFrame):
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

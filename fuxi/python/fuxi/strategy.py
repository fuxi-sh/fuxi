from abc import ABC, abstractmethod
from typing import Dict
from pandas import DataFrame
from ._core import Context, Codes, Mode, Volume, Symbol, LogLevel, Backtest
from .indicator import Indicator
import polars as pl
from polars import DataFrame
from datetime import datetime


class Strategy(ABC):
    # ================================================================ #
    # 属性
    # ================================================================ #

    @property
    def mode(self) -> Mode:
        """模式"""
        return self._context.mode

    @property
    def time(self) -> datetime:
        """当前时间"""
        return self._context.time

    @property
    def spot(self) -> Volume:
        """现货资金"""
        return self._context.spot

    @property
    def swap(self) -> Volume:
        """合约资金"""
        return self._context.swap

    @property
    def symbols(self) -> Dict[Codes, Symbol]:
        """交易对"""
        return self._context.symbols

    # ================================================================ #
    # API
    # ================================================================ #

    def trace_log(self, *args):
        """显示链路日志"""
        self._context.show_log(LogLevel.Trace, *args)

    def debug_log(self, *args):
        """显示调试日志"""
        self._context.show_log(LogLevel.Debug, *args)

    def info_log(self, *args):
        """显示信息日志"""
        self._context.show_log(LogLevel.Info, *args)

    def warn_log(self, *args):
        """显示警告日志"""
        self._context.show_log(LogLevel.Warn, *args)

    def error_log(self, *args):
        """显示错误日志"""
        self._context.show_log(LogLevel.Error, *args)

    @staticmethod
    def millis_to_time(millis: int) -> datetime:
        """毫秒转换为时间"""
        return Context.millis_to_time(millis)

    @staticmethod
    def nanos_to_time(nanos: int) -> datetime:
        """纳秒转换为时间"""
        return Context.nanos_to_time(nanos)

    @staticmethod
    def str_to_time(s: str) -> datetime:
        """
        字符串转换为时间

        格式如下:
        - 2020
        - 2020-01
        - 2020-01-02
        - 2020-01-02 03
        - 2020-01-02 03:04
        - 2020-01-02 03:04:05
        - 2020-01-02 03:04:05.678
        - 2020
        - 202001
        - 20200102
        - 2020010203
        - 202001020304
        - 20200102030405
        - 20200102030405678
        """
        return Context.str_to_time(s)

    @staticmethod
    def time_to_str(t: datetime, fmt: str) -> str:
        """
        时间转换为字符串
        [`fmt`]: 格式如下
        - %Y: 年
        - %m: 月
        - %d: 日
        - %H: 时
        - %M: 分
        - %S: 秒
        - %3f: 毫秒
        """
        return Context.time_to_str(t, fmt)

    @staticmethod
    def new_id() -> str:
        """生成唯一id"""
        return Context.new_id()

    def set_log_level(self, engine: LogLevel, strategy: LogLevel):
        """
        设置日志级别
        - [`engine`]: 引擎日志级别
        - [`strategy`]: 策略日志级别
        """
        self._context.set_log_level(engine, strategy)

    def add_candle_indicator(self, code: Codes, indicator: Indicator):
        """
        添加K线指标
        - [`code`]: 交易对
        - [`indicator`]: 指标
        """
        if code not in self._candle_indicators:
            self._candle_indicators[code] = {}
        self._candle_indicators[code][indicator.name] = indicator

    def get_candle_indicator(self, code: Codes, name: str) -> DataFrame:
        """
        获取K线指标
        - [`code`]: 交易对
        - [`name`]: 指标名称
        """
        if code not in self._candle_indicators:
            return None
        if name not in self._candle_indicators[code]:
            return None
        df = self._candle_indicators[code][name]._indicator
        if self.mode == Mode.Backtest:
            return df.slice(0, self._backtest.offset)
        else:
            return df

    def get_candle(self, code: Codes) -> DataFrame:
        """
        获取K线
        - [`code`]: 交易对
        """
        if code not in self._candles:
            return None
        if self.mode == Mode.Backtest:
            return self._candles[code].slice(0, self._backtest.offset)
        else:
            return self._candles[code]

    # ================================================================ #
    # 事件
    # ================================================================ #
    @abstractmethod
    def on_start(self):
        """启动事件"""

    @abstractmethod
    def on_stop(self):
        """停止事件"""

    @abstractmethod
    def on_timer(self):
        """定时器事件"""

    # ================================================================ #
    # 内部
    # ================================================================ #

    _context: Context
    _backtest: Backtest
    _candles: Dict[Codes, DataFrame]
    _candle_indicators: Dict[Codes, Dict[str, Indicator]]

    def __init__(self):
        self._candles = {}
        self._candle_indicators = {}

    def _on_inject_context(self, context: Context):
        self._context = context

    def _on_inject_backtest(self, backtest: Backtest):
        self._backtest = backtest

    def _on_start(self):
        self.on_start()

    def _on_stop(self):
        self.on_stop()

    def _on_history_candle(self, code: Codes, candles: DataFrame):
        self._candles[code] = candles.rechunk()
        self._calculate_candle_indicators(code, self._candles[code])

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
        self._calculate_candle_indicators(code, self._candles[code])

    def _on_timer(self):
        self.on_timer()

    def _on_position(self):
        pass

    def _on_order(self):
        pass

    def _on_cash(self):
        pass

    def _calculate_candle_indicators(self, code: Codes, candles: DataFrame):
        if code not in self._candle_indicators:
            return
        for name in self._candle_indicators[code]:
            self._candle_indicators[code][name]._on_data(candles)

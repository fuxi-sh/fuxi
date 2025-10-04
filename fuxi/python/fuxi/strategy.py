from abc import ABC
from decimal import Decimal
from typing import Dict, Optional
from pandas import DataFrame
from ._core import Context, Codes, Mode, Volume, Symbol, LogLevel, Backtest, Timer, Order, Method, Direction, Side
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
    # 日志API
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

    # ================================================================ #
    # 辅助API
    # ================================================================ #
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

    def flush_signal(self, signal: DataFrame, name: str = "default"):
        """
        刷新信号
        - [`signal`]: 信号
        - [`name`]: 信号名
        """
        self._signals[name] = signal.rechunk()

    def get_signal(self, name: str = "default") -> DataFrame:
        """
        获取信号
        - [`name`]: 信号名
        """
        if self.mode == Mode.Backtest:
            return self._signals[name].slice(0, self._backtest.offset)
        else:
            return self._signals[name]

    def get_candle(self, code: Codes) -> DataFrame:
        """
        获取K线
        - [`code`]: 交易对
        """
        if self.mode == Mode.Backtest:
            return self._candles[code].slice(0, self._backtest.offset)
        else:
            return self._candles[code]

    # ================================================================ #
    # 订单API
    # ================================================================ #

    def buy(
        self,
        code: Codes,
        size: Decimal,
        price: Decimal,
        remark: Optional[str] = None,
    ) -> Order:
        """
        做多开仓
        - [`code`]: 交易对
        - [`size`]: 数量
        - [`price`]: 价格
        - [`remark`]: 备注
        """
        return self._context.place_order(
            code,
            Method.Limit,
            Direction.Long,
            Side.Buy,
            size,
            price,
            remark,
        )

    def sell(
        self,
        code: Codes,
        size: Decimal,
        price: Decimal,
        remark: Optional[str] = None,
    ) -> Order:
        """
        做多平仓
        - [`code`]: 交易对
        - [`size`]: 数量
        - [`price`]: 价格
        - [`remark`]: 备注
        """
        return self._context.place_order(
            code,
            Method.Limit,
            Direction.Long,
            Side.Sell,
            size,
            price,
            remark,
        )

    def short(
        self,
        code: Codes,
        size: Decimal,
        price: Decimal,
        remark: Optional[str] = None,
    ) -> Order:
        """
        做空开仓
        - [`code`]: 交易对
        - [`size`]: 数量
        - [`price`]: 价格
        - [`remark`]: 备注
        """
        return self._context.place_order(
            code,
            Method.Limit,
            Direction.Short,
            Side.Sell,
            size,
            price,
            remark,
        )

    def cover(
        self,
        code: Codes,
        size: Decimal,
        price: Decimal,
        remark: Optional[str] = None,
    ) -> Order:
        """
        做空平仓
        - [`code`]: 交易对
        - [`size`]: 数量
        - [`price`]: 价格
        - [`remark`]: 备注
        """
        return self._context.place_order(
            code,
            Method.Limit,
            Direction.Short,
            Side.Buy,
            size,
            price,
            remark,
        )

    def send_order(
        self,
        code: Codes,
        method: Method,
        direction: Direction,
        side: Side,
        size: Decimal,
        price: Decimal,
        remark: Optional[str] = None,
    ) -> Order:
        """
        下单
        - [`code`]: 交易对
        - [`method`]: 交易方式
        - [`direction`]: 交易方向
        - [`side`]: 买卖方向
        - [`size`]: 订单数量
        - [`price`]: 订单价格
        - [`remark`]: 备注
        """
        return self._context.place_order(
            code,
            method,
            direction,
            side,
            size,
            price,
            remark,
        )

    def cancel(self, code: Codes, id: str):
        """
        取消订单
        - [`code`]: 交易对
        - [`id`]: 订单id
        """
        self._context.cancel_order(code, id)

    # ================================================================ #
    # 事件
    # ================================================================ #
    def on_init(self):
        """初始化事件"""

    def on_stop(self):
        """停止事件"""

    def on_candle(self, code: Codes, candles: DataFrame):
        """K线事件"""

    def on_signal(self):
        """信号事件"""

    def on_timer(self, timer: Timer):
        """定时器事件"""

    def on_position(self):
        """持仓事件"""

    def on_order(self):
        """订单事件"""

    def on_cash(self):
        """资金事件"""

    # ================================================================ #
    # 内部
    # ================================================================ #

    _context: Context
    _backtest: Backtest
    _candles: Dict[Codes, DataFrame]
    _signals: Dict[str, DataFrame]

    def __init__(self):
        self._candles = {}
        self._signals = {}

    def _on_inject_context(self, context: Context):
        self._context = context

    def _on_inject_backtest(self, backtest: Backtest):
        self._backtest = backtest

    def _on_init(self):
        self.on_init()

    def _on_stop(self):
        self.on_stop()

    def _on_history_candle(self, code: Codes, candles: DataFrame):
        df = candles.rechunk()
        self._candles[code] = df
        self.on_candle(code, df)
        if self.mode != Mode.Backtest:
            self.on_signal()

    def _on_candle(self, code: Codes, candles: DataFrame):
        df = (
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
        self._candles[code] = df
        self.on_candle(code, df)
        self.on_signal()

    def _on_backtest_tick(self):
        self.on_signal()

    def _on_timer(self, timer: Timer):
        self.on_timer(timer)

    def _on_position(self):
        self.on_position()

    def _on_order(self):
        self.on_order()

    def _on_cash(self):
        self.on_cash()

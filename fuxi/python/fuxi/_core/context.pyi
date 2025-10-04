from typing import Dict, Optional
from .market import Symbol
from .code import Codes
from .base import LogLevel, Mode, Volume, Method, Direction, Side
from .alias import Time, Price, Size
from .order import Order

class Context:
    """上下文"""

    mode: Mode
    """模式"""

    time: Time
    """当前时间"""
    spot: Volume
    """现货资金"""
    swap: Volume
    """合约资金"""
    symbols: Dict[Codes, Symbol]
    """交易对"""
    def show_log(self, level: LogLevel, *args):
        """显示日志"""

    def set_log_level(self, engine: LogLevel, strategy: LogLevel):
        """
        设置日志级别
        - [`engine`]: 引擎日志级别
        - [`strategy`]: 策略日志级别
        """

    def place_order(
        self,
        code: Codes,
        method: Method,
        direction: Direction,
        side: Side,
        size: Size,
        price: Price,
        remark: Optional[str],
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

    def cancel_order(self, code: Codes, id: str):
        """
        取消订单
        - [`code`]: 交易对
        - [`id`]: 订单id
        """

    @staticmethod
    def millis_to_time(millis: int) -> Time:
        """毫秒转换为时间"""

    @staticmethod
    def nanos_to_time(nanos: int) -> Time:
        """纳秒转换为时间"""

    @staticmethod
    def str_to_time(s: int) -> Time:
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

    @staticmethod
    def time_to_str(t: Time, fmt: str) -> Time:
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

    @staticmethod
    def new_id() -> str:
        """生成唯一id"""

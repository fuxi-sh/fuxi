from typing import Dict
from .market import Symbol
from .code import Codes
from .base import LogLevel, Mode, Volume
from .alias import Time

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

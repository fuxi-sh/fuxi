from enum import Enum, auto
from typing import List
from .alias import Size

class LogLevel(Enum):
    Trace = auto()
    Debug = auto()
    Info = auto()
    Warn = auto()
    Error = auto()
    @staticmethod
    def members() -> List[LogLevel]: ...

class Mode(Enum):
    Backtest = auto()
    Sandbox = auto()
    Mainnet = auto()
    @staticmethod
    def members() -> List[Mode]: ...

class Market(Enum):
    Spot = auto()
    Swap = auto()
    @staticmethod
    def members() -> List[Market]: ...

class Method(Enum):
    Market = auto()
    Limit = auto()
    @staticmethod
    def members() -> List[Method]: ...

class Direction(Enum):
    Long = auto()
    Short = auto()
    @staticmethod
    def members() -> List[Direction]: ...

class Side(Enum):
    Buy = auto()
    Sell = auto()
    @staticmethod
    def members() -> List[Side]: ...

class OrderStatus(Enum):
    New = auto()
    Submitting = auto()
    Rejected = auto()
    Pending = auto()
    Completed = auto()
    Cancelling = auto()
    Cancelled = auto()
    @staticmethod
    def members() -> List[OrderStatus]: ...

class Interval(Enum):
    Min = auto()
    Min3 = auto()
    Min5 = auto()
    Min15 = auto()
    Min30 = auto()
    Hour = auto()
    Hour2 = auto()
    Hour4 = auto()
    Hour8 = auto()
    Hour12 = auto()
    Day = auto()
    Day3 = auto()
    Week = auto()
    Month = auto()
    @staticmethod
    def members() -> List[Interval]: ...

class Volume:
    total: Size
    avail: Size
    frozen: Size

class Pnl:
    realized: Size
    unrealized: Size

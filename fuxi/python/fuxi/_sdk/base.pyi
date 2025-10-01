from enum import Enum, auto
from .alias import Size

class LogLevel(Enum):
    Trace = auto()
    Debug = auto()
    Info = auto()
    Warn = auto()
    Error = auto()

class RunMode(Enum):
    Backtest = auto()
    Sandbox = auto()
    Mainnet = auto()

class Market(Enum):
    Spot = auto()
    Swap = auto()

class TradeMethod(Enum):
    Market = auto()
    Limit = auto()

class TradeSide(Enum):
    Long = auto()
    Short = auto()

class TradeAction(Enum):
    Buy = auto()
    Sell = auto()

class OrderStatus(Enum):
    New = auto()
    Submitting = auto()
    Rejected = auto()
    Pending = auto()
    Completed = auto()
    Cancelling = auto()
    Cancelled = auto()

class CandlePeriod(Enum):
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

class Volume:
    total: Size
    avail: Size
    frozen: Size

class Pnl:
    realized: Size
    unrealized: Size

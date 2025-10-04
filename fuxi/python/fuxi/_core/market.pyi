from .position import Position
from .alias import Price, Size, Time
from .code import Codes

class Candle:
    """K线"""

    code: Codes
    """交易对"""
    time: Time
    """时间"""
    open: Price
    """开盘价"""
    high: Price
    """最高价"""
    low: Price
    """最低价"""
    close: Price
    """收盘价"""
    volume: Size
    """成交量"""

class FundingRate:
    """资金费率"""

    code: Codes
    """交易对"""
    time: Time
    """时间"""
    rate: Size
    """资金费率"""
    next_time: Time
    """下次结算时间"""
    min: Size
    """最小资金费率"""
    max: Size
    """最大资金费率"""
    update_time: Time
    """更新时间"""

class Symbol:
    """交易对"""

    code: Codes
    """交易对"""
    taker: Size
    """吃单费率"""
    maker: Size
    """挂单费率"""
    position: Position
    """持仓"""

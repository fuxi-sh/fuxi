from .position import Position
from .alias import Price, Size, Time
from .code import Codes

class Candle:
    code: Codes
    time: Time
    open: Price
    high: Price
    low: Price
    close: Price
    volume: Size

class FundingRate:
    code: Codes
    time: Time
    rate: Size
    next_time: Time
    min: Size
    max: Size
    update_time: Time

class Symbol:
    code: Codes
    taker: Size
    maker: Size
    position: Position

from .position import Position
from .alias import Size, Time
from .code import SymbolCode

class FundingRate:
    code: SymbolCode
    value: Size
    time: Time
    next_time: Time
    min: Size
    max: Size
    update_time: Time

class Symbol:
    code: SymbolCode
    taker: Size
    maker: Size
    position: Position

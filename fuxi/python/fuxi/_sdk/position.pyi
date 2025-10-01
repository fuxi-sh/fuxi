from typing import Dict
from .order import Order
from .alias import Price, Size
from .base import Pnl, TradeSide, Volume
from .code import SymbolCode

class SidePosition:
    code: SymbolCode
    side: TradeSide
    size: Volume
    price: Price
    pnl: Pnl

class Position:
    code: SymbolCode
    margin: Volume
    pnl: Pnl
    long: SidePosition
    short: SidePosition
    lever: Size
    orders: Dict[str, Order]

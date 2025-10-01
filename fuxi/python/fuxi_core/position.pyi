from typing import Dict
from .order import Order
from .alias import Price, Size
from .base import Direction, Pnl, Volume
from .code import Codes

class SidePosition:
    code: Codes
    direction: Direction
    size: Volume
    price: Price
    pnl: Pnl

class Position:
    code: Codes
    margin: Volume
    pnl: Pnl
    long: SidePosition
    short: SidePosition
    lever: Size
    orders: Dict[str, Order]

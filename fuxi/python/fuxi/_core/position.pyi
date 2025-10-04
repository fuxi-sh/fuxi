from typing import Dict
from .order import Order
from .alias import Price, Size
from .base import Direction, Pnl, Volume
from .code import Codes

class SidePosition:
    """方向持仓"""

    code: Codes
    """交易对"""
    direction: Direction
    """交易方向"""
    size: Volume
    """持仓数量"""
    price: Price
    """持仓价格"""
    pnl: Pnl
    """持仓盈亏"""

class Position:
    """持仓"""

    code: Codes
    """交易对"""
    margin: Volume
    """保证金"""
    pnl: Pnl
    """持仓盈亏"""
    long: SidePosition
    """多头持仓"""
    short: SidePosition
    """空头持仓"""
    lever: Size
    """杠杆倍数"""
    orders: Dict[str, Order]
    """订单"""

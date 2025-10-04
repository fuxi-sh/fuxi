from typing import Optional
from .alias import Price, Size, Time
from .base import Direction, Method, OrderStatus, Side
from .code import Codes

class Order:
    """订单"""

    code: Codes
    """交易对"""
    id: str
    """订单id"""
    method: Method
    """交易方式"""
    direction: Direction
    """交易方向"""
    side: Side
    """买卖方向"""
    status: OrderStatus
    """订单状态"""
    size: Size
    """订单数量"""
    price: Price
    """订单价格"""
    deal_size: Size
    """成交数量"""
    deal_price: Price
    """成交价格"""
    deal_fee: Size
    """成交手续费"""
    margin: Size
    """保证金"""
    remark: Optional[str]
    """备注"""
    create_time: Time
    """创建时间"""
    update_time: Time
    """更新时间"""

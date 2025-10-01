from typing import Optional
from .alias import Price, Size, Time
from .base import Direction, Method, OrderStatus, Side
from .code import Codes

class Order:
    code: Codes
    id: str
    method: Method
    direction: Direction
    side: Side
    status: OrderStatus
    size: Size
    price: Price
    deal_size: Size
    deal_price: Price
    deal_fee: Size
    margin: Size
    remark: Optional[str]
    create_time: Time
    update_time: Time

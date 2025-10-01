from typing import Optional
from .alias import Price, Size, Time
from .base import OrderStatus, TradeAction, TradeMethod, TradeSide
from .code import SymbolCode

class Order:
    code: SymbolCode
    id: str
    method: TradeMethod
    side: TradeSide
    action: TradeAction
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

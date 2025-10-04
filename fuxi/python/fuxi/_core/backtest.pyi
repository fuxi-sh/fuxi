from typing import Any, List, Tuple
from .code import Codes
from .alias import Size, Time

class Backtest:
    """回测引擎"""

    begin: Time
    """开始时间"""
    end: Time
    """结束时间"""
    history_size: int
    """历史数据大小"""
    offset: int
    """数据偏移量"""

    def __init__(
        self,
        strategy: Any,
        begin: str,
        end: str,
        symbols: List[Tuple[Codes, Size, Size, Size]],
        spot: Size = 1000,
        swap: Size = 1000,
        history_size: int = 5000,
        force_sync_data: bool = False,
    ):
        """
        初始化回测引擎
        - [`strategy`]: 策略实例
        - [`begin`]: 开始时间
        - [`end`]: 结束时间
        - [`symbols`]: 交易对配置
        - [`symbols.item`]: (交易对, 吃单费率, 挂单费率, 杠杆倍数)
        - [`spot`]: 现货资金
        - [`swap`]: 合约资金
        - [`history_size`]: 历史数据大小
        - [`force_sync_data`]: 强制同步历史行情数据
        """

    def launche(self):
        """启动回测"""

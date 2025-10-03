from typing import Any, List, Tuple
from .code import Codes
from .alias import Size

class Backtest:
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
    ): ...
    def launche(self): ...

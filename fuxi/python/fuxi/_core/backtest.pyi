from typing import List, Tuple
from .context import Context
from .code import Codes
from .alias import Size

class Backtest:
    def __init__(
        self,
        strategy: Context,
        begin: str,
        end: str,
        symbols: List[Tuple[Codes, Size, Size, Size]],
        spot: Size = 1000,
        swap: Size = 1000,
        history_size: int = 5000,
    ): ...
    def run(self): ...

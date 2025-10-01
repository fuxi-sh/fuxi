from typing import List, Tuple
from .context import Context
from .code import Codes
from .base import LogLevel
from .alias import Size

class Backtest(Context):
    def __init__(
        self,
        begin: str,
        end: str,
        symbols: List[Tuple[Codes, Size, Size, Size]],
        spot: Size = 1000,
        swap: Size = 1000,
        history_size: int = 5000,
        log_level: Tuple[LogLevel, LogLevel] = (LogLevel.Info, LogLevel.Info),
    ): ...
    def launcher(self): ...

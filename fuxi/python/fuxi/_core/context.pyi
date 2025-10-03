from typing import Dict
from .market import Symbol
from .code import Codes
from .base import LogLevel, Mode, Volume
from .alias import Time

class Context:
    mode: Mode
    time: Time
    spot: Volume
    swap: Volume
    symbols: Dict[Codes, Symbol]
    FMT_MS: str
    FMT_MS_CPT: str
    FMT_S: str
    FMT_S_CPT: str
    def show_log(self, level: LogLevel, *args): ...
    @staticmethod
    def millis_to_time(millis: int) -> Time: ...
    @staticmethod
    def nanos_to_time(nanos: int) -> Time: ...
    @staticmethod
    def str_to_time(s: int) -> Time: ...
    @staticmethod
    def time_to_str(t: Time, fmt: str) -> Time: ...
    @staticmethod
    def new_id() -> str: ...

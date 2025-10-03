from abc import ABC, abstractmethod
from typing import Optional
from .strategy import Strategy
from polars import DataFrame
import numpy as np
from numpy.typing import NDArray


class Indicator(ABC):
    _indicator: NDArray[np.float64]

    @abstractmethod
    def on_calculate(self, strategy: Strategy, candles: DataFrame) -> NDArray[np.float64]: ...

    def indicator(self, size: Optional[int] = None):
        if size is None:
            return self._indicator
        else:
            return self._indicator[:size]

    def _on_candles(self, strategy: Strategy, candles: DataFrame):
        self._indicator = self.on_calculate(strategy, candles)

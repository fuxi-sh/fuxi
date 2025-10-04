from abc import ABC, abstractmethod
from typing import List
from polars import DataFrame, Series


class Indicator(ABC):
    name: str
    _indicator: DataFrame

    def __init__(self, name: str):
        self.name = name

    @abstractmethod
    def on_calculate(self, data: DataFrame) -> List[Series]: ...

    def _on_data(self, data: DataFrame):
        self._indicator = data.select("time").with_columns(self.on_calculate(data)).rechunk()

from abc import ABC, abstractmethod
from polars import DataFrame


class Indicator(ABC):
    name: str
    _indicator: DataFrame

    def __init__(self, name: str):
        self.name = name

    @abstractmethod
    def on_calculate(self, data: DataFrame) -> DataFrame: ...

    def _on_data(self, data: DataFrame):
        self._indicator = self.on_calculate(data)

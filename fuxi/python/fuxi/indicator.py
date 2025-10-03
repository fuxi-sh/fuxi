from abc import ABC, abstractmethod
from polars import DataFrame, Series
import talib as ta


class Indicator(ABC):
    name: str
    _indicator: DataFrame

    def __init__(self, name: str):
        self.name = name

    @abstractmethod
    def on_calculate(self, data: DataFrame) -> DataFrame: ...

    def _on_data(self, data: DataFrame):
        self._indicator = self.on_calculate(data).rechunk()


class SMA(Indicator):
    data_name: str
    indicator_name: str
    period: int

    def __init__(
        self,
        name="sma",
        data_name: str = "close",
        indicator_name: str = "sma",
        period: int = 30,
    ):
        super().__init__(name)
        self.data_name = data_name
        self.indicator_name = indicator_name
        self.period = period

    def on_calculate(self, data: DataFrame) -> DataFrame:
        data_series = data[self.data_name].to_numpy(allow_copy=False)
        sma = ta.SMA(data_series, self.period)
        return data.select("time").with_columns(Series(self.indicator_name, sma))


class EMA(Indicator):
    data_name: str
    indicator_name: str
    period: int

    def __init__(
        self,
        name="ema",
        data_name: str = "close",
        indicator_name: str = "ema",
        period: int = 30,
    ):
        super().__init__(name)
        self.data_name = data_name
        self.indicator_name = indicator_name
        self.period = period

    def on_calculate(self, data: DataFrame) -> DataFrame:
        data_series = data[self.data_name].to_numpy(allow_copy=False)
        ema = ta.EMA(data_series, self.period)
        return data.select("time").with_columns(Series(self.indicator_name, ema))


class MACD(Indicator):
    data_name: str
    indicator_macd_name: str
    indicator_macd_signal_name: str
    indicator_macd_hist_name: str
    fast: int
    slow: int
    signal: int

    def __init__(
        self,
        name="macd",
        data_name: str = "close",
        indicator_macd_name: str = "macd",
        indicator_macd_signal_name: str = "signal",
        indicator_macd_hist_name: str = "hist",
        fast: int = 12,
        slow: int = 26,
        signal: int = 9,
    ):
        super().__init__(name)
        self.data_name = data_name
        self.indicator_macd_name = indicator_macd_name
        self.indicator_macd_signal_name = indicator_macd_signal_name
        self.indicator_macd_hist_name = indicator_macd_hist_name
        self.fast = fast
        self.slow = slow
        self.signal = signal

    def on_calculate(self, data: DataFrame) -> DataFrame:
        data_series = data[self.data_name].to_numpy(allow_copy=False)
        macd, macd_signal, macd_hist = ta.MACD(data_series, self.fast, self.slow, self.signal)
        return data.select("time").with_columns(
            Series(self.indicator_macd_name, macd),
            Series(self.indicator_macd_signal_name, macd_signal),
            Series(self.indicator_macd_hist_name, macd_hist),
        )

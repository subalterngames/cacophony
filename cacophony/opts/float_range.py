from typing import List, Callable
import numpy as np
from cacophony.opts import Options


class FloatRange(Options[float]):
    """
    A range of floats with an index at a current value.
    """

    def __init__(self, end: float, start: float = 0, index: int = 0, step: float = 0.01, callback: Callable[[int], None] = None, to_str: Callable[[], str] = None):
        """
        :param end: The ending value of the range.
        :param start: The starting value of the range.
        :param index: The index of the current value of the range.
        :param step: The value of each step.
        """

        self.__start: float = start
        self.__end: float = end
        self.__step: float = step
        super().__init__(index=index, callback=callback, to_str=to_str)

    def _get_range(self) -> List[float]:
        r: List[float] = np.arange(self.__start, self.__end + self.__step, step=self.__step).tolist()
        return [round(f, 2) for f in r]

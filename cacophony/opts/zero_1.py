from typing import List, Callable
import numpy as np
from cacophony.opts import Options


class Zero1(Options[float]):
    def __init__(self, index: int = 0, step: float = 0.01, callback: Callable[[int], None] = None, to_str: Callable[[], str] = None):
        self.__step: float = step
        super().__init__(index=index, callback=callback, to_str=to_str)

    def _get_range(self) -> List[float]:
        r: List[float] = np.arange(0, 1 + self.__step, step=self.__step).tolist()
        return [round(f, 2) for f in r]

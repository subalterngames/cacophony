from typing import List
import numpy as np


def zero1(step: float = 0.01, round_digits: int = 2) -> List[float]:
    arr = np.arange(0, 1, step=step)
    return [round(float(f), round_digits) for f in arr]


def zero127() -> List[int]:
    return list(range(0, 128))


def rangef(end: float, start: float = 0, step: float = 0.01, round_digits: int = 2) -> List[float]:
    r: List[float] = np.arange(start, end + step, step=step).tolist()
    return [round(f, round_digits) for f in r]

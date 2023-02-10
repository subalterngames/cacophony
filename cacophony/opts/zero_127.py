from typing import Callable
from cacophony.int_range import IntRange


class Zero127(IntRange):
    def __init__(self, index: int = 0, callback: Callable[[int], None] = None):
        super().__init__(length=127, index=index, callback=callback)

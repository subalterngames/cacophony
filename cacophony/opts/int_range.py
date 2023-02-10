from typing import List, Callable
from cacophony.opts import Options


class IntRange(Options[int]):
    """
    A range of integers with an index at a current value.
    """

    def __init__(self, length: int, start: int = 0, index: int = 0, callback: Callable[[int], None] = None, to_str: Callable[[], str] = None):
        """
        :param length: The length of the range.
        :param start: The starting value of the range.
        :param index: The index of the current value of the range.
        """

        self.__start: int = start
        self.__length: int = length
        super().__init__(index=index, callback=callback, to_str=to_str)

    def _get_range(self) -> List[int]:
        return list(range(self.__start, self.__start + self.__length))

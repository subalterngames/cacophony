from __future__ import annotations
from abc import ABC
from typing import List, TypeVar, Generic, Optional, Callable
from overrides import final
from cacophony.callbacker.callbacker import Callbacker


T = TypeVar("T")


class IndexedList(Callbacker, Generic[T], ABC):
    """
    A list of values with an index that wraps around the list (i.e. loops back to zero of len(values)).
    """

    def __init__(self, values: List[T], index: int = 0, callback: Callable[[], None] = None):
        """
        :param values: A list of values of type T.
        :param index: The current index.
        :param callback: An optional callback that is invoked when `index` is set.
        """

        super().__init__(callback=callback)
        self.values: List[T] = values
        self._index: int = index

    @property
    def index(self) -> int:
        return self._index

    @index.setter
    def index(self, value: int) -> None:
        self._index = value
        self._invoke()

    @final
    def increment(self, add: bool) -> None:
        """
        Increment or decrement the index. If it is less than 0, it becomes len(values) - 1. If it is greater than len(values) - 1, it becomes 0.

        :param add: If True, increment. If False, decrement.
        """

        if add:
            self.index += 1
            if self.index >= len(self.values):
                self.index = 0
        else:
            self.index -= 1
            if self.index < 0:
                self.index = len(self.values) - 1

    @final
    def get(self) -> T:
        """
        :return: Returns the value in `self.values` at index `self.index`.
        """

        return self.values[self.index]

    def get_str(self) -> str:
        """
        :return: The string representation of the value.
        """

        return str(self.get())

    @final
    def get_index(self, value: T) -> Optional[int]:
        """
        :param value: The value of type T.

        :return: The index in `self.values` of `value`. If `value` isn't in `self.values`, this returns None.
        """

        try:
            return self.values.index(value)
        except ValueError:
            return None

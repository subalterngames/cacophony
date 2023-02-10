from abc import ABC, abstractmethod
from typing import List, TypeVar, Generic, Callable
from overrides import final


T = TypeVar("T")


class Options(ABC, Generic[T]):
    def __init__(self, index: int = 0, callback: Callable[[int], None] = None, to_str: Callable[[], str] = None):
        self.__index: int = index
        self.__range: List[T] = self._get_range()
        self.__length: int = len(self.__range)
        self.__callback: Callable[[int], None] = callback
        self.__has_callback: bool = self.__callback is not None
        self.__to_str: Callable[[], str] = to_str
        self.__has_to_str: bool = self.__to_str is not None

    @final
    def get(self) -> T:
        """
        :return: The value at `range[index]`.
        """

        return self.__range[self.__index]

    @final
    def set(self, index: int) -> T:
        """
        :param index: The new index value.

        :return: The value at `range[index]`.
        """

        if index < 0:
            index = 0
        if index >= len(self.__range):
            index = len(self.__range) - 1
        self.__index = index
        # Invoke the callback.
        if self.__has_callback:
            self.__callback(self.__index)
        return self.get()

    @final
    def increment(self) -> T:
        """
        Increment the index by 1 and return the new value.

        :return: The value at `range[index]`.
        """

        return self.set(self.__index + 1)

    @final
    def decrement(self) -> T:
        """
        Decrement the index by 1 and return the new value.

        :return: The value at `range[index]`.
        """

        return self.set(self.__index - 1)

    @final
    def to_str(self) -> str:
        if not self.__has_to_str:
            return str(self.__index)
        else:
            return self.__to_str()

    @abstractmethod
    def _get_range(self) -> List[T]:
        raise Exception()



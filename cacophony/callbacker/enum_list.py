from enum import Enum
from typing import TypeVar, Type, Generic
from overrides import final
from cacophony.callbacker.indexed_list import IndexedList


T = TypeVar("T", bound=Enum)


class EnumList(IndexedList, Generic[T]):
    """
    An indexed list of enum values.
    """

    def __init__(self, t: Type[T], index: int = 0):
        """
        :param t: The type.
        :param index: The current index.
        """

        super().__init__(values=[e for e in t], index=index)

    @final
    def get_str_value(self) -> str:
        return self.get().name

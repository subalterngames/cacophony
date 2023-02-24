from enum import Enum
from typing import TypeVar, Type, Generic, List, Callable
from overrides import final
from cacophony.callbacker.indexed_list import IndexedList


T = TypeVar("T", bound=Enum)


class EnumList(IndexedList, Generic[T]):
    """
    An indexed list of enum values.
    """

    def __init__(self, t: Type[T], tts: str, index: int = 0, callback: Callable = None, kwargs: dict = None):
        """
        :param t: The type.
        :param tts: Text-to-speech text.
        :param index: The current index.
        :param callback: An optional callback that is invoked when `index` is set.
        :param kwargs: An optional dictionary of keyword arguments.
        """

        super().__init__(values=[e for e in t], tts=tts, index=index, callback=callback, kwargs=kwargs)

    @final
    def get_str_value(self) -> str:
        return self.get().name

    @final
    def get_strs(self) -> List[str]:
        return [v.name for v in self.values]

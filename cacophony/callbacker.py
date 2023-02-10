from typing import Callable, TypeVar, Generic, Optional
from overrides import final


T = TypeVar("T")


class Callbacker(Generic[T]):
    def __init__(self, callback: Callable[[Optional[T]], None]):
        self.__value: Optional[T] = None
        self.__callback: Callable[[T], None] = callback

    @final
    def get(self) -> Optional[T]:
        return self.__value

    @final
    def set(self, value: Optional[T]) -> Optional[T]:
        self.__value = value
        self.__callback(self.__value)
        return self.__value

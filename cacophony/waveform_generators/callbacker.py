from typing import Callable, TypeVar, Generic, Optional
from overrides import final


T = TypeVar("T")


class Callbacker(Generic[T]):
    def __init__(self, value: T, callback: Callable):
        self.__value: Optional[T] = value
        self.__callback: Callable = callback

    @final
    def get(self) -> Optional[T]:
        return self.__value

    @final
    def set(self, value: Optional[T]) -> Optional[T]:
        self.__value = value
        self.__callback()
        return self.__value

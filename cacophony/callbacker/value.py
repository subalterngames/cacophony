from typing import Callable, TypeVar, Generic
from cacophony.callbacker.callbacker import Callbacker

T = TypeVar("T")


class Value(Callbacker, Generic[T]):
    """
    A value with an optional callback method.
    """

    def __init__(self, value: T, tts: str, callback: Callable = None, kwargs: dict = None):
        """
        :param value: The initial value of type T.
        :param tts: Text-to-speech text.
        :param callback: An optional callback that is invoked when `value` is set.
        :param kwargs: An optional dictionary of keyword arguments.
        """

        super().__init__(callback=callback, tts=tts, kwargs=kwargs)
        self._value: T = value

    @property
    def value(self) -> T:
        return self._value

    @value.setter
    def value(self, v: T) -> None:
        self._value = v
        self._invoke()

from __future__ import annotations
from abc import ABC
from typing import Optional, Callable
from overrides import final


class Callbacker(ABC):
    """
    Abstract base class that can invoke a callback when a value is set. Includes text-to-speech help text.
    """

    def __init__(self, tts: str, callback: Callable = None, kwargs: dict = None):
        """
        :param tts: Text-to-speech text.
        :param callback: An optional callback that is invoked when `index` is set.
        :param kwargs: An optional dictionary of keyword arguments.
        """

        self.tts: str = tts
        self.__callback: Optional[Callable] = callback
        self.__kwargs: Optional[dict] = kwargs
        self.__has_callback: bool = self.__callback is not None
        self.__has_kwargs: bool = self.__kwargs is not None

    @final
    def set_kwargs(self, kwargs: dict) -> None:
        """
        Set the keyword arguments after creating the callbacker.
        """

        self.__kwargs = kwargs
        self.__has_kwargs = True

    @final
    def _invoke(self) -> None:
        """
        Invoke the callback.
        """

        if self.__has_callback:
            if self.__has_kwargs:
                self.__callback(**self.__kwargs)
            else:
                self.__callback()

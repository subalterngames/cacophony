from __future__ import annotations
from abc import ABC
from typing import Optional, Callable
from overrides import final


class Callbacker(ABC):
    """
    Abstract base class that can invoke a callback when a value is set. Includes text-to-speech help text.
    """

    def __init__(self, tts: str, callback: Callable[[], None] = None):
        """
        :param tts: Text-to-speech text.
        :param callback: An optional callback that is invoked when `index` is set.
        """

        self.tts: str = tts
        self.__callback: Optional[Callable[[], None]] = callback
        self.__has_callback: bool = self.__callback is not None

    @final
    def _invoke(self) -> None:
        """
        Invoke the callback.
        """

        if self.__has_callback:
            self.__callback()

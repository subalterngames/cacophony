from abc import ABC, abstractmethod
from typing import List, Tuple, Callable, Optional
from overrides import final
from pygame import Rect
from cacophony.render.commands.command import Command
from cacophony.state import State


class Widget(ABC):
    """
    A wrapper for a UI widget.
    """

    def __init__(self, callback: Callable = None, kwargs: dict = None):
        """
        :param callback: An optional callback method.
        :param kwargs: Optional keyword arguments for the callback.
        """

        # A stack of undo operations. Each element is a tuple: Callable, kwargs.
        self.undo_stack: List[Tuple[Callable, dict]] = list()
        self.__callback: Optional[callback] = callback
        self.__has_callback: bool = self.__callback is not None
        self.__kwargs: Optional[dict] = kwargs
        self.__has_kwargs: bool = self.__kwargs is not None

    @abstractmethod
    def blit(self, position: Tuple[int, int], panel_focus: bool, widget_focus: bool, pivot: Tuple[float, float] = None,
             anchor: Tuple[float, float] = None, parent_rect: Rect = None) -> List[Command]:
        """
        Blit the UI element.

        :param position: The position of this widget.
        :param panel_focus: If True, this widget's panel has focus.
        :param widget_focus: If True, this widget has focus.
        :param pivot: The pivot of this widget.
        :param anchor: The anchor of this widget.
        :param parent_rect: The parent rect.

        :return: A list of commands to blit the widget.
        """

        raise Exception()

    @abstractmethod
    def get_size(self) -> Tuple[int, int]:
        """
        :return: The size of this widget.
        """

        raise Exception()

    @abstractmethod
    def do(self, state: State) -> bool:
        """
        Do something!

        :param state: The [`State`](state.md) of the program.

        :return: True if we did something.
        """

        raise Exception()

    @abstractmethod
    def get_help_text(self) -> str:
        """
        :return: Text-to-speech text.
        """

        raise Exception()

    @final
    def set_callback(self, callback: Callable, kwargs: dict = None):
        """
        Set the callback after creating the widget.

        :param callback: A callback method.
        :param kwargs: Optional keyword arguments for the callback.
        """

        self.__callback = callback
        self.__has_callback = True
        self.__kwargs = kwargs
        self.__has_kwargs = self.__kwargs is not None

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
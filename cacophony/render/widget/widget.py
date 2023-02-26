from abc import ABC, abstractmethod
from typing import List, Tuple, Callable
from pygame import Rect
from cacophony.render.commands.command import Command
from cacophony.state import State


class Widget(ABC):
    """
    A wrapper for a UI widget.
    """

    def __init__(self):
        """
        (no parameters)
        """

        # A stack of undo operations. Each element is a tuple: Callbable, kwargs.
        self.undo_stack: List[Tuple[Callable, dict]] = list()

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

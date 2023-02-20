from abc import ABC, abstractmethod
from typing import List, Tuple
from pygame import Rect
from cacophony.render.commands.command import Command


class UiElement(ABC):
    """
    A wrapper for a UI element.
    """

    @abstractmethod
    def blit(self, position: Tuple[int, int], panel_focus: bool, element_focus: bool, pivot: Tuple[float, float] = None,
             anchor: Tuple[float, float] = None, parent_rect: Rect = None) -> List[Command]:
        """
        Blit the UI element.

        :param position: The position of this element.
        :param panel_focus: If True, this element's panel has focus.
        :param element_focus: If True, this element has focus.
        :param pivot: The pivot of this element.
        :param anchor: The anchor of this element.
        :param parent_rect: The parent rect.

        :return: A list of commands to blit the element.
        """

        raise Exception()

    @abstractmethod
    def get_size(self) -> Tuple[int, int]:
        """
        :return: The size of this element.
        """

        raise Exception()

from typing import Tuple
from pygame import Surface, Rect
from abc import ABC, abstractmethod


class RenderCommand(ABC):
    """
    Abstract base class for render commands.
    """

    @abstractmethod
    def do(self) -> Tuple[Surface, Rect]:
        """
        Do something!

        :return: Tuple: The surface, the rect.
        """

        raise Exception()

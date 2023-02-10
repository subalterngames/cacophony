from typing import Tuple
from abc import ABC, abstractmethod
from overrides import final
import pygame
from pygame import Surface, Rect
from cacophony.render.render_command import RenderCommand
from cacophony.render.globals import CELL_SIZE, CLEAR_COLOR


class DrawRectangle(RenderCommand, ABC):
    """
    Abstract base class for drawing rectangles.
    """

    def __init__(self, position: Tuple[int, int], size: Tuple[int, int], color: Tuple[int, int, int]):
        """
        :param position: The top-left position in grid coordinates.
        :param size: The size in grid coordinates.
        :param color: The RGB (0-255) color.
        """

        self._position: Tuple[int, int] = position
        self._size: Tuple[int, int] = size
        self._color: Tuple[int, int, int] = color

    @final
    def do(self) -> Tuple[Surface, Rect]:
        surface = Surface((self._size[0] * CELL_SIZE[0],
                           self._size[1] * CELL_SIZE[1]))
        surface = self._on_create_surface(surface)
        border_width: int = self._get_width()
        pygame.draw.rect(surface, self._color, Rect(0, 0, self._size[0], self._size[1]), width=border_width)
        rect = surface.get_rect()
        rect.x = self._position[0] * CELL_SIZE[0]
        rect.y = self._position[1] * CELL_SIZE[1]
        surface.set_colorkey(CLEAR_COLOR)
        return surface, rect

    @abstractmethod
    def _on_create_surface(self, surface: Surface) -> Surface:
        raise Exception()

    @abstractmethod
    def _get_width(self) -> int:
        raise Exception()

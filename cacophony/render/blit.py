from typing import Tuple
from overrides import final
from pygame import Surface, Rect
from cacophony.render.render_command import RenderCommand
from cacophony.render.globals import CELL_SIZE


class Blit(RenderCommand):
    """
    Blit an arbitrary surface.
    """

    def __init__(self, position: Tuple[int, int], surface: Surface):
        """
        :param position: The top-left position in grid coordinates.
        :param surface: The surface that we'll blit.
        """

        self._position: Tuple[int, int] = position
        self._surface: Surface = surface

    @final
    def do(self) -> Tuple[Surface, Rect]:
        rect = self._surface.get_rect()
        rect.x = self._position[0] * CELL_SIZE[0]
        rect.y = self._position[1] * CELL_SIZE[1]
        return self._surface, rect

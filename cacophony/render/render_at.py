from abc import ABC, abstractmethod
from typing import Tuple
from overrides import final
from pygame import Surface, Rect
from cacophony.render.render_command import RenderCommand
from cacophony.render.globals import WINDOW_PIXEL_WIDTH, WINDOW_PIXEL_HEIGHT, CELL_SIZE


class RenderAt(RenderCommand, ABC):
    """
    Render at a given position.
    """

    def __init__(self, position: Tuple[int, int], pivot: Tuple[float, float] = None, anchor: Tuple[float, float] = None):
        """
        :param position: The position in grid coordinates.
        :param pivot: The pivot as a (x, y) 0-1 tuple. If the pivot is `(0, 0)`, then it is located at the top-left of the surface we're about to render. `(0, 1)` would be bottom-left, etc.
        :param anchor: The anchor as a (x, y) 0-1 tuple. If the anchor is `(0, 0)`, there is no offset. If the anchor is `(1, 0)`, then the position would be offset by the pivot offset plus the width of the screen.
        """

        if pivot is None:
            self._pivot: Tuple[int, int] = (0, 0)
        else:
            self._pivot = pivot
        if anchor is None:
            self._anchor: Tuple[int, int] = (0, 0)
        else:
            self._anchor = anchor
        self._position: Tuple[int, int] = position

    @final
    def do(self) -> Tuple[Surface, Rect]:
        # Get the surface.
        surface = self._get_surface()
        # Get the rect.
        rect = surface.get_rect()
        # Set the position.
        rect.x = self._position[0] * CELL_SIZE[0]
        rect.y = self._position[1] * CELL_SIZE[1]
        # Offset the position.
        rect.x -= int(rect.w * self._pivot[0])
        rect.y -= int(rect.h * self._pivot[1])
        # Apply the anchor.
        rect.x += int(WINDOW_PIXEL_WIDTH * self._anchor[0])
        rect.y += int(WINDOW_PIXEL_HEIGHT * self._anchor[1])
        return surface, rect

    @abstractmethod
    def _get_surface(self) -> Surface:
        raise Exception()

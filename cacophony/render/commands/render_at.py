from abc import ABC, abstractmethod
from typing import Tuple
from overrides import final
from pygame import Surface, Rect
from pygame.display import get_surface
from cacophony.render.commands.command import Command
from cacophony.render.globals import CELL_SIZE


class RenderAt(Command, ABC):
    """
    Blit at a given position.
    """

    def __init__(self, position: Tuple[int, int], pivot: Tuple[float, float] = None, anchor: Tuple[float, float] = None,
                 parent_rect: Rect = None):
        """
        :param position: The position in grid coordinates.
        :param pivot: The pivot as a (x, y) 0-1 tuple. If the pivot is `(0, 0)`, then it is located at the top-left of the surface we're about to render. `(0, 1)` would be bottom-left, etc.
        :param anchor: The anchor as a (x, y) 0-1 tuple. If the anchor is `(0, 0)`, there is no offset. If the anchor is `(1, 0)`, then the position would be offset by the pivot offset plus the width of the screen.
        :param parent_rect: The parent rect that pivot and anchor are relative to. If None, this defaults to the display surface rect.
        """

        if pivot is None:
            self._pivot: Tuple[int, int] = (0, 0)
        else:
            self._pivot = pivot
        if anchor is None:
            self._anchor: Tuple[int, int] = (0, 0)
        else:
            self._anchor = anchor
        if parent_rect is None:
            # Get the display surface rect.
            self._parent_rect: Rect = get_surface().get_rect()
        else:
            self._parent_rect = parent_rect
        self._position: Tuple[int, int] = position

    @final
    def do(self) -> Tuple[Surface, Rect]:
        # Get the surface.
        surface = self._get_surface()
        # Get the rect.
        rect = surface.get_rect()
        # Set the display position.
        rect.x = self._position[0] * CELL_SIZE[0]
        rect.y = self._position[1] * CELL_SIZE[1]
        # Apply the pivot as an offset from this surface's dimensions.
        rect.x -= int(rect.w * self._pivot[0])
        rect.y -= int(rect.h * self._pivot[1])
        # Apply the anchor as an offset from the parent rect.
        rect.x += self._parent_rect.x + int(self._parent_rect.w * self._anchor[0])
        rect.y += self._parent_rect.y + int(self._parent_rect.h * self._anchor[1])
        return surface, rect

    @abstractmethod
    def _get_surface(self) -> Surface:
        raise Exception()

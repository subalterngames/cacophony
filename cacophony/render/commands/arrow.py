from typing import Tuple
import pygame
from pygame import Surface
import numpy as np
from cacophony.cardinal_direction import CardinalDirection
from cacophony.render.globals import ARROWS, NUMPY_COLOR, CLEAR_COLOR
from cacophony.render.commands.render_at import RenderAt


class Arrow(RenderAt):
    def __init__(self, color: Tuple[int, int, int], direction: CardinalDirection, position: Tuple[int, int],
                 pivot: Tuple[float, float] = None, anchor: Tuple[float, float] = None):
        """
        :param color: The RGB (0-255) color.
        :param direction: The direction of the arrow. East is pointing eastwards.
        :param position: The position in grid coordinates.
        :param pivot: The pivot as a (x, y) 0-1 tuple. If the pivot is `(0, 0)`, then it is located at the top-left of the surface we're about to render. `(0, 1)` would be bottom-left, etc.
        :param anchor: The anchor as a (x, y) 0-1 tuple. If the anchor is `(0, 0)`, there is no offset. If the anchor is `(1, 0)`, then the position would be offset by the pivot offset plus the width of the screen.
        """

        super().__init__(position=position, pivot=pivot, anchor=anchor)
        self._color: Tuple[int, int, int] = color
        self._direction: CardinalDirection = direction

    def _get_surface(self) -> Surface:
        arr: np.ndarray = np.copy(ARROWS[self._direction])
        arr[np.all(arr == NUMPY_COLOR, axis=2), :] = self._color
        surface = pygame.surfarray.make_surface(arr)
        surface.set_colorkey(CLEAR_COLOR)
        return surface

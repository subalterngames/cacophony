from typing import Tuple
from abc import ABC
from cacophony.render.commands.render_at import RenderAt
from cacophony.render.globals import CELL_SIZE


class AbcRectangle(RenderAt, ABC):
    """
    Abstract base class for drawing rectangles.
    """

    def __init__(self, size: Tuple[int, int], color: Tuple[int, int, int], position: Tuple[int, int], pivot: Tuple[float, float] = None, anchor: Tuple[float, float] = None):
        """
        :param size: The size in grid coordinates.
        :param color: The RGB (0-255) color.
        :param position: The top-left position in grid coordinates.
        :param pivot: The pivot as a (x, y) 0-1 tuple. If the pivot is `(0, 0)`, then it is located at the top-left of the surface we're about to render. `(0, 1)` would be bottom-left, etc.
        :param anchor: The anchor as a (x, y) 0-1 tuple. If the anchor is `(0, 0)`, there is no offset. If the anchor is `(1, 0)`, then the position would be offset by the pivot offset plus the width of the screen.
        """

        self._size: Tuple[int, int] = (size[0] * CELL_SIZE[0], size[1] * CELL_SIZE[1])
        self._color: Tuple[int, int, int] = color
        super().__init__(position=position, pivot=pivot, anchor=anchor)

from typing import Tuple
from pygame import Surface, Rect
from pygame.draw import line
from cacophony.render.globals import BORDER_PIXEL_WIDTH, CLEAR_COLOR, CELL_SIZE
from cacophony.render.commands.rectangle import Rectangle


class Line(Rectangle):
    """
    Blit a line.
    """

    def __init__(self, length: int, vertical: bool, color: Tuple[int, int, int], position: Tuple[int, int],
                 d: float = 0.5,  pivot: Tuple[float, float] = None, anchor: Tuple[float, float] = None,
                 parent_rect: Rect = None):
        """
        :param length: The length of the line in grid coordinates.
        :param color: The RGB (0-255) color.
        :param position: The top-left position in grid coordinates.
        :param d: A value (0-1) the determines how centered the line is within a row or column. 0.5 = centered.
        :param pivot: The pivot as a (x, y) 0-1 tuple. If the pivot is `(0, 0)`, then it is located at the top-left of the surface we're about to render. `(0, 1)` would be bottom-left, etc.
        :param anchor: The anchor as a (x, y) 0-1 tuple. If the anchor is `(0, 0)`, there is no offset. If the anchor is `(1, 0)`, then the position would be offset by the pivot offset plus the width of the screen.
        :param parent_rect: The parent rect that pivot and anchor are relative to. If None, this defaults to the display surface rect.
        """

        super().__init__(size=(1, length) if vertical else (length, 1),
                         color=CLEAR_COLOR,
                         position=position,
                         pivot=pivot,
                         anchor=anchor,
                         parent_rect=parent_rect)
        self._line_color: Tuple[int, int, int] = color
        self._vertical: bool = vertical
        self._d: float = d

    def _get_surface(self) -> Surface:
        surface = super()._get_surface()
        surface.set_colorkey(CLEAR_COLOR)
        if self._vertical:
            x0 = int(CELL_SIZE[0] * self._d)
            y0 = 0
            x1 = x0
            y1 = surface.get_size()[1]
        else:
            x0 = 0
            y0 = int(CELL_SIZE[1] * self._d)
            x1 = surface.get_size()[0]
            y1 = y0
        line(surface, self._line_color, (x0, y0), (x1, y1), width=BORDER_PIXEL_WIDTH)
        return surface

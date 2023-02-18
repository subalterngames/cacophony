from typing import Tuple
from pygame.display import get_surface
from pygame import Rect
from cacophony.render.globals import CELL_SIZE


def get_parent_rect(position: Tuple[int, int], size: Tuple[int, int], pivot: Tuple[float, float] = None,
                    anchor: Tuple[float, float] = None, grandparent_rect: Rect = None) -> Rect:
    """
    :param position: The position in grid coordinates.
    :param size: The size in grid coordinates.
    :param pivot: The pivot as a (x, y) 0-1 tuple. If the pivot is `(0, 0)`, then it is located at the top-left of the surface we're about to render. `(0, 1)` would be bottom-left, etc.
    :param anchor: The anchor as a (x, y) 0-1 tuple. If the anchor is `(0, 0)`, there is no offset. If the anchor is `(1, 0)`, then the position would be offset by the pivot offset plus the width of the screen.
    :param grandparent_rect: The parent rect of this rect that pivot and anchor are relative to. If None, this defaults to the display surface rect.

    :return: The parent rect.
    """

    if pivot is None:
        pivot = (0, 0)
    if anchor is None:
        anchor = (0, 0)
    if grandparent_rect is None:
        grandparent_rect = get_surface().get_rect()
    rect: Rect = Rect(position[0] * CELL_SIZE[0],
                      position[1] * CELL_SIZE[1],
                      size[0] * CELL_SIZE[0],
                      size[1] * CELL_SIZE[1])
    rect.x -= int(rect.w * pivot[0])
    rect.y -= int(rect.h * pivot[1])
    rect.x += grandparent_rect.x + int(grandparent_rect.w * anchor[0])
    rect.y += grandparent_rect.y + int(grandparent_rect.h * anchor[1])
    return rect

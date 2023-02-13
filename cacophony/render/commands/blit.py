from typing import Tuple
from pygame import Surface
from cacophony.render.commands.render_at import RenderAt


class Blit(RenderAt):
    """
    Blit an arbitrary surface.
    """

    def __init__(self, surface: Surface, position: Tuple[int, int], pivot: Tuple[float, float] = None, anchor: Tuple[float, float] = None):
        """
        :param surface: The surface to be blitted.
        :param position: The position in grid coordinates.
        :param pivot: The pivot as a (x, y) 0-1 tuple. If the pivot is `(0, 0)`, then it is located at the top-left of the surface we're about to render. `(0, 1)` would be bottom-left, etc.
        :param anchor: The anchor as a (x, y) 0-1 tuple. If the anchor is `(0, 0)`, there is no offset. If the anchor is `(1, 0)`, then the position would be offset by the pivot offset plus the width of the screen.
        """

        self._surface: Surface = surface
        super().__init__(position=position, pivot=pivot, anchor=anchor)

    def _get_surface(self) -> Surface:
        return self._surface

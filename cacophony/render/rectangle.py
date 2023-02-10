from pygame import Surface
from cacophony.render.abc_rectangle import AbcRectangle


class Rectangle(AbcRectangle):
    """
    Draw a filled rectangle.
    """

    def _get_surface(self) -> Surface:
        surface = Surface(self._size)
        surface.fill(self._color)
        return surface

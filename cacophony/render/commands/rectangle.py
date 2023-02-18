from pygame import Surface
from cacophony.render.commands.abc_rectangle import AbcRectangle


class Rectangle(AbcRectangle):
    """
    Blit a filled rectangle surface.
    """

    def _get_surface(self) -> Surface:
        surface = Surface(self._size)
        surface.fill(self._color)
        return surface

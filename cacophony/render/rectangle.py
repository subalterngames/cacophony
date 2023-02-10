from pygame import Surface
from cacophony.render.draw_rectangle import DrawRectangle


class Rectangle(DrawRectangle):
    """
    Draw a filled rectangle.
    """

    def _get_width(self) -> int:
        return 0

    def _on_create_surface(self, surface: Surface) -> Surface:
        return surface

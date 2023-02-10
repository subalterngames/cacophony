from pygame import Surface
from cacophony.render.globals import BORDER_PIXEL_WIDTH, CLEAR_COLOR
from cacophony.render.draw_rectangle import DrawRectangle


class Border(DrawRectangle):
    """
    Draw a rectangular border.
    """

    def _get_width(self) -> int:
        return BORDER_PIXEL_WIDTH

    def _on_create_surface(self, surface: Surface) -> Surface:
        surface.fill(CLEAR_COLOR)
        return surface

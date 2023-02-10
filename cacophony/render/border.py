import pygame
from pygame import Surface, Rect
from cacophony.render.globals import BORDER_PIXEL_WIDTH, CLEAR_COLOR, CLEAR_COLOR_IS_BLACK
from cacophony.render.abc_rectangle import AbcRectangle


class Border(AbcRectangle):
    """
    Draw a rectangular border.
    """

    def _get_surface(self) -> Surface:
        surface = Surface(self._size)
        # Make the surface transparent.
        if not CLEAR_COLOR_IS_BLACK:
            surface.fill(CLEAR_COLOR)
        # Draw the rectangle.
        pygame.draw.rect(surface, self._color, Rect(0, 0, self._size[0], self._size[1]), width=BORDER_PIXEL_WIDTH)
        return surface

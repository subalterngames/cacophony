import pygame
from pygame import Surface, Rect
from cacophony.render.globals import BORDER_PIXEL_WIDTH, CLEAR_COLOR, CLEAR_COLOR_IS_BLACK, CELL_SIZE
from cacophony.render.commands.abc_rectangle import AbcRectangle


class Border(AbcRectangle):
    """
    Blit a rectangular border.
    """

    def _get_surface(self) -> Surface:
        surface = Surface(self._size)
        # Make the surface transparent.
        if not CLEAR_COLOR_IS_BLACK:
            surface.fill(CLEAR_COLOR)
        # Draw the rectangle.
        pygame.draw.rect(surface, self._color, Rect(CELL_SIZE[0] // 2,
                                                    CELL_SIZE[1] // 3,
                                                    self._size[0] - CELL_SIZE[0],
                                                    int(self._size[1] - CELL_SIZE[1] * (2 / 3))),
                         width=BORDER_PIXEL_WIDTH)
        surface.set_colorkey(CLEAR_COLOR)
        return surface

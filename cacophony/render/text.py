from textwrap import wrap
from typing import Tuple, Optional, List
from overrides import final
import pygame
from pygame import Surface, Rect
from cacophony.render.render_command import RenderCommand
from cacophony.render.globals import CELL_SIZE, CLEAR_COLOR, FONT


class Text(RenderCommand):
    """
    Add text, which can optionally wrap around a width.
    """

    def __init__(self, text: str, position: Tuple[int, int], color: Tuple[int, int, int], size: Tuple[int, int] = None):
        """
        :param text: The text.
        :param position: The top-left position in grid coordinates.
        :param color: The RGB (0-255) color.
        :param size: The size in grid coordinates. The text will wrap around the width and won't exceed the height. If None, the text is a single unwrapped line.
        """

        self._text: str = text
        self._position: Tuple[int, int] = position
        self._size: Optional[Tuple[int, int]] = size
        self._color: Tuple[int, int, int] = color

    @final
    def do(self) -> Tuple[Surface, Rect]:
        # One line of text.
        if self._size is None:
            surface = FONT.render(self._text, True, self._color, CLEAR_COLOR)
            rect = surface.get_rect()
            rect.x = self._position[0] * CELL_SIZE[0]
            rect.y = self._position[1] * CELL_SIZE[1]
        # Wrap the text.
        else:
            lines: List[str] = wrap(text=self._text, width=self._size[0])
            # Set the bounded height.
            if len(lines) > self._size[1]:
                height = self._size[1]
            else:
                height = len(lines)
            surface = Surface((self._size[0] * CELL_SIZE[0], height * CELL_SIZE[1]))
            surface.fill(CLEAR_COLOR)
            y = 0
            for i in range(height):
                text_surface = FONT.render(lines[i], True, self._color, CLEAR_COLOR)
                surface.blit(text_surface, (0, y))
                y += CELL_SIZE[1]
        rect = surface.get_rect()
        rect.x = self._position[0] * CELL_SIZE[0]
        rect.y = self._position[1] * CELL_SIZE[1]
        surface.set_colorkey(CLEAR_COLOR)
        return surface, rect


pygame.init()
Text(text="oots in a piece of classical Latin literature from 45 BC, making it over 2000 years old. Richard McClintock, a Latin professor at Hampden-Sydney College in Virginia, looked up one of the more obscure Latin words, consectetur, from a Lorem Ipsum passage, and going through the cites of the word in classical literature, discovered the undou",
     position=(1, 3), size=(18, 16), color=(255, 0, 0)).do()
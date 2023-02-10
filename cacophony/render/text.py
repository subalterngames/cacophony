from textwrap import wrap
from typing import Tuple, Optional, List, Union
from pygame import Surface
from cacophony.render.render_at import RenderAt
from cacophony.render.globals import CELL_SIZE, CLEAR_COLOR, CLEAR_COLOR_IS_BLACK, FONT


class Text(RenderAt):
    """
    Add text, which can optionally wrap around a width.
    """

    def __init__(self, text: str, color: Tuple[int, int, int], position: Tuple[int, int], size: Union[int, Tuple[int, int]] = None, truncate_left: bool = True, pivot: Tuple[float, float] = None, anchor: Tuple[float, float] = None):
        """
        :param text: The text.
        :param color: The RGB (0-255) color.
        :param position: The top-left position in grid coordinates.
        :param size: Either an integer or a tuple value for the size in grid coordinates. If an integer, the text will be one line, truncated to this length (see `overflow`). If a tuple, the text will wrap around the width and won't exceed the height. If None, the text is a single unwrapped line.
        :param truncate_left: If `size` is an integer, this handles how to truncate text. If True, text will be rendered from `0` to `size`. If False, text will be rendered from `len(text) - size)` to `len(text)`.
        :param pivot: The pivot as a (x, y) 0-1 tuple. If the pivot is `(0, 0)`, then it is located at the top-left of the surface we're about to render. `(0, 1)` would be bottom-left, etc.
        :param anchor: The anchor as a (x, y) 0-1 tuple. If the anchor is `(0, 0)`, there is no offset. If the anchor is `(1, 0)`, then the position would be offset by the pivot offset plus the width of the screen.
        """

        super().__init__(position=position, pivot=pivot, anchor=anchor)
        self._text: str = text
        self._size: Optional[Union[int, Tuple[int, int]]] = size
        self._color: Tuple[int, int, int] = color
        self._truncate_left: bool = truncate_left

    def _get_surface(self) -> Surface:
        # One line of text.
        if self._size is None:
            return FONT.render(self._text, True, self._color, CLEAR_COLOR)
        # Maybe truncate.
        elif isinstance(self._size, int):
            # No need to truncate.
            if len(self._text) <= self._size:
                return FONT.render(self._text, True, self._color, CLEAR_COLOR)
            # Truncate.
            else:
                if self._truncate_left:
                    text = self._text[:self._size]
                else:
                    text = self._text[len(self._text) - self._size:]
                return FONT.render(text, True, self._color, CLEAR_COLOR)
        # Wrap the text.
        elif isinstance(self._size, tuple):
            lines: List[str] = wrap(text=self._text, width=self._size[0])
            # Set the bounded height.
            if len(lines) > self._size[1]:
                height = self._size[1]
            else:
                height = len(lines)
            surface = Surface((self._size[0] * CELL_SIZE[0], height * CELL_SIZE[1]))
            if not CLEAR_COLOR_IS_BLACK:
                surface.fill(CLEAR_COLOR)
            y = 0
            for i in range(height):
                text_surface = FONT.render(lines[i], True, self._color, CLEAR_COLOR)
                surface.blit(text_surface, (0, y))
                y += CELL_SIZE[1]
            return surface
        else:
            raise Exception(f"Invalid size: {self._size}")

from typing import List, Tuple, Union
from overrides import final
from pygame import Rect
from cacophony.render.commands.command import Command
from cacophony.render.commands.text import Text
from cacophony.render.commands.border import Border
from cacophony.render.color import Color
from cacophony.render.globals import COLORS
from cacophony.render.widget.widget import Widget
from cacophony.state import State


class Label(Widget):
    """
    A text label.
    """

    def __init__(self, text: str, size: Union[int, Tuple[int, int]] = None):
        """
        :param text: The label text.
        :param size: Either an integer or a tuple value for the size of the element in grid coordinates. If an integer, the text will be one line, truncated to this length (see `overflow`). If a tuple, the text will wrap around the width and won't exceed the height. If None, the text is a single unwrapped line.
        """

        super().__init__()
        self._text: str = text
        # One line, as long as the text is long.
        if size is None:
            self._size: Tuple[int, int] = (len(self._text) + 2, 3)
        # One line, with a specified width.
        elif isinstance(size, int):
            self._size = (size, 3)
        # A specified size.
        elif isinstance(size, tuple):
            self._size = size
        # Oops.
        else:
            raise Exception(f"Invalid label size: {size}")

    def blit(self, position: Tuple[int, int], panel_focus: bool, widget_focus: bool, pivot: Tuple[float, float] = None,
             anchor: Tuple[float, float] = None, parent_rect: Rect = None) -> List[Command]:
        if not panel_focus:
            c = Color.border_no_focus
        elif widget_focus:
            c = Color.parameter_value
        else:
            c = Color.border_focus
        color = COLORS[c]
        commands = []
        # Show the border.
        if widget_focus:
            commands.append(Border(position=position,
                                   size=self._size,
                                   color=color,
                                   pivot=pivot,
                                   anchor=anchor,
                                   parent_rect=parent_rect))
        # Show the text.
        commands.append(Text(text=self._text,
                             position=(position[0] + 1, position[1] + 1),
                             text_color=color,
                             background_color=COLORS[Color.panel_background],
                             pivot=pivot,
                             anchor=anchor,
                             parent_rect=parent_rect))
        return commands

    @final
    def get_size(self) -> Tuple[int, int]:
        return self._size

    def do(self, state: State) -> bool:
        return False

    def get_help_text(self) -> str:
        return self._text

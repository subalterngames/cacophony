from typing import List, Tuple
from overrides import final
from pygame import Rect
from cacophony.render.commands.command import Command
from cacophony.render.commands.text import Text
from cacophony.render.commands.border import Border
from cacophony.render.commands.arrow import Arrow
from cacophony.render.color import Color
from cacophony.render.globals import COLORS
from cacophony.render.widget.widget import Widget
from cacophony.render.render_result import RenderResult
from cacophony.render.input_key import InputKey
from cacophony.cardinal_direction import CardinalDirection
from cacophony.util import tooltip


class Options(Widget):
    """
    Cycle through a list of options.
    """

    def __init__(self, title: str, options: List[str], index: int):
        """
        :param title: The title text.
        :param options: A list of options.
        :param index: The index of the current option.
        """

        super().__init__()
        self._title: str = title
        self.options: List[str] = options
        self.index: int = index
        self._max_text_length: int = max([len(o) for o in self.options])
        if len(title) > self._max_text_length:
            self._max_text_length = len(title)
        self._size: Tuple[int, int] = (self._max_text_length + 4, 4)

    def blit(self, position: Tuple[int, int], panel_focus: bool, element_focus: bool, pivot: Tuple[float, float] = None,
             anchor: Tuple[float, float] = None, parent_rect: Rect = None) -> List[Command]:
        if not panel_focus:
            c = Color.border_no_focus
        elif element_focus:
            c = Color.parameter_value
        else:
            c = Color.border_focus
        color = COLORS[c]
        commands = []
        text_y = position[1] + 1
        text_x = position[0] + 2
        # Show the title text.
        commands.append(Text(text=self._title,
                             position=(position[0] + 1, text_y),
                             text_color=color,
                             background_color=COLORS[Color.panel_background],
                             pivot=pivot,
                             anchor=anchor,
                             parent_rect=parent_rect))
        text_y += 1
        # Show the border and the arrows.
        if element_focus:
            commands.extend([Border(position=position,
                                    size=self._size,
                                    color=color,
                                    pivot=pivot,
                                    anchor=anchor,
                                    parent_rect=parent_rect),
                             Arrow(position=(position[0] + 1, text_y),
                                   direction=CardinalDirection.west,
                                   color=color,
                                   pivot=pivot,
                                   anchor=anchor,
                                   parent_rect=parent_rect),
                             Arrow(position=(position[0] + 2 + self._max_text_length, text_y),
                                   direction=CardinalDirection.east,
                                   color=color,
                                   pivot=pivot,
                                   anchor=anchor,
                                   parent_rect=parent_rect)])
        # Show the text.
        commands.append(Text(text=self.options[self.index],
                             position=(text_x, text_y),
                             size=self._max_text_length,
                             text_color=color,
                             background_color=COLORS[Color.panel_background],
                             pivot=pivot,
                             anchor=anchor,
                             parent_rect=parent_rect))
        return commands

    @final
    def do(self, result: RenderResult) -> bool:
        if InputKey.left in result.inputs_scroll:
            self.undo_stack.append((self._scroll, {"increment": True}))
            self._scroll(increment=False)
            return True
        elif InputKey.right in result.inputs_scroll:
            self.undo_stack.append((self._scroll, {"increment": False}))
            self._scroll(increment=True)
            return True
        return False

    def get_help_text(self) -> str:
        """
        :return: Text-to-speech text.
        """

        return f"{self._title}. {self.options[self.index]}. " + tooltip(keys=[InputKey.left, InputKey.right],
                                                                        predicate="cycle", boop="and") + " "

    @final
    def get_size(self) -> Tuple[int, int]:
        return self._size

    @final
    def _scroll(self, increment: bool) -> None:
        if increment:
            self.index += 1
            if self.index >= len(self.options):
                self.index = 0
        else:
            self.index -= 1
            if self.index < 0:
                self.index = len(self.options) - 1
from typing import List, Tuple, Callable
from overrides import final
from pygame import Rect
from cacophony.render.commands.command import Command
from cacophony.render.commands.text import Text
from cacophony.render.commands.border import Border
from cacophony.render.commands.arrow import Arrow
from cacophony.render.color import Color
from cacophony.render.globals import COLORS
from cacophony.render.widget.widget import Widget
from cacophony.render.input_key import InputKey
from cacophony.cardinal_direction import CardinalDirection
from cacophony.util import tooltip
from cacophony.state import State


class Options(Widget):
    """
    Cycle through a list of options.
    """

    def __init__(self, title: str, options: List[str], index: int, width: int,
                 callback: Callable = None, kwargs: dict = None):
        """
        :param title: The title text.
        :param options: A list of options.
        :param index: The index of the current option.
        :param width: The width of the widget.
        :param callback: An optional callback method.
        :param kwargs: Optional keyword arguments for the callback.
        """

        super().__init__(callback=callback, kwargs=kwargs)
        self._title: str = title
        self.options: List[str] = options
        self.index: int = index
        self._size: Tuple[int, int] = (width, 4)

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
        if widget_focus:
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
                             Arrow(position=(position[0] + self._size[0] - 2, text_y),
                                   direction=CardinalDirection.east,
                                   color=color,
                                   pivot=pivot,
                                   anchor=anchor,
                                   parent_rect=parent_rect)])
        # Show the text.
        if len(self.options) > 0:
            commands.append(Text(text=self.options[self.index],
                                 position=(text_x, text_y),
                                 size=self._size[0] - 4,
                                 text_color=color,
                                 background_color=COLORS[Color.panel_background],
                                 pivot=pivot,
                                 anchor=anchor,
                                 parent_rect=parent_rect))
        return commands

    @final
    def do(self, state: State) -> bool:
        if InputKey.left in state.result.inputs_scroll:
            self.undo_stack.append((self._scroll, {"increment": True}))
            self._scroll(increment=False)
            return True
        elif InputKey.right in state.result.inputs_scroll:
            self.undo_stack.append((self._scroll, {"increment": False}))
            self._scroll(increment=True)
            return True
        return False

    def get_help_text(self) -> str:
        """
        :return: Text-to-speech text.
        """

        text = f"{self._title}. "
        if len(self.options) > 0:
            text += f"{self.options[self.index]}. "
        text += tooltip(keys=[InputKey.left, InputKey.right], predicate="cycle", boop="and") + " "
        return text

    @final
    def get_size(self) -> Tuple[int, int]:
        return self._size

    @final
    def _scroll(self, increment: bool) -> None:
        """
        Scroll up or down.

        :param increment: Increment or decrement.
        """

        if increment:
            self.index += 1
            if self.index >= len(self.options):
                self.index = 0
        else:
            self.index -= 1
            if self.index < 0:
                self.index = len(self.options) - 1
        # Invoke the callback.
        self._invoke()

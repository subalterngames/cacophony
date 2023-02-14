from typing import List, Tuple
from cacophony.cardinal_direction import CardinalDirection
from cacophony.render.globals import COLORS
from cacophony.render.color import Color
from cacophony.render.commands.command import Command
from cacophony.render.commands.text import Text
from cacophony.render.commands.arrow import Arrow
from cacophony.render.ui_fields.ui_field import UiField


class Options(UiField):
    """
    Cycle through a list of options.
    """

    def __init__(self, title: str, font_color: Color, options: list, index: int = 0):
        super().__init__(title=title, font_color=font_color)
        self._options: list = options
        self._index: int = index

    def select(self) -> None:
        return

    def up(self) -> None:
        return

    def down(self) -> None:
        return

    def left(self) -> None:
        self._index -= 1
        if self._index < 0:
            self._index = len(self._options) - 1

    def right(self) -> None:
        self._index += 1
        if self._index >= len(self._options):
            self._index = 0

    def _render(self, position: Tuple[int, int], focus: bool, vertical: bool) -> List[Command]:
        commands = []
        # The value.
        v_text = str(self._options[self._index])
        y = position[1] + 1
        background_color = COLORS[Color.parameter_value_background_focus if focus else Color.panel_background]
        commands.append(Text(text=v_text,
                             position=(position[0] + 1, y),
                             size=len(v_text),
                             text_color=COLORS[Color.parameter_value],
                             background_color=background_color))
        # Arrows.
        if focus:
            commands.extend([Arrow(position=(position[0], y),
                                   direction=CardinalDirection.west,
                                   color=COLORS[Color.parameter_value]),
                             Arrow(position=(position[0] + len(v_text), y),
                                   direction=CardinalDirection.east,
                                   color=COLORS[Color.parameter_value])])
        return commands

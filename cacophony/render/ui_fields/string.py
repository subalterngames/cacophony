from typing import List, Tuple, Callable
from cacophony.render.color import Color
from cacophony.render.commands.command import Command
from cacophony.render.commands.text import Text
from cacophony.render.ui_fields.ui_field import UiField
from cacophony.render.globals import COLORS
from cacophony.waveform_generators.callbacker import Callbacker


class String(UiField):
    """
    A string field with a callback on select.
    """

    def __init__(self, title: str, value: Callbacker, width: int, callback: Callable):
        super().__init__(title=title, key_color=Color.parameter_key)
        self._callback: Callable = callback
        self._value: Callbacker = value
        self._width: int = width

    def select(self) -> None:
        self._callback()

    def up(self) -> None:
        return

    def down(self) -> None:
        return

    def left(self) -> None:
        return

    def right(self) -> None:
        return

    def _render(self, position: Tuple[int, int], focus: bool, vertical: bool) -> List[Command]:
        return [Text(text=self._value.get(),
                     text_color=COLORS[Color.parameter_value_background_focus if focus else Color.parameter_value],
                     background_color=COLORS[Color.parameter_value if focus else Color.parameter_value_background_focus],
                     position=(position[0], position[1] + 1),
                     size=self._width)]

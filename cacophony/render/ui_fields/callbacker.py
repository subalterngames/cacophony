from typing import List, Tuple, Callable
from cacophony.render.color import Color
from cacophony.render.commands.command import Command
from cacophony.render.ui_fields.ui_field import UiField


class Callbacker(UiField):
    """
    Callback on select.
    """

    def __init__(self, title: str, font_color: Color, callback: Callable[[], None]):
        super().__init__(title=title, font_color=font_color)
        self._callback: Callable[[], None] = callback

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
        return []

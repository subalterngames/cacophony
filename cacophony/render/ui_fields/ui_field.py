from abc import ABC, abstractmethod
from typing import List, Tuple
from overrides import final
from cacophony.render.globals import COLORS
from cacophony.render.color import Color
from cacophony.render.commands.command import Command
from cacophony.render.commands.text import Text


class UiField(ABC):
    def __init__(self, title: str, key_color: Color):
        self._title: str = title
        self._key_color: Color = key_color

    @final
    def render(self, position: Tuple[int, int], focus: bool, vertical: bool) -> List[Command]:
        commands = [Text(text=self._title,
                         text_color=COLORS[Color.panel_background] if focus else COLORS[self._key_color],
                         background_color=COLORS[self._key_color] if focus else COLORS[Color.panel_background],
                         position=position,
                         size=len(self._title))]
        commands.extend(self._render(position=position, focus=focus, vertical=vertical))
        return commands

    @abstractmethod
    def select(self) -> None:
        raise Exception()

    @abstractmethod
    def up(self) -> None:
        raise Exception()

    @abstractmethod
    def down(self) -> None:
        raise Exception()

    @abstractmethod
    def left(self) -> None:
        raise Exception()

    @abstractmethod
    def right(self) -> None:
        raise Exception()

    @abstractmethod
    def _render(self, position: Tuple[int, int], focus: bool, vertical: bool) -> List[Command]:
        raise Exception()

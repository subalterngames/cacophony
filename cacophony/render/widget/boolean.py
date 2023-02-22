from typing import List, Tuple
from pygame import Rect
from cacophony.render.commands.command import Command
from cacophony.render.commands.text import Text
from cacophony.render.color import Color
from cacophony.render.globals import COLORS
from cacophony.render.widget.label import Label
from cacophony.render.input_key import InputKey
from cacophony.render.render_result import RenderResult
from cacophony.util import tooltip


class Boolean(Label):
    """
    A text label and a boolean value.
    """

    def __init__(self, title: str, value: bool):
        """
        :param title: The label text.
        :param value: The value.
        """

        super().__init__(text=title, size=len(title) + 5)
        self.value: bool = value

    def blit(self, position: Tuple[int, int], panel_focus: bool, element_focus: bool, pivot: Tuple[float, float] = None,
             anchor: Tuple[float, float] = None, parent_rect: Rect = None) -> List[Command]:
        # Blit the label.
        commands = super().blit(position=position, panel_focus=panel_focus, element_focus=element_focus, pivot=pivot,
                                anchor=anchor, parent_rect=parent_rect)
        # Show the box.
        if not panel_focus:
            c = Color.border_no_focus
        elif self.value:
            c = Color.parameter_boolean_true
        else:
            c = Color.parameter_boolean_false
        commands.append(Text(text="Y" if self.value else "N",
                             position=(position[0] + len(self._text) + 2, position[1] + 1),
                             text_color=COLORS[c],
                             background_color=COLORS[Color.panel_background],
                             pivot=pivot,
                             anchor=anchor,
                             parent_rect=parent_rect))
        return commands

    def do(self, result: RenderResult) -> bool:
        if InputKey.select in result.inputs_pressed:
            self.value = not self.value
            return True
        return False

    def get_help_text(self) -> str:
        return f"{self.get_size}. Set to {self.value}. " + tooltip(keys=[InputKey.select], predicate="set")  + " "
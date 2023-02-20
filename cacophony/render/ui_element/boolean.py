from typing import List, Tuple
from pygame import Rect
from cacophony.render.commands.command import Command
from cacophony.render.commands.border import Border
from cacophony.render.commands.rectangle import Rectangle
from cacophony.render.color import Color
from cacophony.render.globals import COLORS
from cacophony.render.ui_element.label import Label


class Boolean(Label):
    """
    A text label and a boolean value.
    """

    def __init__(self, text: str, value: bool):
        """
        :param text: The label text.
        :param value: The value.
        """

        super().__init__(text=text, size=len(text) + 3)
        self.value: bool = value

    def blit(self, position: Tuple[int, int], panel_focus: bool, element_focus: bool, pivot: Tuple[float, float] = None,
             anchor: Tuple[float, float] = None, parent_rect: Rect = None) -> List[Command]:
        # Blit the label.
        commands = super().blit(position=position, panel_focus=panel_focus, element_focus=element_focus, pivot=pivot,
                                anchor=anchor, parent_rect=parent_rect)
        # Show the box.
        if not panel_focus:
            c = Color.border_no_focus
        elif element_focus:
            c = Color.parameter_value
        else:
            c = Color.border_focus
        color = COLORS[c]
        box_position: Tuple[int, int] = (position[0] + len(self._text) + 2, position[1] + 1)
        # A filled box.
        if self.value:
            commands.append(Rectangle(position=box_position,
                                      size=(1, 1),
                                      color=color,
                                      pivot=pivot,
                                      anchor=anchor,
                                      parent_rect=parent_rect))
        # An unfilled box.
        else:
            commands.extend([Rectangle(position=box_position,
                                       size=(1, 1),
                                       color=COLORS[Color.panel_background],
                                       pivot=pivot,
                                       anchor=anchor,
                                       parent_rect=parent_rect),
                             Border(position=box_position,
                                    size=(1, 1),
                                    color=color,
                                    pivot=pivot,
                                    anchor=anchor,
                                    parent_rect=parent_rect)])
        return commands

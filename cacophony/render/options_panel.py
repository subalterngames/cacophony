from abc import ABC
from typing import Tuple, List, Optional
from cacophony.render.renderer import Renderer
from cacophony.render.color import Color
from cacophony.render.globals import COLORS
from cacophony.render.commands.command import Command
from cacophony.render.commands.text import Text
from cacophony.render.commands.rectangle import Rectangle
from cacophony.render.commands.border import Border
from cacophony.render.ui_fields.ui_field import UiField
from cacophony.render.input_key import InputKey


class OptionsPanel(Renderer, ABC):
    def __init__(self, position: Tuple[int, int], size: Tuple[int, int], vertical: bool, panel_focus: bool, fields: List[UiField], title: str = None):
        super().__init__()
        self.panel_focus: bool = panel_focus
        self._position: Tuple[int, int] = position
        self._size: Tuple[int, int] = size
        self._vertical: bool = vertical
        self._title: Optional[str] = title
        self._fields: List[UiField] = fields
        self._field_focus: int = 0

    def do(self) -> None:
        result = self.render(self.show_panel())
        while self.panel_focus:
            # Key down to cycle fields or options.
            if InputKey.up in result.inputs_pressed:
                if self._vertical:
                    self._decrement()
                self._fields[self._field_focus].up()
            if InputKey.down in result.inputs_pressed:
                if self._vertical:
                    self._increment()
                self._fields[self._field_focus].down()
            if InputKey.left in result.inputs_pressed:
                if not self._vertical:
                    self._increment()
                self._fields[self._field_focus].left()
            if InputKey.right in result.inputs_pressed:
                if not self._vertical:
                    self._decrement()
                self._fields[self._field_focus].right()
            # Select.
            if InputKey.select in result.inputs_pressed:
                self._fields[self._field_focus].select()
            # Listen to held input.
            if InputKey.up in result.inputs_held:
                self._fields[self._field_focus].up()
            if InputKey.down in result.inputs_held:
                self._fields[self._field_focus].down()
            if InputKey.left in result.inputs_held:
                self._fields[self._field_focus].left()
            if InputKey.right in result.inputs_held:
                self._fields[self._field_focus].right()
            if InputKey.next_panel in result.inputs_pressed or InputKey.previous_panel in result.inputs_pressed:
                self.panel_focus = False
            # Redraw the panel.
            if len(result.inputs_pressed) > 0 or len(result.inputs_held) > 0:
                result = self.render(self.show_panel())
            # Do nothing and loop.
            else:
                result = self.render([])

    def show_panel(self) -> List[Command]:
        border_color = COLORS[Color.border_focus if self.panel_focus else Color.border_no_focus]
        background_color = COLORS[Color.panel_background]
        # Clear the previous panel by drawing a rectangle with the background color.
        commands = [Rectangle(position=self._position, size=self._size, color=background_color)]
        y = self._position[1]
        # Add the title.
        if self._title is not None:
            commands.extend([Rectangle(position=self._position, size=self._size, color=border_color),
                             Text(text=self._title,
                                  position=(self._position[0] + 3, y),
                                  size=self._size[0],
                                  text_color=COLORS[Color.panel_title_focus if self.panel_focus else Color.panel_title_no_focus],
                                  background_color=COLORS[Color.border_focus if self.panel_focus else Color.border_no_focus])])
            y += 2
        # Add the fields.
        x = self._position[0] + 2
        dx = (self._size[0] - 4) // len(self._fields)
        for i, field in enumerate(self._fields):
            commands.extend(self._fields[i].render(position=(x, y), focus=i == self._field_focus, vertical=self._vertical))
            if self._vertical:
                y += 2
            else:
                x += dx
        # Add a border.
        commands.append(Border(position=(0, 0), size=self._size, color=border_color))
        return commands

    def _increment(self) -> None:
        self._field_focus += 1
        if self._field_focus >= len(self._fields):
            self._field_focus = 0

    def _decrement(self) -> None:
        self._field_focus -= 1
        if self._field_focus < 0:
            self._field_focus = len(self._fields) - 1

job342:6

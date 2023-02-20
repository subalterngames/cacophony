from typing import List
from cacophony.render.commands.command import Command
from cacophony.render.commands.text import Text
from cacophony.render.commands.rectangle import Rectangle
from cacophony.render.globals import COLORS, WINDOW_GRID_WIDTH, INPUT_KEYS
from cacophony.render.color import Color
from cacophony.render.panel.panel import Panel
from cacophony.render.input_key import InputKey


class MainMenu(Panel):
    """
    The top-bar main menu for the program.
    """

    def __init__(self):
        """
        (no parameters)
        """

        super().__init__(title="Cacophony", position=(0, 0), size=(WINDOW_GRID_WIDTH, 3), anchor=(0, 0), pivot=(0, 0),
                         parent_rect=None)
        help_texts = []
        for help_key in [InputKey.panel_help, InputKey.widget_help, InputKey.app_help]:
            help_texts.append("[" + ", ".join([str(v).capitalize() for v in INPUT_KEYS[help_key]]) + " " + help_key.name.split("_")[0].title() + "]")
        self._help_text: str = "Help: " + " ".join(help_texts)

    def blit(self, focus: bool) -> List[Command]:
        commands = super().blit(focus=focus)
        commands.extend([Rectangle(position=(-2, self._position[1]),
                                   size=(len(self._help_text) + 2, 1),
                                   color=COLORS[Color.panel_background],
                                   pivot=(1, 0),
                                   anchor=(1, 0),
                                   parent_rect=None),
                         Text(text=self._help_text,
                              position=(-3, 0),
                              text_color=COLORS[Color.border_focus],
                              background_color=COLORS[Color.panel_background],
                              pivot=(1, 0),
                              anchor=(1, 0),
                              parent_rect=None)])
        return commands

    def get_panel_help(self) -> str:
        return "Main menu"

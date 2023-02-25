from typing import List
from cacophony.render.commands.command import Command
from cacophony.render.commands.text import Text
from cacophony.render.commands.rectangle import Rectangle
from cacophony.render.globals import COLORS, INPUT_KEYS, LAYOUTS
from cacophony.render.color import Color
from cacophony.render.panel.panel import Panel
from cacophony.render.panel.panel_type import PanelType
from cacophony.render.input_key import InputKey


class MainMenu(Panel):
    """
    The top-bar main menu for the program.
    """

    def __init__(self):
        """
        (no parameters)
        """

        layout = LAYOUTS[self.__class__.__name__]
        super().__init__(title="Cacophony",
                         position=(layout[0], layout[1]),
                         size=(layout[2], layout[3]),
                         anchor=(0, 0), pivot=(0, 0),
                         parent_rect=None)
        help_texts = []
        for help_key in [InputKey.panel_help, InputKey.widget_help, InputKey.app_help, InputKey.stop_tts]:
            help_texts.append("[" + ", ".join([str(v).capitalize() for v in INPUT_KEYS[help_key]]) + " " + help_key.name.split("_")[0].title() + "]")
        self._help_text: str = "Help: " + " ".join(help_texts)

    def get_panel_help(self) -> str:
        return "Main menu"

    def get_panel_type(self) -> PanelType:
        return PanelType.main_menu

    def _render_panel(self, focus: bool) -> List[Command]:
        commands = super()._render_panel(focus=focus)
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

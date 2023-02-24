from typing import List, Tuple
from pathlib import Path
from overrides import final
from pygame import Rect
from cacophony.render.commands.command import Command
from cacophony.render.commands.text import Text
from cacophony.render.commands.border import Border
from cacophony.render.color import Color
from cacophony.render.globals import COLORS
from cacophony.render.widget.widget import Widget
from cacophony.render.render_result import RenderResult
from cacophony.render.input_key import InputKey
from cacophony.util import tooltip, get_string_path


class FilePrompt(Widget):
    """
    A prompt that show a file path and a "button" to change it.
    """

    def __init__(self, path: str, width: int, suffixes: List[str]):
        """
        :param path: The path.
        :param width: The width of the widget.
        :param suffixes: Valid file suffixes.
        """

        super().__init__()
        self.path: str = path
        self._suffixes: List[str] = suffixes
        self._size: Tuple[int, int] = (width, 4)

    def blit(self, position: Tuple[int, int], panel_focus: bool, element_focus: bool, pivot: Tuple[float, float] = None,
             anchor: Tuple[float, float] = None, parent_rect: Rect = None) -> List[Command]:
        if not panel_focus:
            c = Color.border_no_focus
        elif element_focus:
            c = Color.parameter_value
        else:
            c = Color.border_focus
        color = COLORS[c]
        commands = []
        # Show the border.
        if element_focus:
            commands.append(Border(position=position,
                                   size=self._size,
                                   color=color,
                                   pivot=pivot,
                                   anchor=anchor,
                                   parent_rect=parent_rect))
        button_text = "file:"
        # Get the colors for the button.
        if panel_focus:
            if element_focus:
                button_background_color = Color.border_focus
                button_text_color = Color.parameter_value
            else:
                button_background_color = Color.panel_background
                button_text_color = Color.border_focus
        else:
            button_background_color = Color.panel_background
            button_text_color = Color.border_no_focus
        # Draw the button text.
        commands.append(Text(text=button_text,
                             position=(position[0] + 1, position[1] + 1),
                             text_color=COLORS[button_text_color],
                             background_color=COLORS[button_background_color],
                             pivot=pivot,
                             anchor=anchor,
                             parent_rect=parent_rect))
        # Show the path.
        if len(self.path) > 0:
            commands.append(Text(text=self.path,
                                 size=self._size[0] - 2,
                                 position=(position[0] + 1, position[1] + 2),
                                 text_color=color,
                                 background_color=COLORS[Color.panel_background],
                                 truncate_left=False,
                                 pivot=pivot,
                                 anchor=anchor,
                                 parent_rect=parent_rect))
        return commands

    @final
    def get_size(self) -> Tuple[int, int]:
        return self._size

    def do(self, result: RenderResult) -> bool:
        # Open the file.
        if InputKey.select in result.inputs_pressed:
            # Open the prompt.
            from cacophony.render.panel.open_file import OpenFile
            from cacophony.render.renderer import Renderer
            r = Renderer()
            result = r.render([])
            of = OpenFile(suffixes=self._suffixes)
            while not of.done:
                result = r.render(of.render(result=result, focus=True))
            # The path changed. Set my path and return True.
            if of.path is not None:
                str_path = get_string_path(of.path)
                if str_path != self.path:
                    self.path = str_path
                    return True
        return False

    def get_help_text(self) -> str:
        """
        :return: Text-to-speech text.
        """

        text = tooltip(keys=[InputKey.select], predicate="open file") + " "
        if len(self.path) > 0:
            text += f"The current file is {Path(self.path).name}. "
        return text

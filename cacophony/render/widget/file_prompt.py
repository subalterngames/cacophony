from typing import List, Tuple, Callable
from pathlib import Path
from overrides import final
from pygame import Rect
from cacophony.render.commands.command import Command
from cacophony.render.commands.text import Text
from cacophony.render.commands.border import Border
from cacophony.render.color import Color
from cacophony.render.globals import COLORS
from cacophony.render.widget.widget import Widget
from cacophony.render.input_key import InputKey
from cacophony.render.panel.panel_type import PanelType
from cacophony.util import tooltip
from cacophony.state import State


class FilePrompt(Widget):
    """
    A prompt that show a file path and a "button" to change it.
    """

    def __init__(self, path: str, width: int, suffixes: List[str], callback: Callable = None, kwargs: dict = None):
        """
        :param path: The path.
        :param width: The width of the widget.
        :param suffixes: Valid file suffixes.
        :param callback: An optional callback method.
        :param kwargs: Optional keyword arguments for the callback.
        """

        super().__init__(callback=callback, kwargs=kwargs)
        self.path: str = path
        self._suffixes: List[str] = suffixes
        self._size: Tuple[int, int] = (width, 4)

    def blit(self, position: Tuple[int, int], panel_focus: bool, widget_focus: bool, pivot: Tuple[float, float] = None,
             anchor: Tuple[float, float] = None, parent_rect: Rect = None) -> List[Command]:
        if not panel_focus:
            c = Color.border_no_focus
        elif widget_focus:
            c = Color.parameter_value
        else:
            c = Color.border_focus
        color = COLORS[c]
        commands = []
        # Show the border.
        if widget_focus:
            commands.append(Border(position=position,
                                   size=self._size,
                                   color=color,
                                   pivot=pivot,
                                   anchor=anchor,
                                   parent_rect=parent_rect))
        button_text = "file:"
        # Get the colors for the button.
        if panel_focus:
            if widget_focus:
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
            commands.append(Text(text=Path(self.path).name,
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

    def do(self, state: State) -> bool:
        # Open the file.
        if InputKey.select in state.result.inputs_pressed:
            # Set the suffixes.
            state.open_file_state.suffixes.clear()
            state.open_file_state.suffixes.extend(self._suffixes)
            # When we're done opening the file, set the path.
            state.open_file_state.callback = self._set_path
            # Mark the focused panel as dirty.
            state.dirty_panels.append(state.focused_panel)
            # Remember the focused panel.
            state.open_file_state.previous_focus = state.focused_panel
            # Remember the active panels.
            state.open_file_state.previous_active.clear()
            state.open_file_state.previous_active.extend(state.active_panels)
            # Clear the active panels and open the open file panel.
            state.active_panels.clear()
            state.active_panels.append(PanelType.open_file)
            state.dirty_panels.append(PanelType.open_file)
            state.focused_panel = PanelType.open_file
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

    def _set_path(self, path: str) -> None:
        self.path = path
        self._invoke()

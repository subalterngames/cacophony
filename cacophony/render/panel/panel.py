from typing import List, Tuple, Callable
from overrides import final
from pygame.display import get_surface
from pygame import Rect
from cacophony.render.commands.command import Command
from cacophony.render.commands.border import Border
from cacophony.render.commands.rectangle import Rectangle
from cacophony.render.commands.text import Text
from cacophony.render.globals import COLORS
from cacophony.render.color import Color
from cacophony.render.macros.parent_rect import get_parent_rect
from cacophony.render.render_result import RenderResult
from cacophony.render.input_key import InputKey
from cacophony.text_to_speech import TextToSpeech


class Panel:
    """
    A rectangular panel with a title.
    """

    def __init__(self, title: str, position: Tuple[int, int], size: Tuple[int, int], pivot: Tuple[float, float] = None,
                 anchor: Tuple[float, float] = None, parent_rect: Rect = None):
        """
        :param title: The title text.
        :param position: The position of the panel in grid coordinates.
        :param size: The size of the panel in grid coordinates.
        :param pivot: The pivot as a (x, y) 0-1 tuple. If the pivot is `(0, 0)`, then it is located at the top-left of the surface we're about to render. `(0, 1)` would be bottom-left, etc.
        :param anchor: The anchor as a (x, y) 0-1 tuple. If the anchor is `(0, 0)`, there is no offset. If the anchor is `(1, 0)`, then the position would be offset by the pivot offset plus the width of the screen.
        :param parent_rect: The parent rect that pivot and anchor are relative to. If None, this defaults to the display surface rect.
        """

        self._title: str = title
        self._position: Tuple[int, int] = position
        self._size: Tuple[int, int] = size
        if pivot is None:
            self._pivot: Tuple[float, float] = (0, 0)
        else:
            self._pivot = pivot
        if anchor is None:
            self._anchor: Tuple[float, float] = (0, 0)
        else:
            self._anchor = anchor
        if parent_rect is None:
            self._parent_rect: Rect = get_surface().get_rect()
        else:
            self._parent_rect = parent_rect
        self._title_text_rect: Rect = get_parent_rect(position=(0, 0),
                                                      size=size,
                                                      pivot=pivot,
                                                      anchor=anchor,
                                                      grandparent_rect=self._parent_rect)
        self.initialized: bool = False
        # A stack of undo operations. Each element is a tuple: Callbable, kwargs.
        self.undo_stack: List[Tuple[Callable, dict]] = list()

    @final
    def render(self, result: RenderResult, focus: bool) -> List[Command]:
        """
        Process user input and blit the panel.

        :param result: The [`RenderResult`](render_result.md) from the previous frame.
        :param focus: If True, the panel has focus.

        :return: A list of commands.
        """

        if InputKey.panel_help in result.inputs_pressed:
            TextToSpeech.say(self.get_panel_help())
        elif InputKey.widget_help in result.inputs_pressed:
            TextToSpeech.say(self.get_widget_help())
        rerender = False
        if not self.initialized:
            self.initialized = True
            rerender = True
        elif focus and self._do_result(result=result):
            rerender = True
        if rerender:
            return self._render_panel(focus=focus)
        else:
            return []

    def _render_panel(self, focus: bool) -> List[Command]:
        """
        Blit the panel.

        :param focus: If True, the panel has focus.

        :return: A list of commands.
        """

        return [Rectangle(position=self._position,
                          size=self._size,
                          color=COLORS[Color.panel_background],
                          pivot=self._pivot,
                          anchor=self._anchor,
                          parent_rect=self._parent_rect),
                Border(position=self._position,
                       size=self._size,
                       color=COLORS[Color.border_focus if focus else Color.border_no_focus],
                       pivot=self._pivot,
                       anchor=self._anchor,
                       parent_rect=self._parent_rect),
                Rectangle(position=(self._position[0] + 2, self._position[1]),
                          size=(len(self._title) + 2, 1),
                          color=COLORS[Color.panel_background],
                          parent_rect=self._title_text_rect),
                Text(text=self._title,
                     position=(self._position[0] + 3, self._position[1]),
                     text_color=COLORS[Color.border_focus if focus else Color.border_no_focus],
                     background_color=COLORS[Color.panel_background],
                     parent_rect=self._title_text_rect)]

    def get_panel_help(self) -> str:
        """
        :return: Panel help text.
        """

        return self._title

    def get_widget_help(self) -> str:
        """
        :return: Help text for the focused widget (if any).
        """

        return ""

    def _do_result(self, result: RenderResult) -> bool:
        """
        :param result: The `RenderResult` from the previous frame.

        :return: True if the panel needs to be re-rendered.
        """

        return False

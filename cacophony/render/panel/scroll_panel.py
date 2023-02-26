from abc import ABC, abstractmethod
from typing import List, Tuple
from overrides import final
from pygame import Rect
from cacophony.render.commands.command import Command
from cacophony.render.commands.arrow import Arrow
from cacophony.render.globals import COLORS
from cacophony.render.color import Color
from cacophony.render.panel.panel import Panel
from cacophony.render.input_key import InputKey
from cacophony.render.widget.widget import Widget
from cacophony.render.macros.parent_rect import get_parent_rect
from cacophony.util import tooltip
from cacophony.cardinal_direction import CardinalDirection
from cacophony.state import State


class ScrollPanel(Panel, ABC):
    """
    A panel with scrollable elements.
    """

    def __init__(self, title: str, position: Tuple[int, int],
                 size: Tuple[int, int], pivot: Tuple[float, float] = None, anchor: Tuple[float, float] = None,
                 parent_rect: Rect = None):
        """
        :param title: The title text.
        :param position: The position of the panel in grid coordinates.
        :param size: The size of the panel in grid coordinates.
        :param pivot: The pivot as a (x, y) 0-1 tuple. If the pivot is `(0, 0)`, then it is located at the top-left of the surface we're about to render. `(0, 1)` would be bottom-left, etc.
        :param anchor: The anchor as a (x, y) 0-1 tuple. If the anchor is `(0, 0)`, there is no offset. If the anchor is `(1, 0)`, then the position would be offset by the pivot offset plus the width of the screen.
        :param parent_rect: The parent rect that pivot and anchor are relative to. If None, this defaults to the display surface rect.
        """

        super().__init__(title=title, position=position, size=size, pivot=pivot, anchor=anchor, parent_rect=parent_rect)
        self._pages: List[List[Widget]] = list()
        self._page_index: int = 0
        self._widget_page_index: int = 0

    def _do_result(self, state: State) -> bool:
        if len(self._widgets) == 0:
            return False
        if InputKey.up in state.result.inputs_scroll:
            # Scroll up a page.
            if self._widget_page_index == 0 and self._page_index > 0:
                self._scroll(state=state,
                             page_index_delta=-1,
                             widget_index=len(self._pages[self._page_index]) - 1,
                             widget_index_delta=False,
                             selection_index_delta=-1)
                return True
            # Scroll up an element.
            elif self._widget_page_index > 0:
                self._scroll(state=state,
                             page_index_delta=0,
                             widget_index=-1,
                             widget_index_delta=True,
                             selection_index_delta=-1)
                return True
        elif InputKey.down in state.result.inputs_scroll:
            # Scroll down a page.
            if self._widget_page_index == len(self._pages[self._page_index]) - 1 and self._page_index < len(self._pages) - 1:
                self._scroll(state=state,
                             page_index_delta=1,
                             widget_index=0,
                             widget_index_delta=False,
                             selection_index_delta=1)
                return True
            # Scroll down an element.
            elif self._widget_page_index < len(self._pages[self._page_index]) - 1:
                self._scroll(state=state,
                             page_index_delta=0,
                             widget_index=1,
                             widget_index_delta=True,
                             selection_index_delta=1)
                return True
        return False

    @final
    def _render_panel(self, state: State, focus: bool) -> List[Command]:
        # Initialize by populating pages.
        if self.do_render:
            self._set(state=state)
        # Blit the panel.
        commands = super()._render_panel(state=state, focus=focus)
        if len(self._pages) == 0:
            return commands
        parent_rect = get_parent_rect(position=(self._position[0] + 1, self._position[1] + 1),
                                      size=self._size,
                                      pivot=self._pivot,
                                      anchor=self._anchor)
        y = 0
        # Blit each element in the page.
        for i, element in enumerate(self._pages[self._page_index]):
            commands.extend(element.blit(position=(0, y),
                                         panel_focus=focus,
                                         widget_focus=i == self._focused_widget_index,
                                         pivot=self._pivot,
                                         anchor=self._anchor,
                                         parent_rect=parent_rect))
            y += element.get_size()[1]
        # Blit arrows.
        arrow_x = self._size[0] - 4
        arrow_color = COLORS[Color.border_focus if focus else Color.border_no_focus]
        if self._page_index > 0:
            commands.append(Arrow(color=arrow_color,
                                  direction=CardinalDirection.north,
                                  position=(arrow_x, 0),
                                  pivot=self._pivot,
                                  anchor=self._anchor,
                                  parent_rect=parent_rect))
        if self._page_index < len(self._pages) - 1:
            commands.append(Arrow(color=arrow_color,
                                  direction=CardinalDirection.south,
                                  position=(arrow_x, self._size[1] - 2),
                                  pivot=self._pivot,
                                  anchor=self._anchor,
                                  parent_rect=parent_rect))
        return commands

    @final
    def _scroll(self, state: State, page_index_delta: int, widget_index: int, widget_index_delta: bool, selection_index_delta: int) -> None:
        """
        Scroll. Apply deltas to each index.

        :param state: The `State` of the program.
        :param page_index_delta: The page index delta.
        :param widget_index: The element index.
        :param widget_index_delta: If True, `widget_index` is a delta.
        :param selection_index_delta: The selection index delta.
        """

        track_index: int = state.track_index
        state.undo_stack.append((state.set_track_index, {"track_index": track_index}))
        self._page_index += page_index_delta
        if widget_index_delta:
            self._widget_page_index += widget_index
        else:
            self._widget_page_index = widget_index
        self._focused_widget_index += selection_index_delta

    def get_panel_help(self, state: State) -> str:
        return self._get_panel_title(state=state) + ". " + tooltip(keys=[InputKey.up, InputKey.down], predicate="scroll", boop="and")

    def get_widget_help(self, state: State) -> str:
        return self._widgets[self._focused_widget_index].get_help_text()

    def _get_panel_title(self, state: State) -> str:
        return "Scroll panel"

    @final
    def _set(self, state: State) -> None:
        """
        Set the scrollable panel.

        :param state: The `State` of the program.
        """

        self._widgets.clear()
        self._set_widgets(state=state)
        self._populate_pages()

    @abstractmethod
    def _set_widgets(self, state: State) -> None:
        """
        Set the widgets in the panel.

        :param state: The `State` of the program.
        """

        raise Exception()

    def _populate_pages(self) -> None:
        """
        Populate the pages lists.
        """

        self._pages.clear()
        max_h = self._size[1] - 2
        page: List[Widget] = list()
        page_h = 0
        for element in self._widgets:
            element_h = element.get_size()[1]
            # Exceeded the max height. End the page.
            if page_h + element_h > max_h:
                self._pages.append(page[:])
                page_h = 0
                page.clear()
            # Append to the page.
            page.append(element)
            page_h += element_h
        if len(page) > 0:
            self._pages.append(page)
        self._page_index = 0
        self._widget_page_index = 0
        self._focused_widget_index = 0

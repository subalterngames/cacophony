from typing import List, Tuple
from overrides import final
from pygame import Rect
from cacophony.render.commands.command import Command
from cacophony.render.commands.arrow import Arrow
from cacophony.render.globals import COLORS
from cacophony.render.color import Color
from cacophony.render.panel.panel import Panel
from cacophony.render.ui_element.ui_element import UiElement
from cacophony.render.macros.parent_rect import get_parent_rect
from cacophony.cardinal_direction import CardinalDirection


class ScrollPanel(Panel):
    """
    A panel with scrollable elements.
    """

    def __init__(self, elements: List[UiElement], title: str, position: Tuple[int, int],
                 size: Tuple[int, int], pivot: Tuple[float, float] = None, anchor: Tuple[float, float] = None,
                 parent_rect: Rect = None):
        """
        :param elements: A list of UI elements.
        :param title: The title text.
        :param position: The position of the panel in grid coordinates.
        :param size: The size of the panel in grid coordinates.
        :param pivot: The pivot as a (x, y) 0-1 tuple. If the pivot is `(0, 0)`, then it is located at the top-left of the surface we're about to render. `(0, 1)` would be bottom-left, etc.
        :param anchor: The anchor as a (x, y) 0-1 tuple. If the anchor is `(0, 0)`, there is no offset. If the anchor is `(1, 0)`, then the position would be offset by the pivot offset plus the width of the screen.
        :param parent_rect: The parent rect that pivot and anchor are relative to. If None, this defaults to the display surface rect.
        """

        super().__init__(title=title, position=position, size=size, pivot=pivot, anchor=anchor, parent_rect=parent_rect)
        self._elements: List[UiElement] = elements
        self._pages: List[List[UiElement]] = list()
        max_h = self._size[1] - 2
        page: List[UiElement] = list()
        page_h = 0
        for element in elements:
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
        self._page_index: int = 0
        self._element_index: int = 0
        self._selection_index: int = 0

    @final
    def scroll(self, up: bool) -> int:
        """
        Increment or decrement the selection.
        :param up: If True, we're scrolling up. If False, we're scrolling down.

        :return: The selection index.
        """

        if up:
            # Scroll up a page.
            if self._element_index == 0 and self._page_index > 0:
                self._page_index -= 1
                self._element_index = len(self._pages[self._page_index]) - 1
                self._selection_index -= 1
            # Scroll up an element.
            elif self._element_index > 0:
                self._element_index -= 1
                self._selection_index -= 1
        else:
            # Scroll down a page.
            if self._element_index == len(self._pages[self._page_index]) - 1 and self._page_index < len(self._pages) - 1:
                self._page_index += 1
                self._element_index = 0
                self._selection_index += 1
            # Scroll down an element.
            elif self._element_index < len(self._pages[self._page_index]) - 1:
                self._element_index += 1
                self._selection_index += 1
        return self._selection_index

    def blit(self, focus: bool) -> List[Command]:
        # Blit the panel.
        commands = super().blit(focus=focus)
        parent_rect = get_parent_rect(position=(1, 4),
                                      size=self._size,
                                      pivot=self._pivot,
                                      anchor=self._anchor)
        y = 0
        # Blit each element in the page.
        for i, element in enumerate(self._pages[self._page_index]):
            commands.extend(element.blit(position=(0, y),
                                         panel_focus=focus,
                                         element_focus=i == self._element_index,
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

    def get_help_text(self) -> str:
        """
        :return: Terse help text.
        """

        return f"Scroll panel {self._title}. Selection is {self._elements[self._selection_index].get_help_text()}"

    def get_verbose_help_text(self) -> str:
        """
        :return: Verbose help text.
        """

        text = f"{super().get_help_text()}"
        if self._selection_index > 0:
            text += "You can scroll up."
        if self._selection_index < len(self._elements) - 1:
            text += "You can scroll down."
        return text

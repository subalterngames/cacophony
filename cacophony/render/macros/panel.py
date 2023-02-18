from typing import List, Tuple
from pygame import Rect
from cacophony.render.commands.command import Command
from cacophony.render.commands.border import Border
from cacophony.render.commands.rectangle import Rectangle
from cacophony.render.commands.text import Text
from cacophony.render.globals import COLORS
from cacophony.render.color import Color
from cacophony.render.macros.parent_rect import get_parent_rect


def panel(position: Tuple[int, int], size: Tuple[int, int], focus: bool, pivot: Tuple[float, float] = None,
          anchor: Tuple[float, float] = None, parent_rect: Rect = None, title: str = None, border: bool = True) -> List[Command]:
    """
    :param position: The position of the panel in grid coordinates.
    :param size: The size of the panel in grid coordinates.
    :param focus: If True, this panel has focus. This affects the panel's colors.
    :param pivot: The pivot as a (x, y) 0-1 tuple. If the pivot is `(0, 0)`, then it is located at the top-left of the surface we're about to render. `(0, 1)` would be bottom-left, etc.
    :param anchor: The anchor as a (x, y) 0-1 tuple. If the anchor is `(0, 0)`, there is no offset. If the anchor is `(1, 0)`, then the position would be offset by the pivot offset plus the width of the screen.
    :param parent_rect: The parent rect that pivot and anchor are relative to. If None, this defaults to the display surface rect.
    :param title: The text of the panel's title. This will be in the top-left on top of the border. Ignored if `title` is None or `border` is False.
    :param border: If True, blit a border.

    :return: A list of commands to blit a panel.
    """

    text_rect: Rect = get_parent_rect(position=(0, 0),
                                      size=size,
                                      pivot=pivot,
                                      anchor=anchor,
                                      grandparent_rect=parent_rect)
    # Blit the panel background.
    commands = [Rectangle(position=position,
                          size=size,
                          color=COLORS[Color.panel_background],
                          pivot=pivot,
                          anchor=anchor,
                          parent_rect=parent_rect)]
    # Blit the border.
    if border:
        commands.append(Border(position=position,
                               size=size,
                               color=COLORS[Color.border_focus if focus else Color.border_no_focus],
                               pivot=pivot,
                               anchor=anchor,
                               parent_rect=parent_rect))
        # Blit the title.
        if title is not None:
            commands.extend([Rectangle(position=(position[0] + 2, position[1]),
                                       size=(len(title) + 2, 1),
                                       color=COLORS[Color.panel_background],
                                       parent_rect=text_rect),
                             Text(text=title,
                                  position=(position[0] + 3, position[1]),
                                  text_color=COLORS[Color.border_focus if focus else Color.border_no_focus],
                                  background_color=COLORS[Color.panel_background],
                                  parent_rect=text_rect)])
    return commands

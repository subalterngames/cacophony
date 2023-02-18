from typing import List
from cacophony.music.track import Track
from cacophony.render.color import Color
from cacophony.render.globals import WINDOW_GRID_WIDTH, WINDOW_GRID_HEIGHT, COLORS
from cacophony.render.commands.command import Command
from cacophony.render.commands.text import Text
from cacophony.render.commands.border import Border
from cacophony.render.macros.panel import panel
from cacophony.render.macros.parent_rect import get_parent_rect


def tracks_list(tracks: List[Track], selected_track_index: int, focus: bool) -> List[Command]:
    """
    :param tracks: A list of music tracks.
    :param selected_track_index: The index of the selected track.
    :param focus: If True, this panel has focus.

    :return: A list of commands to show the tracks.
    """

    x = 0
    y = 3
    pivot = (0, 0)
    anchor = (0, 0)
    panel_size = (WINDOW_GRID_WIDTH // 7,  WINDOW_GRID_HEIGHT - y)
    # Show the panel for the list.
    commands = panel(position=(x, y),
                     size=panel_size,
                     focus=focus,
                     border=True,
                     title="Tracks [ctrl+1]",
                     pivot=pivot,
                     anchor=anchor)
    # Show each track.
    tracks_rect = get_parent_rect(position=(x, y),
                                  size=panel_size,
                                  pivot=pivot,
                                  anchor=anchor)
    y = 1
    for i, track in enumerate(tracks):
        if not focus:
            c = Color.border_no_focus
        elif i == selected_track_index:
            c = Color.parameter_value
        else:
            c = Color.border_focus
        color = COLORS[c]
        # Show the track border.
        if i == selected_track_index:
            commands.append(Border(position=(1, y),
                                   size=(panel_size[0] - 2, 3),
                                   color=color,
                                   pivot=pivot,
                                   anchor=anchor,
                                   parent_rect=tracks_rect))
        commands.append(Text(text=track.synthesizer.__class__.__name__,
                             position=(2, y + 1),
                             text_color=color,
                             background_color=COLORS[Color.panel_background],
                             pivot=pivot,
                             anchor=anchor,
                             parent_rect=tracks_rect))
        y += 3
    return commands

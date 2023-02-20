from typing import List, Tuple
from pygame.midi import midi_to_ansi_note
from cacophony.render.commands.command import Command
from cacophony.render.commands.arrow import Arrow
from cacophony.render.commands.text import Text
from cacophony.render.commands.line import Line
from cacophony.render.commands.piano_roll_note import PianoRollNote
from cacophony.render.globals import COLORS
from cacophony.render.color import Color
from cacophony.render.panel.panel import Panel
from cacophony.cardinal_direction import CardinalDirection
from cacophony.music.track import Track


class PianoRoll(Panel):
    """
    A piano roll view of a track.
    """

    _NOTE_NAME_WIDTH: int = 3

    def __init__(self, track: Track, selected_note: int, time_0: int, note_0: int, position: Tuple[int, int],
                 size: Tuple[int, int]):
        """
        :param track: The music track.
        :param selected_note: The index of the selected note.
        :param time_0: The minimum time in beats to be visualized in this panel.
        :param note_0: The minimum MIDI note value to be visualized in this panel.
        :param position: The position of the panel in grid coordinates.
        :param size: The size of the panel in grid coordinates.
        """

        super().__init__(title=f"Piano Roll t0={time_0} [- track roll]", position=position, size=size, pivot=(0, 0), anchor=(0, 0),
                         parent_rect=None)
        self._track: Track = track
        self._selected_note: int = selected_note
        self._time_0: int = time_0
        self._note_0: int = note_0

    def blit(self, focus: bool) -> List[Command]:
        commands = super().blit(focus=focus)
        x0 = self._position[0] + PianoRoll._NOTE_NAME_WIDTH + 1
        x1 = self._position[0] + self._size[0] - 2
        y0 = self._position[1] + 1
        h = self._size[1] - 2
        w = x1 - x0
        time_1 = self._time_0 + w
        note_1 = self._note_0 + h
        note_name_x = self._position[0] + 1
        note_y = y0
        # Get the range of notes.
        note_range = list(range(self._note_0, self._note_0 + h))
        # Reverse the order.
        note_range.reverse()
        for note_value in note_range:
            # Blit the note name and the horizontal line.
            commands.extend([Text(text=midi_to_ansi_note(note_value),
                                  text_color=COLORS[Color.piano_roll_note_name_focus if focus else Color.piano_roll_note_name_no_focus],
                                  position=(note_name_x, note_y),
                                  background_color=COLORS[Color.panel_background],
                                  pivot=self._pivot,
                                  anchor=self._anchor,
                                  parent_rect=self._parent_rect),
                             Line(position=(x0, note_y),
                                  length=w,
                                  color=COLORS[Color.piano_roll_note_line_focus if focus else Color.piano_roll_note_line_no_focus],
                                  vertical=False,
                                  pivot=self._pivot,
                                  anchor=self._anchor,
                                  parent_rect=self._parent_rect)])
            # Blit each note.
            for i, note in enumerate(self._track.notes):
                # Ignore notes that are out of range.
                if note.note != note_value or note.note < self._note_0 or note.note > note_1 or note.start + note.duration <= self._time_0 or note.start > time_1:
                    continue
                selected = i == self._selected_note
                # Set the color of the note.
                if selected:
                    note_color = COLORS[Color.note_panel_selected_focus if focus else Color.note_panel_selected_no_focus]
                else:
                    note_color = COLORS[Color.note_panel_focus if focus else Color.note_panel_no_focus]
                # Blit the note.
                commands.append(PianoRollNote(t0=self._time_0 - note.start,
                                              duration=note.duration,
                                              color=note_color,
                                              arrows=(note.start < self._time_0, note.start + note.duration > time_1),
                                              position=(x0 + int(note.start - self._time_0), note_y),
                                              pivot=self._pivot,
                                              anchor=self._anchor,
                                              parent_rect=self._parent_rect))
            note_y += 1
        # Add arrows.
        max_time = 0
        for note in self._track.notes:
            note_t1 = note.start + note.duration
            if note_t1 > max_time:
                max_time = note_t1
        if focus:
            arrow_color = COLORS[Color.border_focus]
            mid_x = self._position[0] + self._size[0] // 2
            mid_y = self._position[1] + self._size[1] // 2
            if self._time_0 > 0:
                commands.append(Arrow(position=(self._position[0], mid_y),
                                      direction=CardinalDirection.west,
                                      color=arrow_color,
                                      pivot=self._pivot,
                                      anchor=self._anchor,
                                      parent_rect=self._parent_rect))
            if time_1 < max_time:
                commands.append(Arrow(position=(self._position[0] + self._size[0], mid_y),
                                      direction=CardinalDirection.east,
                                      color=arrow_color,
                                      pivot=self._pivot,
                                      anchor=self._anchor,
                                      parent_rect=self._parent_rect))
            if self._note_0 > 0:
                commands.append(Arrow(position=(mid_x, self._position[1]),
                                      direction=CardinalDirection.north,
                                      color=arrow_color,
                                      pivot=self._pivot,
                                      anchor=self._anchor,
                                      parent_rect=self._parent_rect))
            if note_1 < 127:
                commands.append(Arrow(position=(mid_x, self._position[1] + self._size[1]),
                                      direction=CardinalDirection.south,
                                      color=arrow_color,
                                      pivot=self._pivot,
                                      anchor=self._anchor,
                                      parent_rect=self._parent_rect))

        return commands

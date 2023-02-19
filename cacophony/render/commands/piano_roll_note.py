from math import ceil
from typing import Tuple
from pygame import Surface, Rect
from pygame.draw import polygon, rect
from cacophony.render.globals import CELL_SIZE, CLEAR_COLOR
from cacophony.render.commands.abc_rectangle import AbcRectangle


class PianoRollNote(AbcRectangle):
    """
    Draw a piano roll note. This is a rectangle that spans a portion of some cell(s).

    A piano roll note
     may have arrows on either side. The width of the re
    """
    _ARROW_HEIGHT: int = CELL_SIZE[0] // 3
    _HALF_HEIGHT: int = CELL_SIZE[1] // 2

    def __init__(self, t0: float, duration: float, color: Tuple[int, int, int], arrows: Tuple[bool, bool],
                 position: Tuple[int, int], pivot: Tuple[float, float] = None, anchor: Tuple[float, float] = None,
                 parent_rect: Rect = None):
        """
        :param t0: The start time in beats. 1 beat = 1 cell.
        :param duration: The duration in beats. 1 beat = 1 cell.
        :param color: The RGB (0-255) color.
        :param arrows: A 2-element bool tuple indicating whether there is an arrow on the left and/or right.
        :param position: The top-left position in grid coordinates.
        :param pivot: The pivot as a (x, y) 0-1 tuple. If the pivot is `(0, 0)`, then it is located at the top-left of the surface we're about to render. `(0, 1)` would be bottom-left, etc.
        :param anchor: The anchor as a (x, y) 0-1 tuple. If the anchor is `(0, 0)`, there is no offset. If the anchor is `(1, 0)`, then the position would be offset by the pivot offset plus the width of the screen.
        :param parent_rect: The parent rect that pivot and anchor are relative to. If None, this defaults to the display surface rect.
        """

        w = int(ceil(duration))
        x = position[0]
        if arrows[0]:
            x -= 1
            w += 1
        if arrows[1]:
            w += 1
        super().__init__(size=(w, 1),
                         color=CLEAR_COLOR,
                         position=(x, position[1]),
                         pivot=pivot,
                         anchor=anchor,
                         parent_rect=parent_rect)
        self._t0: float = t0
        self._duration: float = duration
        self._note_color: Tuple[int, int, int] = color
        self._arrows: Tuple[bool, bool] = arrows

    def _get_surface(self) -> Surface:
        # Draw the background surface.
        surface = Surface(self._size)
        surface.fill(self._color)
        surface.set_colorkey(self._color)
        # Get the start and stop of the note.
        t01 = self._t0 % 1
        note_x0: int = int(t01 * CELL_SIZE[0])
        if self._arrows[0]:
            note_x0 += CELL_SIZE[0]
        note_w: int = int((self._duration - self._t0 % 1) * CELL_SIZE[0])
        # Draw the arrows.
        if self._arrows[0]:
            arrow_x_0 = note_x0
            arrow_x_1 = arrow_x_0 - PianoRollNote._ARROW_HEIGHT
            polygon(surface, self._note_color, [(arrow_x_0, 0),
                                                (arrow_x_1, PianoRollNote._HALF_HEIGHT),
                                                (arrow_x_0, CELL_SIZE[1])])
        if self._arrows[1]:
            arrow_x_0 = note_x0 + note_w - 1
            arrow_x_1 = arrow_x_0 + PianoRollNote._ARROW_HEIGHT
            polygon(surface, self._note_color, [(arrow_x_0, 0),
                                                (arrow_x_1, PianoRollNote._HALF_HEIGHT),
                                                (arrow_x_0, CELL_SIZE[1])])
        # Draw the note.
        rect(surface, self._note_color, Rect(note_x0, 0, note_w, CELL_SIZE[1]))
        return surface

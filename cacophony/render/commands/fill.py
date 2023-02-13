from typing import Tuple
from cacophony.render.commands.rectangle import Rectangle
from cacophony.render.globals import WINDOW_GRID_WIDTH, WINDOW_GRID_HEIGHT


class Fill(Rectangle):
    """
    Fill the screen with a color.
    """

    def __init__(self, color: Tuple[int, int, int]):
        """
        :param color: The RGB (0-255) color.
        """

        super().__init__(position=(0, 0), size=(WINDOW_GRID_WIDTH, WINDOW_GRID_HEIGHT), color=color)

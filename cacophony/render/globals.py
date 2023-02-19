from json import loads
from typing import Tuple, Dict, Union
from configparser import ConfigParser, SectionProxy
from pathlib import Path
import pygame.font
from pygame import Surface
import numpy as np
from cacophony.paths import USER_DIRECTORY, DATA_DIRECTORY
from cacophony.cardinal_direction import CardinalDirection
from cacophony.render.color import Color
from cacophony.render.input_key import InputKey

__parser = ConfigParser()
__local_config_file = USER_DIRECTORY.joinpath("config.init")
# Read a user-defined config file.
if __local_config_file.exists():
    __parser.read(str(__local_config_file))
# Read the default config file.
else:
    __parser.read(str(DATA_DIRECTORY.joinpath("config.ini")))
__ui_section: SectionProxy = __parser["RENDER"]
# The width of the window in number of grid cells.
WINDOW_GRID_WIDTH: int = int(__ui_section["window_width"])
# The height of the window in number of grid cells.
WINDOW_GRID_HEIGHT: int = int(__ui_section["window_height"])
if __ui_section["font_path"] == "<default>":
    __font_path: Path = DATA_DIRECTORY.joinpath("UbuntuMono-Regular.ttf")
else:
    __font_path = Path(__ui_section["font_path"]).absolute()
pygame.font.init()
# The pygame font.
FONT: pygame.font.Font = pygame.font.Font(str(__font_path), int(__ui_section["font_size"]))
CELL_SIZE: Tuple[int, int] = FONT.size("a")
# The width of the window in pixels.
WINDOW_PIXEL_WIDTH: int = CELL_SIZE[0] * WINDOW_GRID_WIDTH
# The height of the window in pixels.
WINDOW_PIXEL_HEIGHT: int = CELL_SIZE[1] * WINDOW_GRID_HEIGHT
# The width of rectangle borders in pixels.
BORDER_PIXEL_WIDTH: int = int(__ui_section["border_width"])
# The clear (alpha) color.
CLEAR_COLOR: Tuple[int, int, int] = tuple(loads(__ui_section["clear_color"]))
# True if the clear color is (0, 0, 0).
CLEAR_COLOR_IS_BLACK: bool = CLEAR_COLOR == (0, 0, 0)

__color_aliases_section: SectionProxy = __parser["COLOR_ALIASES"]
__color_aliases: Dict[str, Tuple[int, int, int]] = {__k: tuple(loads(__color_aliases_section[__k])) for __k in __color_aliases_section}
__colors_section: SectionProxy = __parser["COLORS"]
# A dictionary of UI colors.
COLORS: Dict[Color, Tuple[int, int, int]] = {Color[__k]: __color_aliases[__colors_section[__k]] if __colors_section[__k] in __color_aliases else tuple(loads(__colors_section[__k])) for __k in __colors_section}
# The color used for 1bit numpy image arrays.
NUMPY_COLOR: Tuple[int, int, int] = (255, 255, 255)
# Arrow numpy arrays.
ARROWS: Dict[CardinalDirection, np.ndarray] = dict()
# Draw arrows.
arrow_surface: Surface = Surface(CELL_SIZE)
arrow_surface.fill(CLEAR_COLOR)
pygame.draw.polygon(arrow_surface, NUMPY_COLOR, [(0, 0), (CELL_SIZE[0], CELL_SIZE[1] // 2), (0, CELL_SIZE[1])])
arrow: np.ndarray = pygame.surfarray.array3d(arrow_surface)
# Store arrows.
ARROWS[CardinalDirection.east] = arrow
ARROWS[CardinalDirection.south] = np.rot90(ARROWS[CardinalDirection.east])
ARROWS[CardinalDirection.west] = np.rot90(ARROWS[CardinalDirection.south])
ARROWS[CardinalDirection.north] = np.rot90(ARROWS[CardinalDirection.west])
# Input.
__input_section: SectionProxy = __parser["KEYBOARD_INPUT"]
INPUTS: Dict[Union[str, Tuple[int, int, int]], InputKey] = dict()
for __i in InputKey:
    INPUTS[__input_section[__i.name]] = __i
# A stippled rectangle.
STIPPLE_ARRAY: np.ndarray = np.zeros(CELL_SIZE, dtype=np.uint8)
STIPPLE_ARRAY[::2, 1::2] = 255
STIPPLE_ARRAY[1::2, 0::2] = 255
STIPPLE_ARRAY = np.stack((STIPPLE_ARRAY,) * 3, axis=-1)

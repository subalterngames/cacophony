from json import loads
from typing import Tuple
from configparser import ConfigParser, SectionProxy
from pathlib import Path
import pygame.font
from cacophony.paths import USER_DIRECTORY, DATA_DIRECTORY

__parser = ConfigParser()
__local_config_file = USER_DIRECTORY.joinpath("config.init")
# Read a user-defined config file.
if __local_config_file.exists():
    __parser.read(str(__local_config_file))
# Read the default config file.
else:
    __parser.read(str(DATA_DIRECTORY.joinpath("config.ini")))
__ui_section: SectionProxy = __parser["UI"]
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

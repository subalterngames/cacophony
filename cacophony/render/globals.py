from json import loads
from typing import Tuple, Dict, Union, List
from configparser import SectionProxy
from pathlib import Path
import pygame.font
from pygame import Surface
import numpy as np
from cacophony.paths import DATA_DIRECTORY
from cacophony.cardinal_direction import CardinalDirection
from cacophony.render.color import Color
from cacophony.render.input_key import InputKey
from cacophony.util import get_config

__parser = get_config()
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
INPUTS: Dict[Union[str, tuple], InputKey] = dict()
for __i in InputKey:
    __input_key = __input_section[__i.name]
    # This is a list.
    if __input_key[0] == "[" and __input_key[-1] == "]":
        __input_key = __input_key[1:-1].split(",")
        __input_key = tuple([__ik.strip() for __ik in __input_key])
    INPUTS[__input_key] = __i
INPUT_KEYS: Dict[InputKey, List[Union[str, tuple]]] = dict()
for __k in INPUTS:
    __v = INPUTS[__k]
    if __v not in INPUT_KEYS:
        INPUT_KEYS[__v] = list()
    INPUT_KEYS[__v].append(__k)
# Layout.
__layout_section: SectionProxy = __parser["LAYOUT"]
__main_menu_rect: Tuple[int, int, int, int] = (0,
                                               0,
                                               WINDOW_GRID_WIDTH,
                                               int(__layout_section["main_menu_height"]))
__tracks_list_width_fr = __layout_section["tracks_list_width"].split("/")
__tracks_list_width: int = int(WINDOW_GRID_WIDTH * float(__tracks_list_width_fr[0]) / float(__tracks_list_width_fr[1]))
__tracks_list_rect: Tuple[int, int, int, int] = (0,
                                                 __main_menu_rect[3],
                                                 __tracks_list_width,
                                                 WINDOW_GRID_HEIGHT - __main_menu_rect[3])
__piano_roll_width_fr = __layout_section["piano_roll_width"].split("/")
__piano_roll_width = int(WINDOW_GRID_WIDTH * float(__piano_roll_width_fr[0]) / float(__piano_roll_width_fr[1]))
__piano_roll_rect: Tuple[int, int, int, int] = (__tracks_list_rect[2],
                                                __main_menu_rect[3],
                                                __piano_roll_width,
                                                __tracks_list_rect[3])
__right_panels_width: int = WINDOW_GRID_WIDTH - (__tracks_list_width + __piano_roll_width)
__synthesizer_rect: Tuple[int, int, int, int] = (WINDOW_GRID_WIDTH - __right_panels_width,
                                                 __main_menu_rect[3],
                                                 __right_panels_width,
                                                 __tracks_list_rect[3])
LAYOUTS: Dict[str, Tuple[int, int, int, int]] = {"MainMenu": __main_menu_rect,
                                                 "TracksList": __tracks_list_rect,
                                                 "PianoRoll": __piano_roll_rect,
                                                 "SynthesizerPanel": __synthesizer_rect}
LAYOUTS["NewTrack"] = LAYOUTS["TracksList"]
# Scrolling.
SCROLL_DT: float = float(__parser["SCROLL"]["dt"])
# UI audio.
__ui_audio: SectionProxy = __parser["UI_AUDIO"]
UI_AUDIO_GAIN: float = int(__ui_audio["gain"]) / 127
# MIDI
__midi_devices: SectionProxy = __parser["MIDI_DEVICES"]
MIDI_INPUT_DEVICE_ID: str = __midi_devices["input"]
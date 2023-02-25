from typing import List, Callable
import sys
import importlib
import inspect
from pathlib import Path
from pkg_resources import resource_filename
from pkgutil import iter_modules
from pygame.display import get_surface
from pygame import Surface, Rect
from cacophony.paths import USER_DIRECTORY
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.synthesizer.util import get_synthesizers
from cacophony.render.panel.scroll_panel import ScrollPanel


class NewTrack(ScrollPanel):
    pass
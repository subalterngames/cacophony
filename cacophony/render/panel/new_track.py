from typing import List, Callable, Optional
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
from cacophony.render.widget.widget import Widget
from cacophony.render.widget.label import Label
from cacophony.render.widget.divider import Divider
from cacophony.render.render_result import RenderResult
from cacophony.render.input_key import InputKey
from cacophony.render.globals import LAYOUTS


class NewTrack(ScrollPanel):
    def __init__(self):
        self._synthesizer_callables = get_synthesizers()
        self.synthesizer: Optional[Synthesizer] = None
        self.done: bool = False
        layout = LAYOUTS[self.__class__.__name__]
        panel_size = (layout[2], layout[3])
        w = panel_size[0]
        widgets: List[Widget] = [Divider(text="Waveformers", width=w),
                                 Divider(text="Synthesizers", width=w)]
        self._synthesizer_labels: List[Widget] = [Label(text=sc.__name__,
                                                        size=w)
                                                  for sc in self._synthesizer_callables]
        widgets.extend(self._synthesizer_labels)
        super().__init__(title=f"New Track",
                         position=(layout[0], layout[1]),
                         size=(layout[2], layout[3]),
                         widgets=widgets)
        # Scroll past the dividers.
        for i in range(2):
            self.selection_index = 2
            self._widget_index = 2

    def _do_result(self, result: RenderResult) -> bool:
        did = super()._do_result(result=result)
        if did:
            return did
        # Selected a synthesizer.
        if InputKey.select in result.inputs_pressed and isinstance(self._widgets[self.selection_index], Label):
            # Get the selected synthesizer label.
            if self._widgets[self.selection_index] in self._synthesizer_labels:
                index = self._synthesizer_labels.index(self._widgets[self.selection_index])
                self.synthesizer = self._synthesizer_callables[index]
                self.done = True
                return True
        # Cancel.
        elif InputKey.cancel in result.inputs_pressed:
            self.synthesizer = None
            self.done = True
            return True
        return False

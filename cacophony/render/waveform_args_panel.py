from typing import Tuple, List
from cacophony.waveform_generators.waveform_generator import WaveformGenerator
from cacophony.waveform_generators.callbacker import Callbacker
from cacophony.render.options_panel import OptionsPanel
from cacophony.render.globals import WINDOW_GRID_WIDTH, WINDOW_GRID_HEIGHT
from cacophony.render.ui_fields.ui_field import UiField
from cacophony.render.ui_fields.options import Options
from cacophony.render.ui_fields.string import String


class WaveformArgsPanel(OptionsPanel):
    def __init__(self, wfg: WaveformGenerator, panel_focus: bool):
        panel_size: Tuple[int, int] = (WINDOW_GRID_WIDTH // 5, WINDOW_GRID_HEIGHT)
        self._wfg: WaveformGenerator = wfg
        # Build lists of parameters.
        fields: List[UiField] = list()
        for k in self._wfg.__dict__:
            # Ignore private or class fields.
            if k[0] == "_" or k.startswith(self._wfg.__class__.__name__):
                continue
            v = self._wfg.__dict__[k]
            # Add options.
            if isinstance(v, list) or isinstance(v, dict):
                fields.append(Options(title=k,
                                      index=0,
                                      options=v))
            elif isinstance(v, Callbacker):
                fields.append(String(title=k,
                                     value=v,
                                     width=panel_size[0] - 2,
                                     callback=self.show_panel))
            else:
                raise Exception(k, v)
        super().__init__(position=(WINDOW_GRID_WIDTH - panel_size[0], 1),
                         size=panel_size,
                         vertical=True,
                         panel_focus=panel_focus,
                         fields=fields,
                         title=wfg.__class__.__name__)

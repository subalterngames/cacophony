from cacophony.render.options_panel import OptionsPanel
from cacophony.render.ui_fields.callbacker import Callbacker
from cacophony.render.globals import WINDOW_GRID_WIDTH, COLORS
from cacophony.render.color import Color

def do_nothing():
    pass


c = Color.parameter_key
r = OptionsPanel(position=(0, 0),
                 size=(WINDOW_GRID_WIDTH, 1),
                 panel_focus=True,
                 vertical=False,
                 fields=[Callbacker(title="New",
                                    font_color=c,
                                    callback=do_nothing),
                         Callbacker(title="Load",
                                    font_color=c,
                                    callback=do_nothing),
                         Callbacker(title="Save",
                                    font_color=c,
                                    callback=do_nothing),
                         Callbacker(title="Export",
                                    font_color=c,
                                    callback=do_nothing)])
r.do()


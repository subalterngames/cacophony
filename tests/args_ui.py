from cacophony.waveform_generators.soundfont import SoundFont
from cacophony.render.waveform_args_panel import WaveformArgsPanel

r = WaveformArgsPanel(SoundFont(), panel_focus=True)
r.do()

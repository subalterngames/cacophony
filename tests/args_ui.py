from cacophony.soundfont import SoundFont
from cacophony.render.waveform_generator_args import WaveformGeneratorArgs

r = WaveformGeneratorArgs(SoundFont(), focus=True)
r.do()

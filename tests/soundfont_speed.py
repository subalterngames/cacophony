from io import BytesIO
import pygame.mixer
from time import time, sleep
from sf2_loader.read_sf2.read_sf2 import sf2_loader
from cacophony.waveform_generators.globals import NOTES


t0 = time()
loader = sf2_loader("D:/SoundFonts/ms_basic.sf3")
print("load time:", time() - t0)
loader.change(bank=0, channel=0, preset=0)
times = []
segments = []
for note in NOTES:
    t0 = time()
    resp = loader.export_note(note,
                              duration=1,
                              decay=0,
                              volume=127,
                              channel=0,
                              start_time=0,
                              sample_width=2,
                              channels=2,
                              frame_rate=44100,
                              name=None,
                              format='wav',
                              effects=None,
                              bpm=60,
                              export_args={},
                              get_audio=True)
    times.append(time() - t0)
    segments.append(resp.raw_data)

print("soundfont.", "total:", sum(times), "average:", sum(times) / len(times))

pygame.mixer.init(allowedchanges=pygame.AUDIO_ALLOW_CHANNELS_CHANGE)
times.clear()
sounds = []
for s in segments:
    t0 = time()
    sound = pygame.mixer.Sound(s)
    times.append(time() - t0)
    sounds.append(sound)
print("pygame.", "total:", sum(times), "average:", sum(times) / len(times))


from musicpy import C, play, note, track, chord, piece
from musicpy.daw import *

new_song = daw(3, name='my first song')
new_song.load(0, "D:/SoundFonts/ms_basic.sf3")
new_song.channel_sound_modules[0].change(bank=0)
t0 = time()
guitar = (C('CM7', 3, 1/4, 1/8)^2 |
          C('G7sus', 2, 1/4, 1/8)^2 |
          C('A7sus', 2, 1/4, 1/8)^2 |
          C('Em7', 2, 1/4, 1/8)^2 |
          C('FM7', 2, 1/4, 1/8)^2 |
          C('CM7', 3, 1/4, 1/8)@1 |
          C('AbM7', 2, 1/4, 1/8)^2 |
          C('G7sus', 2, 1/4, 1/8)^2) * 2
print(time() - t0)
q = new_song.export(guitar, action="get")
print(q)
print(time() - t0)
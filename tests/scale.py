from io import BytesIO
import pygame.mixer
from musicpy.daw import daw
from procemon_rpg.music.track import Track
from sf2_loader.read_sf2.fluidsynth import raw_audio_string
from time import time, sleep
from pydub import AudioSegment

new_song = daw(1, name='my first song', bpm=120)
new_song.load(0, "D:/SoundFonts/ms_basic.sf3")
scale = Track.get_scale(key="C", major=True)
new_song.channel_sound_modules[0].change(bank=0, channel=0, preset=44)
audio = bytearray()
duration = 1
t0 = time()
for note in range(60, 70):
    new_song.channel_sound_modules[0].synth.noteon(0, note, 127)
    z = new_song.channel_sound_modules[0].synth.get_samples(len=int(44100 * duration))
    new_song.channel_sound_modules[0].synth.noteoff(0, note)
    audio.extend(raw_audio_string(z))
print(time() - t0)
s = pygame.mixer.Sound(bytes(audio))
s.play()
sleep(s.get_length())
exit()
start_time = 0
scale = Track.get_scale(key="C", major=True)
chords = []
t0 = time()
for ns in scale:
    n = note(name=ns, num=4, duration=0.25, volume=127)
    chords.append(chord(notes=[n], start_time=start_time, duration=0.25))
    # start_time += 0.25
q = chords[0]
for i in range(1, len(chords)):
    q = q | chords[i]
print(time() - t0)
print(q)
qq = new_song.export(q, action="get")
print(time() - t0)
print(qq)
new_song.play(q, wait=True)

from os.path import getsize
from typing import List
from time import sleep
import pygame.midi
from cacophony.music.music import Music
from cacophony.synthesizer.chiptune import Chiptune
from cacophony.synthesizer.chiptune_pcm import ChiptunePCM
from cacophony.synthesizer.soundfont import SoundFont
from cacophony.synthesizer.clatter import Clatter
from cacophony.music.track import Track
from cacophony.music.note import Note

pygame.mixer.init(allowedchanges=pygame.AUDIO_ALLOW_CHANNELS_CHANGE)
pygame.midi.init()
m = pygame.midi.Input(pygame.midi.get_default_input_id())
print(m)
print(pygame.midi.get_default_input_id())

bpm = 120
beat = 1
sf = SoundFont(channel=0)
sf.load("D:/SoundFonts/ms_basic.sf3")
sf.set_instrument(bank=0, preset=0)
synths = [Chiptune(ChiptunePCM.saw),
          sf,
          Clatter()]
synth_index = 0
knob = 0
music = Music(bpm=bpm, tracks=[Track(track_id=0, synthesizer=synths[synth_index])])
pygame.init()
pygame.display.set_mode((256, 256))
pygame.mixer.init(allowedchanges=pygame.AUDIO_ALLOW_CHANNELS_CHANGE)
pygame.midi.init()
m = pygame.midi.Input(pygame.midi.get_default_input_id())
done = False
t = 0
volume = 100
fixed_volume = False
note_ons: List[int] = list()
while not done:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            exit()
        elif event.type == pygame.KEYDOWN:
            k = pygame.key.name(event.key)
            if k == "space":
                done = True
    if m.poll():
        events = m.read(16)
        note_off = False
        for event in events:
            event_type = event[0][0]
            # Note on.
            if 144 <= event_type <= 159:
                note = Note(event[0][1], start=t, duration=beat, volume=volume if fixed_volume else event[0][2])
                a = synths[synth_index].audio(note=note, bpm=bpm)
                sound = pygame.mixer.Sound(a)
                sound.play()
                music.tracks[0].notes.append(note)
                note_ons.append(event[0][1])
        # Note off.
        for event in events:
            event_type = event[0][0]
            if 128 <= event_type <= 143:
                if event[0][1] in note_ons:
                    note_ons.remove(event[0][1])
                    note_off = True
        # Knobs.
        for event in events:
            if event[0][0] == 176:
                if event[0][1] == 16:
                    knob += 0.1
                    if knob < 1:
                        continue
                    knob = 0
                    if event[0][2] == 127:
                        synth_index -= 1
                        if synth_index < 0:
                            synth_index = len(synths) - 1
                    else:
                        synth_index += 1
                        if synth_index >= len(synths):
                            synth_index = 0
                    music.tracks[0].synthesizer = synths[synth_index]
                elif event[0][1] == 20:
                    print(event)
        # Advance time.
        if note_off and len(note_ons) == 0:
            t += beat
# Playback.
filename = "test.hdf5"
music.serialize(filename)
print(getsize(filename))
audio_segment = music.audio()
print(len(audio_segment.raw_data))
sound = pygame.mixer.Sound(audio_segment.raw_data)
sound.play()
sleep(sound.get_length())

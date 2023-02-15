from typing import List, Tuple
from time import sleep
import pygame.midi
from cacophony.music.music import Music
from cacophony.synthesizer.chiptune import Chiptune
from cacophony.synthesizer.chiptune_pcm import ChiptunePCM
from cacophony.music.track import Track
from cacophony.music.note import Note


bpm = 120
beat = 1
synth = Chiptune(ChiptunePCM.saw)
music = Music(bpm=bpm, tracks=[Track(synthesizer=synth)])
pygame.init()
pygame.display.set_mode((256, 256))
pygame.mixer.init(allowedchanges=pygame.AUDIO_ALLOW_CHANNELS_CHANGE)
pygame.midi.init()
m = pygame.midi.Input(pygame.midi.get_default_input_id())
done = False
t = 0
volume = 100
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
                note = Note(event[0][1], start=t, duration=beat, volume=volume)
                a = synth.audio(note=note, bpm=bpm)
                sound = pygame.mixer.Sound(a)
                sound.play()
                music.tracks[0].notes.append(note)
                note_ons.append(event[0][1])
        # Note off.
        for event in events:
            event_type = event[0][0]
            if 128 <= event_type <= 143:
                note_ons.remove(event[0][1])
                note_off = True
        # Advance time.
        if note_off and len(note_ons) == 0:
            t += beat
# Playback.
audio_segment = music.audio()
sound = pygame.mixer.Sound(audio_segment.raw_data)
sound.play()
sleep(sound.get_length())

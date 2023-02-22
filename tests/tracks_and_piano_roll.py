import pygame.mixer
from cacophony.music.music import Music
from cacophony.music.track import Track
from cacophony.music.note import Note
from cacophony.synthesizer.soundfont import SoundFont
from cacophony.synthesizer.chiptune import Chiptune
from cacophony.synthesizer.chiptune_pcm import ChiptunePCM
from cacophony.render.renderer import Renderer
from cacophony.render.panel.tracks_list import TracksList
from cacophony.render.panel.piano_roll import PianoRoll
from cacophony.render.panel.main_menu import MainMenu
from cacophony.render.panel.synthesizer_panel import SynthesizerPanel
from cacophony.render.input_key import InputKey


m = Music(bpm=120)
m.tracks.append(Track(track_id=0, synthesizer=Chiptune(ChiptunePCM.saw)))
synth_1 = SoundFont(channel_index=0)
synth_1.path = "D:/SoundFonts/ms_basic.sf3"
synth_1.bank = 0
synth_1.preset = 2
track_1 = Track(track_id=1, synthesizer=synth_1)
t = 0
for n in range(60, 80):
    track_1.notes.append(Note(note=n,
                              start=t,
                              duration=1,
                              volume=127))
    t += 1
m.tracks.append(track_1)
track_id = 2
for i in range(4):
    m.tracks.append(Track(track_id=track_id, synthesizer=SoundFont(channel_index=i)))
    track_id += 1
r = Renderer()
result = r.render([])
panels = [MainMenu(),
          TracksList(music=m),
          PianoRoll(music=m, track_index=0, selected_note=-1, note_0=60, time_0=0),
          SynthesizerPanel(music=m, track_index=0)]
focus = 0
track_index = 0
channel = pygame.mixer.find_channel()
while True:
    # Play the audio.
    if not channel.get_busy() and InputKey.play in result.inputs_pressed:
        audio_segment = m.tracks[1].audio_segment(bpm=m.bpm)
        sound = pygame.mixer.Sound(audio_segment.raw_data)
        channel.play(sound)
    # Cycle between panels.
    if InputKey.next_panel in result.inputs_pressed:
        panels[focus].initialized = False
        focus += 1
        if focus >= len(panels):
            focus = 0
        panels[focus].initialized = False
    # Render.
    commands = panels[0].render(result=result, focus=focus == 0)
    commands.extend(panels[1].render(result=result, focus=focus == 1))
    if panels[1].selection_index != track_index:
        track_index = panels[1].selection_index
        panels[2].track_index = track_index
        panels[2].initialized = False
    commands.extend(panels[2].render(result=result, focus=focus == 2))
    commands.extend(panels[3].render(result=result, focus=focus == 3))
    result = r.render(commands)

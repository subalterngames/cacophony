import pygame.mixer
from cacophony.music.music import Music
from cacophony.music.track import Track
from cacophony.music.note import Note
from cacophony.synthesizer.soundfont import SoundFont
from cacophony.render.renderer import Renderer
from cacophony.render.panel.scroll_panel import ScrollPanel
from cacophony.render.panel.piano_roll import PianoRoll
from cacophony.render.panel.main_menu import MainMenu
from cacophony.render.ui_element.label import Label
from cacophony.render.globals import WINDOW_GRID_WIDTH, WINDOW_GRID_HEIGHT
from cacophony.render.input_key import InputKey


m = Music(bpm=120)
m.tracks.append(Track(track_id=0, synthesizer=SoundFont(channel=0)))
synth_1 = SoundFont(channel=1)
synth_1.load("D:/SoundFonts/ms_basic.sf3")
synth_1.set_instrument(bank=0, preset=2)
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
    m.tracks.append(Track(track_id=track_id, synthesizer=SoundFont(channel=track_id)))
    track_id += 1
r = Renderer()
x = 0
y = 3
pivot = (0, 0)
anchor = (0, 0)
panel_size = (WINDOW_GRID_WIDTH // 7, WINDOW_GRID_HEIGHT - y)
tracks_list = ScrollPanel(elements=[Label(text=str(i) + " " + track.synthesizer.__class__.__name__,
                                    size=panel_size[0] - 2) for i, track in enumerate(m.tracks)],
                          position=(0, 3),
                          size=panel_size,
                          title="Tracks")
t0 = 0
n0 = 50
piano_roll_position = (panel_size[0], y)
piano_roll_size = (WINDOW_GRID_WIDTH - piano_roll_position[0], panel_size[1])
piano_roll = PianoRoll(track=m.tracks[0], selected_note=0, time_0=t0, note_0=n0, position=piano_roll_position, size=piano_roll_size)
commands = tracks_list.blit(False)
commands.extend(piano_roll.render(False))
main_menu = MainMenu()
commands.extend(main_menu.render(True))
panels = [main_menu, tracks_list, piano_roll]
focus = 0
selected_track = 0
result = r.render(commands)
channel = pygame.mixer.find_channel()
while True:
    # Play the audio.
    if not channel.get_busy() and InputKey.play in result.inputs_pressed:
        audio_segment = m.tracks[1].audio_segment(bpm=m.bpm)
        sound = pygame.mixer.Sound(audio_segment.raw_data)
        channel.play(sound)
    # Cycle between panels.
    if InputKey.next_panel in result.inputs_pressed:
        focus += 1
        if focus >= len(panels):
            focus = 0
        commands = []
        for i, panel in enumerate(panels):
            commands.extend(panel.blit(focus=i == focus))
        result = r.render(commands)
    # Scroll through tracks.
    elif focus == 1:
        if InputKey.up in result.inputs_pressed:
            st1 = panels[1].scroll(up=True)
        elif InputKey.down in result.inputs_pressed:
            st1 = panels[1].scroll(up=False)
        else:
            st1 = selected_track
        if st1 != selected_track:
            selected_track = st1
            panels[2] = PianoRoll(track=m.tracks[selected_track], selected_note=0, time_0=t0, note_0=n0,
                                  position=piano_roll_position, size=piano_roll_size)
            commands = panels[1].render(True)
            commands.extend(panels[2].render(False))
            result = r.render(commands)
        else:
            result = r.render([])
    # Scroll around the piano roll.
    elif focus == 2:
        refresh = False
        if n0 > 0 and InputKey.down in result.inputs_pressed:
            n0 -= 1
            refresh = True
        elif n0 < 127 and InputKey.up in result.inputs_pressed:
            n0 += 1
            refresh = True
        elif t0 > 0 and InputKey.left in result.inputs_pressed:
            t0 -= 1
            refresh = True
        elif t0 < 100 and InputKey.right in result.inputs_pressed:
            t0 += 1
            refresh = True
        if refresh:
            panels[2] = PianoRoll(track=m.tracks[selected_track], selected_note=0, time_0=t0, note_0=n0,
                                  position=piano_roll_position, size=piano_roll_size)
            commands = panels[1].blit(False)
            commands.extend(panels[2].blit(True))
            result = r.render(commands)
        else:
            result = r.render([])
    else:
        result = r.render([])


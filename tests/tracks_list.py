from cacophony.music.music import Music
from cacophony.music.track import Track
from cacophony.synthesizer.clatter import Clatter
from cacophony.synthesizer.soundfont import SoundFont
from cacophony.render.renderer import Renderer
from cacophony.render.panel.scroll_panel import ScrollPanel
from cacophony.render.ui_element.label import Label
from cacophony.render.globals import WINDOW_GRID_WIDTH, WINDOW_GRID_HEIGHT
from cacophony.render.input_key import InputKey


m = Music(bpm=120)
m.tracks.append(Track(track_id=0, synthesizer=Clatter()))
track_id = 1
channel = 0
for i in range(40):
    m.tracks.append(Track(track_id=track_id, synthesizer=SoundFont(channel=channel)))
    track_id += 1
    channel += 1
r = Renderer()
x = 0
y = 3
pivot = (0, 0)
anchor = (0, 0)
panel_size = (WINDOW_GRID_WIDTH // 7, WINDOW_GRID_HEIGHT - y)
panel = ScrollPanel(elements=[Label(text=str(i) + " " + track.synthesizer.__class__.__name__,
                                    size=panel_size[0] - 2) for i, track in enumerate(m.tracks)],
                    position=(0, 3),
                    size=panel_size,
                    title="Tracks")
result = r.render(panel.blit(True))
while True:
    if InputKey.up in result.inputs_pressed:
        panel.scroll(up=True)
    elif InputKey.down in result.inputs_pressed:
        panel.scroll(up=False)
    result = r.render(panel.blit(True))

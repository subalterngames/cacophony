from cacophony.render.renderer import Renderer
from cacophony.render.panel.piano_roll import PianoRoll
from cacophony.music.track import Track
from cacophony.music.note import Note
from cacophony.synthesizer.chiptune_pcm import ChiptunePCM
from cacophony.synthesizer.chiptune import Chiptune
from cacophony.render.globals import WINDOW_GRID_WIDTH, WINDOW_GRID_HEIGHT


track = Track(synthesizer=Chiptune(pcm=ChiptunePCM.saw), track_id=0)
t = 0
for n in range(60, 80):
    track.notes.append(Note(note=n,
                            start=t,
                            duration=1,
                            volume=127))
    t += 1
r = Renderer()
position = (WINDOW_GRID_WIDTH // 7, 3)
size = (WINDOW_GRID_WIDTH - position[0], WINDOW_GRID_HEIGHT - position[1])
piano_roll = PianoRoll(track=track, selected_note=2, time_0=2, note_0=50, position=position, size=size)
r.render(piano_roll.blit(focus=True))
r.do()

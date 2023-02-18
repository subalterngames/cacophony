from cacophony.music.music import Music
from cacophony.music.track import Track
from cacophony.synthesizer.clatter import Clatter
from cacophony.synthesizer.soundfont import SoundFont
from cacophony.render.renderer import Renderer
from cacophony.render.macros.tracks_list import tracks_list


m = Music(bpm=120)
m.tracks.append(Track(track_id=0, synthesizer=Clatter()))
m.tracks.append(Track(track_id=1, synthesizer=SoundFont(channel=0)))
m.tracks.append(Track(track_id=2, synthesizer=SoundFont(channel=1)))
r = Renderer()
r.render(tracks_list(tracks=m.tracks, selected_track_index=1, focus=True))
r.do()
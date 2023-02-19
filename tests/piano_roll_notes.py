from cacophony.render.renderer import Renderer
from cacophony.render.commands.piano_roll_note import PianoRollNote
from cacophony.render.color import Color
from cacophony.render.globals import COLORS


r = Renderer()
r.render([PianoRollNote(position=(5, 16),
                        t0=1.25,
                        duration=2.75,
                        color=COLORS[Color.note_panel_focus],
                        arrows=(False, False)),
          PianoRollNote(position=(5, 17),
                        t0=1,
                        duration=3,
                        color=COLORS[Color.note_panel_focus],
                        arrows=(False, False)),
          PianoRollNote(position=(5, 18),
                        t0=1,
                        duration=3,
                        color=COLORS[Color.note_panel_selected_focus],
                        arrows=(True, True)),
          PianoRollNote(position=(5, 19),
                        t0=1,
                        duration=2.75,
                        color=COLORS[Color.note_panel_selected_no_focus],
                        arrows=(False, True))
          ])
r.do()

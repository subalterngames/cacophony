from typing import Tuple, Dict
from overrides import final
from pygame.mixer import Sound
from cacophony.render.globals import LAYOUTS
from cacophony.render.panel.scroll_panel import ScrollPanel
from cacophony.render.render_result import RenderResult
from cacophony.render.widget.widget import Widget
from cacophony.render.widget.boolean import Boolean
from cacophony.render.widget.options import Options
from cacophony.music.music import Music
from cacophony.music.note import Note
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.callbacker.callbacker import Callbacker
from cacophony.callbacker.indexed_list import IndexedList
from cacophony.callbacker.value import Value
from cacophony.util import note_on


class SynthesizerPanel(ScrollPanel):
    """
    A panel with synthesizer parameters. This is an abstract class.
    """

    def __init__(self, music: Music, track_index: int):
        """
        :param music: The music.
        :param track_index: The track index.
        """

        self.music: Music = music
        self.track_index: int = track_index
        layout: Tuple[int, int, int, int] = LAYOUTS["SynthesizerPanel"]
        # Generate elements from the Callbacker parameters.
        synthesizer: Synthesizer = self.music.tracks[self.track_index].synthesizer
        self._synthesizer_widgets: Dict[Widget, str] = dict()
        for k in synthesizer.__dict__:
            # Ignore hidden parameters.
            if k[0] == "_":
                continue
            v = synthesizer.__dict__[k]
            if isinstance(v, IndexedList):
                widget = Options(title=k, options=v.get_strs(), index=v.index)
            elif isinstance(v, Value):
                if isinstance(v.value, bool):
                    widget = Boolean(title=k, value=v.value)
                else:
                    raise Exception(f"Unsupported value widget: {v}")
            else:
                raise Exception(f"Unsupported parameter: {k}, {v}")
            self._synthesizer_widgets[widget] = k
        position = (layout[0], layout[1])
        size = (layout[2], layout[3])
        super().__init__(title=self._get_panel_title(), position=position, size=size,
                         widgets=list(self._synthesizer_widgets.keys()))

    @final
    def _do_result(self, result: RenderResult) -> bool:
        did = super()._do_result(result=result)
        # Something happened. Update the synthesizer parameters.
        if did:
            focused_widget = self._widgets[self.selection_index]
            # Get the class attribute.
            attr = getattr(self.music.tracks[self.track_index].synthesizer, self._synthesizer_widgets[focused_widget])
            # Set the index.
            if isinstance(focused_widget, Options):
                attr.index = focused_widget.index
            elif isinstance(focused_widget, Boolean):
                attr.value = focused_widget.value
            else:
                raise Exception(f"Unsupported: {attr}, {focused_widget}")
        # Play the notes.
        beat = self.music.tracks[self.track_index].synthesizer.beat.get().value
        for i in range(len(result.midi)):
            # Play the note.
            if note_on(midi_event=result.midi[i]):
                note = Note(result.midi[i][1], start=0, duration=beat, volume=result.midi[i][2])
                a = self.music.tracks[self.track_index].synthesizer.audio(note=note, bpm=self.music.bpm)
                sound = Sound(a)
                sound.play()
        return did

    def get_widget_help(self) -> str:
        focused_widget = self._widgets[self.selection_index]
        attr: Callbacker = getattr(self.music.tracks[self.track_index].synthesizer, self._synthesizer_widgets[focused_widget])
        return f"{super().get_widget_help()} {attr.tts}"

    def _get_panel_title(self) -> str:
        return f"{self.track_index} {self.music.tracks[self.track_index].synthesizer.__class__.__name__}"

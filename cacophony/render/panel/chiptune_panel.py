from cacophony.render.panel.synthesizer_panel import SynthesizerPanel
from cacophony.render.input_key import InputKey
from cacophony.render.render_result import RenderResult
from cacophony.render.ui_element.options import Options
from cacophony.music.music import Music
from cacophony.synthesizer.chiptune_pcm import ChiptunePCM
from cacophony.synthesizer.chiptune import Chiptune
from cacophony.util import tooltip


class ChiptunePanel(SynthesizerPanel[Chiptune]):
    """
    A panel for a chiptune synthesizer.
    """

    def __init__(self, music: Music, track_index: int):
        super().__init__(music=music, track_index=track_index)
        self.pcm: Options = Options(title="PCM", options=[p.name for p in ChiptunePCM], index=0)
        self.widgets.append(self.pcm)

    def _do_synthesizer_result(self, result: RenderResult) -> bool:
        if self.focus_index == 3:
            if InputKey.left in result.inputs_pressed:
                self.pcm.cycle(False)
                self.undo_stack.append((self.pcm.cycle, {"increment": True}))
                return True
            if InputKey.right in result.inputs_pressed:
                self.pcm.cycle(True)
                self.undo_stack.append((self.pcm.cycle, {"increment": False}))
                return True
        return False

    def get_widget_help(self) -> str:
        if self.focus_index == 3:
            return tooltip(keys=[InputKey.left, InputKey.right], predicate="set chiptune PCM type.", boop="and")
        else:
            return super().get_widget_help()

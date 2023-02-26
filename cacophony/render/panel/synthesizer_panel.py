from typing import Tuple, Dict
from overrides import final
from cacophony.render.globals import LAYOUTS
from cacophony.render.panel.scroll_panel import ScrollPanel
from cacophony.render.panel.panel_type import PanelType
from cacophony.render.widget.widget import Widget
from cacophony.render.widget.boolean import Boolean
from cacophony.render.widget.options import Options
from cacophony.render.widget.file_prompt import FilePrompt
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.callbacker.callbacker import Callbacker
from cacophony.callbacker.indexed_list import IndexedList
from cacophony.callbacker.file_path import FilePath
from cacophony.callbacker.value import Value
from cacophony.state import State


class SynthesizerPanel(ScrollPanel):
    """
    A panel with synthesizer parameters. This is an abstract class.
    """

    def __init__(self):
        """
        (no parameters)
        """

        self._synthesizer_widgets: Dict[Widget, str] = dict()
        layout: Tuple[int, int, int, int] = LAYOUTS["SynthesizerPanel"]
        position = (layout[0], layout[1])
        size = (layout[2], layout[3])
        super().__init__(title="Synthesizer", position=position, size=size)

    def get_panel_type(self) -> PanelType:
        return PanelType.synthesizer_panel

    @final
    def _do_result(self, state: State) -> bool:
        did = super()._do_result(state=state)
        if len(self._widgets) == 0:
            return False
        # Something happened. Update the synthesizer parameters.
        if did:
            focused_widget = self._widgets[self._focused_widget_index]
            # Update the other widgets.
            for k in self._synthesizer_widgets:
                # Ignore hidden parameters.
                if k == focused_widget:
                    continue
                v = state.music.tracks[state.track_index].synthesizer.__dict__[self._synthesizer_widgets[k]]
                if isinstance(v, IndexedList):
                    k.options = v.get_strs()
                    k.index = v.index
                elif isinstance(v, Value):
                    k.value = v.value
                else:
                    raise Exception(f"Unsupported parameter: {k}, {v}, {v.__class__.__name__}")
            return True
        return False

    def get_widget_help(self, state: State) -> str:
        if len(state.music.tracks) > 0:
            focused_widget = self._widgets[self._focused_widget_index]
            attr: Callbacker = getattr(state.music.tracks[state.track_index].synthesizer, self._synthesizer_widgets[focused_widget])
            return f"{super().get_widget_help(state=state)} {attr.tts}"
        else:
            return ""

    def _get_panel_title(self, state: State) -> str:
        if len(state.music.tracks) > 0:
            return f"{state.track_index} {state.music.tracks[state.track_index].synthesizer.__class__.__name__}"
        else:
            return "Synthesizer"

    def _set_widgets(self, state: State) -> None:
        self._synthesizer_widgets.clear()
        # Generate elements from the Callbacker parameters.
        if len(state.music.tracks) > 0:
            synthesizer: Synthesizer = state.music.tracks[state.track_index].synthesizer
            for k in synthesizer.__dict__:
                # Ignore hidden parameters.
                if k[0] == "_":
                    continue
                v = synthesizer.__dict__[k]
                if isinstance(v, IndexedList):
                    widget = Options(title=k, options=v.get_strs(), index=v.index, width=self._size[0] - 2)
                    widget.set_callback(callback=SynthesizerPanel._set_indexed_list, kwargs={"indexed_list": v,
                                                                                             "options": widget})
                elif isinstance(v, FilePath):
                    widget = FilePrompt(path=v.value, width=self._size[0] - 2, suffixes=v.suffixes)
                    widget.set_callback(callback=SynthesizerPanel._set_path,
                                        kwargs={"file_path": v,
                                                "file_prompt": widget})
                elif isinstance(v, Value):
                    if isinstance(v.value, bool):
                        widget = Boolean(title=k, value=v.value)
                        widget.set_callback(callback=SynthesizerPanel._set_boolean, kwargs={"value": v,
                                                                                            "widget": widget})
                    else:
                        raise Exception(f"Unsupported value widget: {v}")
                else:
                    raise Exception(f"Unsupported parameter: {k}, {v}, {v.__class__.__name__}")
                self._synthesizer_widgets[widget] = k
        # Repopulate the pages.
        self._widgets = list(self._synthesizer_widgets.keys())
        self._populate_pages()

    @staticmethod
    def _set_indexed_list(indexed_list: IndexedList, options: Options) -> None:
        """
        Set the index of an indexed list from an options widget.

        :param indexed_list: The indexed list.
        :param options: The options widget.
        """

        indexed_list.index = options.index

    @staticmethod
    def _set_boolean(value: Value[bool], widget: Boolean) -> None:
        """
        Set a boolean value.

        :param value: The boolean value.
        :param widget: The boolean widget.
        """

        value.value = widget.value

    @staticmethod
    def _set_path(file_path: FilePath, file_prompt: FilePrompt) -> None:
        """
        Set a file path.

        :param file_path: The file path.
        :param file_prompt: The file prompt widget.
        """

        file_path.value = file_prompt.path

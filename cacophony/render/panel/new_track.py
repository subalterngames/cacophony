from typing import List
from cacophony.synthesizer.util import get_synthesizers
from cacophony.render.panel.scroll_panel import ScrollPanel
from cacophony.render.panel.panel_type import PanelType
from cacophony.render.widget.widget import Widget
from cacophony.render.widget.label import Label
from cacophony.render.widget.divider import Divider
from cacophony.render.input_key import InputKey
from cacophony.render.globals import LAYOUTS
from cacophony.music.track import Track
from cacophony.state import State


class NewTrack(ScrollPanel):
    def __init__(self):
        """
        (no parameters)
        """

        self._synthesizer_callables = get_synthesizers()
        layout = LAYOUTS[self.__class__.__name__]
        self._synthesizer_labels: List[Widget] = list()
        super().__init__(title=f"New Track",
                         position=(layout[0], layout[1]),
                         size=(layout[2], layout[3]))

    def get_panel_type(self) -> PanelType:
        return PanelType.new_track

    def _do_result(self, state: State, did_widget: bool) -> bool:
        did = super()._do_result(state=state, did_widget=did_widget)
        # Selected a synthesizer.
        if InputKey.select in state.result.inputs_pressed and isinstance(self._widgets[self._focused_widget_index], Label):
            # Get the selected synthesizer label.
            if self._widgets[self._focused_widget_index] in self._synthesizer_labels:
                index = self._synthesizer_labels.index(self._widgets[self._focused_widget_index])
                track_id = max([track.track_id for track in state.music.tracks]) + 1 if len(state.music.tracks) > 0 else 0
                # Add the track.
                track: Track = Track(track_id=track_id,
                                     synthesizer=self._synthesizer_callables[index]())
                state.add_track(track)
                self._end(state=state)
                return True
        # Cancel.
        elif InputKey.cancel in state.result.inputs_pressed:
            self._end(state=state)
            return True
        return did

    def _get_panel_title(self, state: State) -> str:
        return "New Track"

    def _set_widgets(self, state: State) -> None:
        w = self._size[0] - 2
        self._widgets.extend([Divider(text="Waveformers", width=w),
                              Divider(text="Synthesizers", width=w)])
        synthesizers = [Label(text=sc.__name__, size=w)
                        for sc in self._synthesizer_callables]
        self._widgets.extend(synthesizers)
        self._synthesizer_labels.clear()
        self._synthesizer_labels.extend(synthesizers)

    def _populate_pages(self) -> None:
        super()._populate_pages()
        self._widget_page_index = 2
        self._focused_widget_index = 2

    @staticmethod
    def _end(state: State) -> None:
        """
        Hide this panel and show the tracks list.

        :param state: The `State` of the program.
        """

        # Mark this panel as inactive and reactivate the tracks list.
        state.active_panels.remove(PanelType.new_track)
        # Reactivate the tracks list.
        state.active_panels.extend([PanelType.main_menu, PanelType.tracks_list, PanelType.piano_roll, PanelType.synthesizer_panel])
        # Render modified panels.
        state.dirty_panels.extend(state.active_panels[:])
        # Set the focus to the tracks list.
        state.focused_panel = PanelType.tracks_list

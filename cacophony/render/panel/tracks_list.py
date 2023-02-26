from cacophony.render.globals import LAYOUTS
from cacophony.render.panel.scroll_panel import ScrollPanel
from cacophony.render.panel.panel_type import PanelType
from cacophony.render.widget.label import Label
from cacophony.render.input_key import InputKey
from cacophony.util import tooltip
from cacophony.state import State


class TracksList(ScrollPanel):
    """
    A scrollable list of tracks.
    """

    def __init__(self):
        """
        (no parameters)
        """

        layout = LAYOUTS[self.__class__.__name__]
        super().__init__(title=f"Tracks", position=(layout[0], layout[1]), size=(layout[2], layout[3]))

    def get_panel_type(self) -> PanelType:
        return PanelType.tracks_list

    def get_widget_help(self, state: State) -> str:
        track = state.music.tracks[self._focused_widget_index]
        return f"Track {track.track_id}. {track.synthesizer.get_help_text()}"

    def get_panel_help(self, state: State) -> str:
        return super().get_panel_help(state=state) + " " + \
               tooltip(keys=[InputKey.add], predicate="add a new track") + " " + \
               tooltip(keys=[InputKey.subtract], predicate="delete the selected track")

    def _get_panel_title(self, state: State) -> str:
        return "List of tracks."

    def _do_result(self, state: State) -> bool:
        did = super()._do_result(state=state)
        # If the user scrolls through the tracks list, change the piano roll and the synthesizer panel.
        if did:
            state.track_index = self._focused_widget_index
            state.dirty_panels.extend([PanelType.piano_roll, PanelType.synthesizer_panel])
            return True
        # Show the new track panel and hide this panel.
        elif InputKey.add in state.result.inputs_pressed:
            self.active = False
            state.active_panels.clear()
            state.active_panels.append(PanelType.new_track)
            state.dirty_panels.append(PanelType.new_track)
            state.focused_panel = PanelType.new_track
            return True
        return False

    def _set_widgets(self, state: State) -> None:
        self._widgets.extend([Label(text=str(i) + " " + track.synthesizer.__class__.__name__,
                                    size=self._size[0] - 2) for i, track in enumerate(state.music.tracks)])

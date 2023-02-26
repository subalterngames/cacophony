from typing import List, Tuple, Callable, Optional
from cacophony.render.panel.panel_type import PanelType
from cacophony.render.render_result import RenderResult
from cacophony.music.music import Music
from cacophony.music.track import Track
from cacophony.piano_roll_state import PianoRollState
from cacophony.open_file_state import OpenFileState


class State:
    """
    The current program state.
    """

    def __init__(self, music: Music, track_index: int, focused_panel: PanelType,
                 piano_roll_state: PianoRollState, open_file_state: OpenFileState):
        """
        :param music: The music.
        :param track_index: The index of the track in `music` that is selected.
        :param focused_panel: The panel that currently has focus.
        :param piano_roll_state: The [`PianoRollState`](piano_roll_state.md).
        :param open_file_state: The [`OpenFileState`](open_file_state.md)
        """

        # The music.
        self.music: Music = music
        # The index of the track in `music` that is selected.
        self.track_index: int = track_index
        # The panel that currently has focus.
        self.focused_panel: PanelType = focused_panel
        # The most recent [`RenderResult`](render_result.md).
        self.result: Optional[RenderResult] = None
        # These panels need to be rerendered.
        self.dirty_panels: List[PanelType] = list()
        self.active_panels: List[PanelType] = [PanelType.main_menu, PanelType.tracks_list, PanelType.piano_roll,
                                               PanelType.synthesizer_panel]
        self.piano_roll_state: PianoRollState = piano_roll_state
        self.open_file_state: OpenFileState = open_file_state
        # A stack of undo operations. Each element is a tuple: Callbable, kwargs.
        self.undo_stack: List[Tuple[Callable, dict]] = list()
        self._removed_tracks: List[Track] = list()

    def set_track_index(self, track_index: int) -> None:
        """
        Set the track index.

        :param track_index: The new track index.
        """

        self.track_index = track_index
        self.dirty_panels.extend([PanelType.tracks_list, PanelType.synthesizer_panel, PanelType.piano_roll])

    def add_track(self, track: Track):
        """
        Add a new track.

        :param track: A new [`Track`](track.md).

        :return:
        """

        self.music.tracks.append(track)
        track_index: int = len(self.music.tracks) - 1
        self.undo_stack.append((self.remove_track, {"track_index": track_index}))

    def remove_track(self, track_index: int) -> None:
        """
        Remove a track.

        :param track_index: The index of the track we're removing.
        """

        self._removed_tracks.append(self.music.tracks.pop(track_index))
        self.track_index = track_index - 1
        self.undo_stack.append((self._add_removed_track, {}))

    def _add_removed_track(self) -> None:
        """
        Using the undo stack, add a removed track.
        """

        self.add_track(self._removed_tracks.pop())


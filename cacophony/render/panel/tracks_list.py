from cacophony.render.globals import LAYOUTS
from cacophony.render.panel.scroll_panel import ScrollPanel
from cacophony.render.panel.panel_type import PanelType
from cacophony.render.widget.label import Label
from cacophony.render.render_result import RenderResult
from cacophony.music.music import Music


class TracksList(ScrollPanel):
    """
    A scrollable list of tracks.
    """

    def __init__(self, music: Music):
        """
        :param music: The music.
        """

        layout = LAYOUTS[self.__class__.__name__]
        panel_size = (layout[2], layout[3])
        self._music: Music = music
        super().__init__(title=f"Tracks",
                         position=(layout[0], layout[1]),
                         size=(layout[2], layout[3]),
                         widgets=[Label(text=str(i) + " " + track.synthesizer.__class__.__name__,
                                        size=panel_size[0] - 2) for i, track in enumerate(music.tracks)])

    def get_panel_type(self) -> PanelType:
        return PanelType.tracks_list

    def get_widget_help(self) -> str:
        track = self._music.tracks[self.selection_index]
        return f"Track {track.track_id}. {track.synthesizer.get_help_text()}"

    def _get_panel_title(self) -> str:
        return "List of tracks."

    def _do_result(self, result: RenderResult) -> bool:
        did = super()._do_result(result=result)
        # If the user scrolls through the tracks list, change the piano roll and the synthesizer panel.
        if did:
            self.affected_panels.update({PanelType.piano_roll, {"track_index", self.selection_index},
                                         PanelType.synthesizer_panel, {"track_index": self.selection_index}})
        return did

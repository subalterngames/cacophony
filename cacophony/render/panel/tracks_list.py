from cacophony.render.globals import LAYOUTS
from cacophony.render.panel.scroll_panel import ScrollPanel
from cacophony.render.ui_element.label import Label
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
                         elements=[Label(text=str(i) + " " + track.synthesizer.__class__.__name__,
                                         size=panel_size[0] - 2) for i, track in enumerate(music.tracks)])

    def get_panel_help(self) -> str:
        return "List of tracks. " + super().get_panel_help()

    def get_widget_help(self) -> str:
        track = self._music.tracks[self.selection_index]
        return f"Track {track.}"*

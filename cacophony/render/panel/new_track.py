from typing import List
from cacophony.synthesizer.util import get_synthesizers
from cacophony.render.panel.scroll_panel import ScrollPanel
from cacophony.render.panel.panel_type import PanelType
from cacophony.render.widget.widget import Widget
from cacophony.render.widget.label import Label
from cacophony.render.widget.divider import Divider
from cacophony.render.render_result import RenderResult
from cacophony.render.input_key import InputKey
from cacophony.render.globals import LAYOUTS
from cacophony.music.music import Music
from cacophony.music.track import Track


class NewTrack(ScrollPanel):
    def __init__(self, music: Music, current_track_index: int):
        """
        :param music: The music.
        :param current_track_index: The current track index. This is used for the undo stack.
        """

        self._music: Music = music
        self._synthesizer_callables = get_synthesizers()
        layout = LAYOUTS[self.__class__.__name__]
        panel_size = (layout[2], layout[3])
        w = panel_size[0] - 2
        widgets: List[Widget] = [Divider(text="Waveformers", width=w),
                                 Divider(text="Synthesizers", width=w)]
        self._synthesizer_labels: List[Widget] = [Label(text=sc.__name__,
                                                        size=w)
                                                  for sc in self._synthesizer_callables]
        widgets.extend(self._synthesizer_labels)
        super().__init__(title=f"New Track",
                         position=(layout[0], layout[1]),
                         size=(layout[2], layout[3]),
                         widgets=widgets)
        # This panel starts inactive.
        self.active = False
        self.current_track_index: int = current_track_index
        # Scroll past the dividers.
        for i in range(2):
            self.selection_index = 2
            self._widget_index = 2

    def get_panel_type(self) -> PanelType:
        return PanelType.new_track

    def _do_result(self, result: RenderResult) -> bool:
        did = super()._do_result(result=result)
        # We did something. Let's go with that.
        if did:
            return did
        # Selected a synthesizer.
        if InputKey.select in result.inputs_pressed and isinstance(self._widgets[self.selection_index], Label):
            # Get the selected synthesizer label.
            if self._widgets[self.selection_index] in self._synthesizer_labels:
                index = self._synthesizer_labels.index(self._widgets[self.selection_index])
                track_id = max([track_id for track_id in self._music.tracks]) + 1 if len(self._music.tracks) > 0 else 0
                current_track_index: int = self.current_track_index
                new_track_index: int = len(self._music.tracks)
                # Add to the undo stack.
                self.undo_stack.append((self._undo_create_track, {"new_track_index": new_track_index,
                                                                  "old_track_index": current_track_index}))
                # Add the track.
                track: Track = Track(track_id=track_id,
                                     synthesizer=self._synthesizer_callables[index]())
                self._music.tracks.append(track)
                # Tell the panels to update.
                self.active = False
                self.affected_panels.update({PanelType.piano_roll: {"track_index": new_track_index},
                                             PanelType.synthesizer_panel: {"track_index": new_track_index},
                                             PanelType.tracks_list: {"selection_index": new_track_index,
                                                                     "active": True}})
                return True
        # Cancel.
        elif InputKey.cancel in result.inputs_pressed:
            self.active = False
            self.affected_panels.update({PanelType.tracks_list, {"active": True}})
            return True
        return False

    def _undo_create_track(self, new_track_index: int, old_track_index: int) -> None:
        """
        Pop at the "new" index and set the panels to the "old" index.

        :param new_track_index: The new track index (the one we're removing).
        :param old_track_index: The old track index (the one we were previously on).
        """

        self._music.tracks.pop(new_track_index)
        self.affected_panels.update({PanelType.piano_roll: {"track_index": old_track_index},
                                     PanelType.synthesizer_panel: {"track_index": old_track_index},
                                     PanelType.tracks_list: {"selection_index": old_track_index,
                                                             "active": True}})

    def _get_panel_title(self) -> str:
        return "New Track"

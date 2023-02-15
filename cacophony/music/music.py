from typing import List
from cacophony.music.track import Track


class Music:
    """
    A music has tracks.
    """

    def __init__(self, bpm: int, tracks: List[Track] = None):
        """
        :param bpm: The beats per minute.
        :param tracks: A list of tracks. If None, the list is empty.
        """

        self.bpm: int = bpm
        if tracks is None:
            self.tracks: List[Track] = list()
        else:
            self.tracks = tracks

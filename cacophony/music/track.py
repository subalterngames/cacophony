from typing import List
from cacophony.music.note import Note
from cacophony.synthesizer.synthesizer import Synthesizer


class Track:
    """
    A track has notes.
    """

    def __init__(self, synthesizer: Synthesizer, notes: List[Note] = None):
        """
        :param synthesizer: The track's synthesizer.
        :param notes: The notes. If None, the list is empty.
        """

        self.synthesizer: Synthesizer = synthesizer
        if notes is None:
            self.notes: List[Note] = list()
        else:
            self.notes = notes

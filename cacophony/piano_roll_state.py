class PianoRollState:
    """
    The state of the piano roll.
    """

    def __init__(self, time_0: float, note_0: int, selected_note_index: int):
        """
        :param time_0: The current rendered start time.
        :param note_0: The current lowest note.
        :param selected_note_index: The index of the selected note.
        """

        self.time_0: float = time_0
        self.note_0: int = note_0
        self.selected_note_index: int = selected_note_index

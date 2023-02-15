from typing import Optional


class Note:
    """
    A musical note.
    """

    def __init__(self, note: Optional[int], start: float, duration: float, volume: int):
        """
        :param note: The note as an integer. If None, this is silence.
        :param start: The start in terms of beats. For example, 4.0 would be four beats after the start of the track.
        :param duration: The duration in terms of beats. For example, 4.0 would be four beats (NOT 4 seconds).
        :param volume: The volume (0-127).
        """

        self.note: Optional[int] = note
        self.start: float = start
        self.duration: float = duration
        self.volume: int = volume

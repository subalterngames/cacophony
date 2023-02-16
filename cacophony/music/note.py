from __future__ import annotations
from struct import pack, unpack
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

    def serialize(self) -> bytes:
        """
        :return: A serialize bytestring of this note.
        """

        bs = bytearray()
        bs.extend(bytes([0 if self.note is None else self.note + 1, self.volume]))
        bs.extend(pack(">ff", self.start, self.duration))
        return bytes(bs)

    @staticmethod
    def deserialize(bs: bytes, index: int) -> Note:
        """
        :param bs: The save file bytestring.
        :param index: The starting index for the serialized note.

        :return: A Note.
        """

        note: Optional[int] = int(bs[index])
        if note == 0:
            note = None
        else:
            note -= 1
        volume = int(bs[index + 1])
        ff = unpack(">ff", bs[index + 2: index + 10])
        start = round(ff[0], 6)
        duration = round(ff[1], 6)
        return Note(note=note, start=start, duration=duration, volume=volume)

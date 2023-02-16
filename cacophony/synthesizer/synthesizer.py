from __future__ import annotations
from abc import ABC, abstractmethod
from overrides import final
from cacophony.music.note import Note


class Synthesizer(ABC):
    @final
    def audio(self, note: Note, bpm: int) -> bytes:
        """
        Synthesize a note.

        :param note: The note.
        :param bpm: The beats per minute.

        :return: A bytestring of audio samples.
        """

        # Return silence.
        if note.note is None:
            return b''
        # Return a note.
        else:
            return self._audio(note=note, duration=Synthesizer.get_duration(bpm=bpm, beat=note.duration))

    @abstractmethod
    def get_channels(self) -> int:
        """
        :return: The number of audio channels.
        """

        raise Exception()

    @staticmethod
    def get_duration(bpm: int, beat: float) -> float:
        """
        :param bpm: The beats per minute.
        :param beat: The duration in terms of beats.

        :return: The duration in terms of seconds.
        """

        return 60.0 / bpm * beat

    @abstractmethod
    def serialize(self) -> bytes:
        """
        :return: A bytestring of the serialization of this synthesizer.
        """

        raise Exception()

    @staticmethod
    @abstractmethod
    def deserialize(bs: bytes, index: int) -> Synthesizer:
        """
        :param bs: The save file bytestring.

        :param index: The starting index.

        :return: A synthesizer.
        """

        raise Exception()

    @abstractmethod
    def _audio(self, note: Note, duration: float) -> bytes:
        """
        Synthesize a note.

        :param note: The note.
        :param duration: The duration of the note in seconds.

        :return: A bytestring of audio samples.
        """

        raise Exception()

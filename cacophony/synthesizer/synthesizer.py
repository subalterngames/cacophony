from __future__ import annotations
from abc import ABC, abstractmethod
from overrides import final
from h5py import Group
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

    @final
    def serialize(self, track_group: Group) -> None:
        """
        :param track_group: The HDF5 group that the synthesizer's group will be serialized to.
        """

        synthesizer_group: Group = track_group.create_group(name="synthesizer")
        # Remember the name.
        synthesizer_group.attrs["type"] = self.__class__.__name__
        self._serialize(group=synthesizer_group)

    @staticmethod
    @abstractmethod
    def deserialize(group: Group) -> Synthesizer:
        """
        :param group: The synthesizer HDF5 group.

        :return: A synthesizer.
        """

        raise Exception()

    @abstractmethod
    def get_help_text(self) -> str:
        """
        :return: Help text for text-to-speech.
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

    @abstractmethod
    def _serialize(self, group: Group) -> None:
        """
        Serialize the synthesizer.

        :param group: The synthesizer's HDF5 group.
        """

        raise Exception()

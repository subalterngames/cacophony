from __future__ import annotations
from abc import ABC, abstractmethod
from typing import List
from inspect import signature
from pydoc import locate
from overrides import final
import numpy as np
from h5py import Group
from cacophony.music.note import Note
from cacophony.music.beat import Beat
from cacophony.util import get_duration
from cacophony.callbacker.int_list import IntList, zero_127
from cacophony.callbacker.enum_list import EnumList
from cacophony.callbacker.value import Value


class Synthesizer(ABC):
    """
    Abstract base class for a synthesizer.
    """

    def __init__(self, beat_index: int = 5, gain_index: int = 127, use_volume: bool = True, volume_index: int = 127):
        """
        :param beat_index: The index of the beat.
        :param gain_index: An index for gain values.
        :param use_volume: If True, use the value of `volume` for all new notes. If False, use the note's volume value.
        :param volume_index: An index for volume values.
        """

        self.beat: EnumList[Beat] = EnumList(t=Beat, index=beat_index, tts="Set the beat for all new notes.")
        self.gain: IntList = zero_127(index=gain_index, tts="Set the gain for this track.")
        self.use_volume: Value[bool] = Value(value=use_volume,
                                             tts="Toggle how volume is set. "
                                                 "If selected, use the volume value below for all new notes. "
                                                 "If not selected, use volume values from MIDI input.")
        self.volume: IntList = zero_127(index=volume_index, tts="Set the volume for all new notes. "
                                                                "This is ignored if the previous widget isn't selected.")

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
            gainf = self.gain.get() / 127.0
            # Use the global volume.
            if self.use_volume:
                volume = int(self.volume.get() * gainf)
            # Use the note's volume.
            else:
                volume = int(note.volume * gainf)
            return self._audio(note=note, volume=volume, duration=get_duration(bpm=bpm, beat=note.duration))

    @abstractmethod
    def get_channels(self) -> int:
        """
        :return: The number of audio channels.
        """

        raise Exception()

    @final
    def serialize(self, track_group: Group) -> None:
        """
        :param track_group: The HDF5 group that the synthesizer's group will be serialized to.
        """

        synthesizer_group: Group = track_group.create_group(name="synthesizer")
        # Remember the name.
        synthesizer_group.attrs["type"] = f"{self.__class__.__module__}.{self.__class__.__qualname__}"
        ints: List[int] = list()
        bools: List[bool] = list()
        for k in sorted(self.__dict__.keys()):
            # Ignore private variables.
            if k[0] == "_":
                continue
            klass = self.__dict__[k].__class__
            heritage = [kl.__name__ for kl in klass.__bases__]
            heritage.insert(0, klass.__name__)
            # For lists, we just need the index.
            if "IndexedList" in heritage or "Dictionary" in heritage:
                ints.append(self.__dict__[k].index)
            elif "Value" in heritage:
                v = self.__dict__[k].value
                if isinstance(v, bool):
                    bools.append(v)
                elif isinstance(v, int):
                    ints.append(v)
                elif isinstance(v, str):
                    synthesizer_group.attrs[k] = v
                else:
                    raise Exception(f"Unsupported value type: {v} {v.__class__}")
            else:
                raise Exception(f"Not supported: {klass.__name__}")
        # Serialize values.
        dtype = np.uint8 if max(ints) < 255 else np.int32
        synthesizer_group.create_dataset(name="ints",
                                         data=np.array(ints, dtype=dtype),
                                         dtype=dtype)
        synthesizer_group.create_dataset(name="bools",
                                         data=np.array(bools, dtype=bool),
                                         dtype=bool)

    @staticmethod
    @final
    def deserialize(group: Group) -> Synthesizer:
        """
        :param group: The synthesizer HDF5 group.

        :return: A synthesizer.
        """

        # Locate the class.
        t = locate(group.attrs["type"])
        # Get the constructor parameters.
        parameters = signature(t.__init__).parameters
        parameters = {key: value.annotation for key, value in parameters.items() if
                      key not in ['self', 'args', 'kwargs']}
        ints = np.array(group["ints"])
        bools = np.array(group["bools"])
        int_index = 0
        bool_index = 0
        kwargs = dict()
        for parameter in sorted(parameters.keys()):
            parameter_type = parameters[parameter]
            if parameter_type == "int":
                kwargs[parameter] = ints[int_index]
                int_index += 1
            elif parameter_type == "bool":
                kwargs[parameter] = bools[bool_index]
                bool_index += 1
            # Strings are stored as attributes.
            elif parameter_type == "str":
                kwargs[parameter] = group.attrs[parameter]
            else:
                raise Exception(f"Unsupported type: {parameter_type}")
        return t(**kwargs)

    @abstractmethod
    def get_help_text(self) -> str:
        """
        :return: Help text for text-to-speech.
        """

        raise Exception()

    @abstractmethod
    def _audio(self, note: Note, volume: int, duration: float) -> bytes:
        """
        Synthesize a note.

        :param note: The note.
        :param volume: The volume of the note.
        :param duration: The duration of the note in seconds.

        :return: A bytestring of audio samples.
        """

        raise Exception()

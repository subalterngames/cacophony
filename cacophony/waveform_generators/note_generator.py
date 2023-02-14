from typing import List, Dict
from abc import ABC, abstractmethod
from overrides import final
from pydub import AudioSegment
from cacophony.waveform_generators.waveform_generator import WaveformGenerator
from cacophony.waveform_generators.globals import FRAMERATE
from cacophony.waveform_generators.options import zero127
from cacophony.waveform_generators.waveform_generator_type import WaveformGeneratorType


class NoteGenerator(WaveformGenerator, ABC):
    """
    Abstract base class for generating musical note waveform data.
    """

    def __init__(self):
        """
        (no parameters)
        """

        beat: List[str] = ["1/64", "1/32", "1/16", "1/8", "1/6", "1/4", "1/3", "1/2", "1", "2", "3", "4", "5", "6", "7", "8"]
        """:field
        A dictionary of possible beats. Key = The name of the beat, e.g. 1/64. Value = The float value.
        """
        self.beat: Dict[str, float] = dict()
        for b in beat:
            bs = b.split("/")
            self.beat[b] = round(float(bs[0]) / float(bs[1]), 3)
        """:field
        A list of possible volumes (0-127).
        """
        self.volume: List[int] = zero127()

    def get(self, **kwargs) -> bytes:
        """
        :param kwargs: Additional args.

        :return: A bytestring of the waveform.
        """

        # Get the duration.
        duration: float = NoteGenerator._get_duration(bpm=kwargs["bpm"], beat=kwargs["beat_length"])
        # Return silence.
        if kwargs["note"] is None:
            return AudioSegment.silent(duration * 1000, frame_rate=FRAMERATE).raw_data
        # Return a note.
        else:
            return self._get(**kwargs)

    @final
    def get_type(self) -> WaveformGeneratorType:
        return WaveformGeneratorType.notes

    @abstractmethod
    def _get(self, **kwargs) -> bytes:
        """
        :param kwargs: Additional args.

        :return: The bytestring of the waveform.
        """

        raise Exception()

    @staticmethod
    def _get_duration(bpm: int, beat: float) -> float:
        """
        :param bpm: The beats per minute.
        :param beat: The beat as a float, e.g. 0.25

        :return: The duration in seconds of the beat.
        """

        return 60.0 / bpm * beat

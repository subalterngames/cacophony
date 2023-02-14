from abc import ABC, abstractmethod
from cacophony.waveform_generators.waveform_generator_type import WaveformGeneratorType


class WaveformGenerator(ABC):
    """
    Abstract base class for generating waveform data.
    """

    @abstractmethod
    def get(self, **kwargs) -> bytes:
        """
        :param kwargs: The args.

        :return: A bytestring of the waveform.
        """

        raise Exception()

    @abstractmethod
    def get_type(self) -> WaveformGeneratorType:
        """
        :return: The [`WaveformGeneratorType`](waveform_generator_type.md)
        """

        raise Exception()

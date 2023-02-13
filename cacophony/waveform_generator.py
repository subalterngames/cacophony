from abc import ABC, abstractmethod
from cacophony.waveform_generator_type import WaveformGeneratorType


class WaveformGenerator(ABC):
    @abstractmethod
    def get(self, **kwargs) -> bytes:
        raise Exception()

    @abstractmethod
    def get_type(self) -> WaveformGeneratorType:
        raise Exception()


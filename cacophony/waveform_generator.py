from abc import ABC, abstractmethod


class WaveformGenerator(ABC):
    @abstractmethod
    def get(self, **kwargs) -> bytes:
        raise Exception()

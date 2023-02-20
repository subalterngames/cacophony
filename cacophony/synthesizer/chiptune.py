from __future__ import annotations
from typing import Union, Callable
from pygame.midi import midi_to_frequency
import numpy as np
from h5py import Group
from chipnumpy.synthesizer import Synthesizer as ChipSynth
from cacophony.synthesizer.chiptune_pcm import ChiptunePCM
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.music.note import Note


class Chiptune(Synthesizer):
    """
    A simple chiptune synthesizer.
    """

    def __init__(self, pcm: ChiptunePCM):
        self._pcm: ChiptunePCM = ChiptunePCM.sine
        self._synth: ChipSynth = ChipSynth(seed=0)
        self._generator: Callable[[Union[str, float], float, float], bytes] = self._synth.sine
        self.set(pcm=pcm)

    def get_channels(self) -> int:
        return 1

    @staticmethod
    def deserialize(group: Group) -> Chiptune:
        return Chiptune(pcm=ChiptunePCM(int(group["pcm"][0])))

    def set(self, pcm: ChiptunePCM) -> None:
        """
        Set the chiptune generator.

        :param pcm: The PCM type.
        """

        self._pcm = pcm
        if self._pcm == ChiptunePCM.sine:
            self._generator = self._synth.sine
        elif self._pcm == ChiptunePCM.pulse:
            self._generator = self._synth.pulse
        elif self._pcm == ChiptunePCM.triangle:
            self._generator = self._synth.triangle
        elif self._pcm == ChiptunePCM.saw:
            self._generator = self._synth.sawtooth
        elif self._pcm == ChiptunePCM.noise:
            self._generator = self._synth.noise
        else:
            raise Exception(f"Undefined: {self._pcm}")

    def get_help_text(self) -> str:
        return f"Chiptune {self._pcm.name} waveform generator."

    def _audio(self, note: Note, duration: float) -> bytes:
        return self._generator(note=midi_to_frequency(note.note),
                               amplitude=note.volume / 127,
                               length=duration)

    def _serialize(self, group: Group) -> None:
        group.create_dataset(name="pcm", shape=[1], data=[self._pcm.value], dtype=np.uint8)

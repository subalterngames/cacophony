from typing import Union, Callable, Dict
from pygame.midi import midi_to_frequency
from chipnumpy.synthesizer import Synthesizer as ChipSynth
from cacophony.synthesizer.chiptune_pcm import ChiptunePCM
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.callbacker.enum_list import EnumList


class Chiptune(Synthesizer):
    """
    A simple chiptune synthesizer.
    """

    def __init__(self, pcm_index: int, beat_index: int = 5, gain_index: int = 127, use_volume: bool = True, volume_index: int = 127):
        """
        :param pcm_index: The index for the PCM type.
        :param beat_index: The index of the beat.
        :param gain_index: An index for gain values.
        :param use_volume: If True, use the value of `volume` for all new notes. If False, use the note's volume value.
        :param volume_index: An index for volume values.
        """

        super().__init__(beat_index=beat_index, gain_index=gain_index, use_volume=use_volume, volume_index=volume_index)
        self.pcm: EnumList[ChiptunePCM] = EnumList(t=ChiptunePCM,
                                                   tts="Set chiptune PCM type.",
                                                   index=pcm_index)
        self._synth: ChipSynth = ChipSynth(seed=0)
        self._generators: Dict[ChiptunePCM, Callable[[Union[str, float], float, float], bytes]] = {ChiptunePCM.noise: self._synth.noise,
                                                                                                   ChiptunePCM.pulse: self._synth.pulse,
                                                                                                   ChiptunePCM.sine: self._synth.sine,
                                                                                                   ChiptunePCM.saw: self._synth.sawtooth,
                                                                                                   ChiptunePCM.triangle: self._synth.triangle}

    def get_channels(self) -> int:
        return 1

    def get_help_text(self) -> str:
        return f"Chiptune {self.pcm.get().name} waveform generator."

    def get(self, note: int, volume: int, duration: float) -> bytes:
        return self._generators[self.pcm.get()](note=midi_to_frequency(note),
                                                amplitude=volume / 127,
                                                length=duration)

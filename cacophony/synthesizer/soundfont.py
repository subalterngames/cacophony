from pathlib import Path
from typing import Optional, Dict, Union
import numpy as np
from sf2_loader.read_sf2.read_sf2 import sf2_loader
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.music.note import Note
from cacophony.path_util import get_string_path
from cacophony.music.globals import FRAMERATE


class SoundFont(Synthesizer):
    """
    A SoundFont synthesizer.
    """

    def __init__(self, channel: int):
        """
        :param channel: The audio channel.
        """

        self.channel: int = channel
        self._loader: Optional[sf2_loader] = None
        self.instruments: Dict[int, Dict[int, str]] = dict()
        self.bank: int = 0
        self.preset: int = 0

    def get_channels(self) -> int:
        return 2

    def load(self, path: Union[str, Path]) -> bool:
        """
        Load a SoundFont.

        :param path: The file path as a string or `Path`.

        :return: True if the SoundFont loaded.
        """

        self._loader = sf2_loader(get_string_path(path))
        if self._loader is None:
            return False
        instruments = self._loader.all_instruments()
        # Set the instruments.
        self.instruments.clear()
        for bank in instruments:
            b = int(bank)
            self.instruments[b] = dict()
            for instrument in instruments[bank]:
                self.instruments[b][int(instrument)] = instruments[bank][instrument]
        # Reset the bank and preset.
        self.bank = 0
        self.preset = 0
        return True

    def set_instrument(self, bank: int, preset: int) -> bool:
        """
        Set the bank and instrument.

        :param bank: The bank.
        :param preset: The preset (instrument) integer value.

        :return: True if the instrument was set.
        """

        if self._loader is None or bank not in self.instruments or preset not in self.instruments[bank]:
            return False
        self._loader.change(channel=self.channel, bank=bank, preset=preset)
        return True

    def _audio(self, note: Note, duration: float) -> bytes:
        # Note on event.
        self._loader.synth.noteon(self.channel, note.note, note.volume)
        # Get samples for the duration.
        a: np.ndarray = self._loader.synth.get_samples(len=int(FRAMERATE * duration))
        # Note off.
        self._loader.synth.noteoff(self.channel, note.note)
        # Return the int16 samples.
        return np.int16(a).tobytes()

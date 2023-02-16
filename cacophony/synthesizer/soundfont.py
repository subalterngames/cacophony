from __future__ import annotations
from struct import pack
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
        self._path: str = ""
        self.instruments: Dict[int, Dict[int, str]] = dict()
        self.bank: int = 0
        self.preset: int = 0

    def get_channels(self) -> int:
        return 2

    def serialize(self) -> bytes:
        # ID, channel, bank, preset.
        bs = bytearray(bytes([2, self.channel, self.bank, self.preset]))
        path = self._path.encode("utf-8")
        # The length of the path.
        bs.extend(pack(">i", len(path)))
        # The path.
        bs.extend(path)
        return bs

    @staticmethod
    def deserialize(bs: bytes, index: int) -> SoundFont:
        channel = int(bs[index + 1])
        bank = int(bs[index + 2])
        preset = int(bs[index + 3])
        path_length = int.from_bytes(bs[index + 4: index + 8], "big")
        path: str = bs[index + 9: index + 9 + path_length].decode("utf-8")
        s = SoundFont(channel=channel)
        s.load(path=path)
        s.set_instrument(bank=bank, preset=preset)
        return s

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
        # Reset the bank, preset, and path.
        self.bank = 0
        self.preset = 0
        self._path = get_string_path(path)
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

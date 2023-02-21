from __future__ import annotations
from pathlib import Path
from typing import Optional, Dict, Union
import numpy as np
from h5py import Group
from sf2_loader.read_sf2.read_sf2 import sf2_loader
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.music.note import Note
from cacophony.util import get_string_path
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

    @staticmethod
    def deserialize(group: Group) -> SoundFont:
        sf: np.ndarray = np.array(group["sf"], dtype=np.uint8)
        channel = int(sf[0])
        bank = int(sf[1])
        preset = int(sf[2])
        path = str(group["path"])
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

    def get_help_text(self) -> str:
        if self._loader is None or self._path is None or self.bank not in self.instruments:
            return "SoundFont. No file loaded."
        return f"SoundFont {Path(self._path).name}. {self.instruments[self.bank][self.preset]}."

    def _audio(self, note: Note, duration: float) -> bytes:
        # Note on event.
        self._loader.synth.noteon(self.channel, note.note, note.volume)
        # Get samples for the duration.
        a: np.ndarray = self._loader.synth.get_samples(len=int(FRAMERATE * duration))
        # Note off.
        self._loader.synth.noteoff(self.channel, note.note)
        # Return the int16 samples.
        return np.int16(a).tobytes()

    def _serialize(self, group: Group) -> None:
        # Serialize the data.
        group.create_dataset(name="sf", shape=[3], data=np.array([self.channel, self.bank, self.preset], dtype=np.uint8),
                             dtype=np.uint8)
        # Add the file path.
        group.attrs["path"] = self._path

from __future__ import annotations
from pathlib import Path
from typing import Optional
import numpy as np
from sf2_loader.read_sf2.read_sf2 import sf2_loader
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.music.note import Note
from cacophony.util import get_path
from cacophony.music.globals import FRAMERATE
from cacophony.callbacker.int_list import IntList, zero_127
from cacophony.callbacker.dictionary import Dictionary
from cacophony.callbacker.file_path import FilePath


class SoundFont(Synthesizer):
    """
    A SoundFont synthesizer.
    """

    def __init__(self, path: str = "", channel_index: int = 0, bank_index: int = 0, preset_index: int = 0,
                 beat_index: int = 5, gain_index: int = 127, use_volume: bool = True, volume_index: int = 127):
        """
        :param path: The path to the SoundFont file.
        :param channel_index: The index of the audio channel.
        :param bank_index: The bank index.
        :param preset_index: The preset index.
        :param beat_index: The index of the beat.
        :param gain_index: An index for gain values.
        :param use_volume: If True, use the value of `volume` for all new notes. If False, use the note's volume value.
        :param volume_index: An index for volume values.
        """

        self._path: str = ""
        self._loader: Optional[sf2_loader] = None
        # Set the channel.
        self.channel: IntList = zero_127(index=channel_index, tts="")
        # A dictionary of instrument integer keys and names.
        self.preset: Dictionary = Dictionary(values=dict(), tts="")
        # A list of bank integers.
        self.bank: IntList = IntList(values=[], callback=self._set_bank, tts="")
        # The path to the file. When this is set, we'll try loading the SoundFont.
        self.path: FilePath = FilePath(suffixes=[".sf2", ".sf3"], value=path, callback=self._set_path, tts="")
        self._set_path()
        # Set the indices of the bank and preset.
        self.bank.index = bank_index
        self.preset.index = preset_index
        super().__init__(beat_index=beat_index, gain_index=gain_index, use_volume=use_volume, volume_index=volume_index)

    def get_channels(self) -> int:
        return 2

    def _set_path(self) -> None:
        """
        Invoked when the path is set.
        """

        # Invalid path.
        v: str = self.path.value
        if v == "" or not get_path(v).exists():
            self._loader = None
            return
        # Don't unload the SoundFont.
        if self._loader is not None and self._path == v:
            return
        self._path = v
        # Try to load the SoundFont.
        self._loader = sf2_loader(v)
        if self._loader is None:
            return
        instruments = self._loader.all_instruments()
        # Set the banks. This will invoke the preset callback.
        self.bank.values = [int(bank) for bank in instruments]
        self.bank.index = 0

    def _set_bank(self) -> None:
        """
        Invoked when the bank is set.
        """

        if self._loader is None:
            return
        instruments = self._loader.all_instruments()[self.bank.get()]
        self.preset = Dictionary(values={int(i): instruments[i] for i in instruments},
                                 index=0,
                                 callback=self._set_preset,
                                 tts="")
        self._set_preset()

    def _set_preset(self) -> None:
        """
        Invoked when the preset is set.
        """

        if self._loader is None:
            return
        # Set the instrument.
        self._loader.change(channel=self.channel.get(), bank=self.bank.get(), preset=self.preset.get())

    def get_help_text(self) -> str:
        if self._loader is None or self._path is None:
            return "SoundFont. No file loaded."
        return f"SoundFont {Path(self._path).name}. {self.preset.get_str()}."

    def _audio(self, note: Note, volume: int, duration: float) -> bytes:
        # Note on event.
        channel = self.channel.get()
        self._loader.synth.noteon(channel, note.note, volume)
        # Get samples for the duration.
        a: np.ndarray = self._loader.synth.get_samples(len=int(FRAMERATE * duration))
        # Note off.
        self._loader.synth.noteoff(channel, note.note)
        # Return the int16 samples.
        return np.int16(a).tobytes()

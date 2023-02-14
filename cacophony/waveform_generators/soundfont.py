from typing import List, Dict, Optional
from sf2_loader.read_sf2.read_sf2 import sf2_loader
from cacophony.waveform_generators.globals import FRAMERATE, SAMPLE_WIDTH, CHANNELS
from cacophony.waveform_generators.note_generator import NoteGenerator


class SoundFont(NoteGenerator):
    """
    Load a SoundFont file and generate notes.
    """

    # A cached dictionary of loaded sound fonts. Key = The file path.
    __SOUND_FONTS: Dict[str, sf2_loader] = dict()

    def __init__(self):
        """
        (no parameters)
        """

        super().__init__()
        """:field
        The path to the SoundFont file.
        """
        self.path: str = ""
        # If this doesn't match self.path, try to reload the SoundFont file.
        self.__current_path: str = ""
        # The loader.
        self.__loader: Optional[sf2_loader] = None
        # All instruments in the SoundFont. Key = Bank. Value = Dictionary. Key = Instrument ID. Value = Instrument name.
        self.__all_instruments: Dict[int, Dict[int, str]] = dict()
        # A list of all banks.
        self.bank: List[int] = list()
        # The current bank.
        self.__bank: int = 0
        self.__instrument: int = 0
        # A dictionary of instruments in this bank. Key = Instrument ID. Value = Instrument name.
        self.instrument: Dict[int, str] = dict()
        """:field
        A dictionary of possible decay times in beat length. Key = The name of the beat, e.g. 1/64. Value = The float value.
        """
        self.decay: Dict[str, float] = {"0": 0}
        self.decay.update({k: v for k, v in self.beat.items()})

    def get(self, note: Optional[str], beat_length: float, bpm: int, bank: int, instrument: int, volume: int, decay: float) -> bytes:
        # There is no loader. Return nothing.
        if not self.__load_soundfont():
            return b''
        # Change banks and presets.
        if bank != self.__bank or instrument != self.__instrument:
            self.__loader.change(bank=bank, channel=0, preset=instrument)
        return super().get(note=note, beat_length=beat_length, bpm=bpm, bank=bank, instrument=instrument, volume=volume, decay=decay)

    def _get(self, note: Optional[str], beat_length: float, bpm: int, bank: int, instrument: int, volume: int, decay: float) -> bytes:
        return self.__loader.export_note(note,
                                         duration=NoteGenerator._get_duration(bpm=bpm, beat=beat_length),
                                         decay=NoteGenerator._get_duration(bpm=bpm, beat=decay),
                                         volume=volume,
                                         channel=0,
                                         start_time=0,
                                         sample_width=SAMPLE_WIDTH,
                                         channels=CHANNELS,
                                         frame_rate=FRAMERATE,
                                         name=None,
                                         format='wav',
                                         effects=None,
                                         bpm=bpm,
                                         export_args={},
                                         get_audio=True).raw_data

    def __load_soundfont(self) -> bool:
        """
        Try to load the SoundFont.

        :return: True if we loaded the SoundFont.
        """

        # We already loaded this SoundFont.
        if self.path == self.__current_path and self.__loader is not None:
            return True
        # Get a cached SoundFont.
        if self.path in SoundFont.__SOUND_FONTS:
            self.__loader = SoundFont.__SOUND_FONTS[self.path]
        # Try to load the SoundFont.
        else:
            try:
                self.__loader = sf2_loader(self.path)
                SoundFont.__SOUND_FONTS[self.path] = self.__loader
            except ValueError:
                return False
        self.__current_path = self.path
        # Get all of the instruments.
        all_instruments = self.__loader.all_instruments()
        self.__all_instruments.clear()
        # Convert the dictionary to integer keys.
        for bank in all_instruments:
            b = int(bank)
            self.__all_instruments[b] = dict()
            for i in all_instruments[bank]:
                self.__all_instruments[b][int(i)] = all_instruments[bank][i]
        # Set the possible banks.
        self.bank.clear()
        self.bank.extend(list(self.__all_instruments.keys()))
        # Set the current bank.
        self.__bank = self.bank[0]
        # Set the instruments.
        self.instrument.clear()
        self.instrument.update({k: v for k, v in self.__all_instruments[self.__bank].items()})
        instrument_keys = list(self.instrument.keys())
        self.__instrument = instrument_keys[0]
        return True

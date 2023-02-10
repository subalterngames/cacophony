from typing import Dict, List, Optional
from pathlib import Path
import sf2_loader as sf
from cacophony.waveform_generator import WaveformGenerator
from cacophony.int_range import IntRange
from cacophony.zero_127 import Zero127
from cacophony.float_range import FloatRange
from cacophony.notes_range import NotesRange


class SoundFont(WaveformGenerator):
    BEAT: float = 1
    BPM: int = 60

    def __init__(self):
        self.path: Optional[str] = None
        self.__all_instruments: Dict[int, Dict[int, str]] = dict()
        self.__loader = None
        self.__banks: List[int] = list()
        self.__instruments: Dict[str, str] = dict()
        self.bank: IntRange = IntRange(1)
        self.instrument: IntRange = IntRange(1)
        self.__instrument: int = -1
        self.__bank: int = -1
        self.decay: FloatRange = FloatRange(start=0, end=5, index=0, step=0.1)
        self.volume: Zero127 = Zero127(index=126)
        self.note: NotesRange = NotesRange()

    def get(self) -> bytes:
        if self.path is None or not Path(self.path).exists():
            return b''
        # Try to load the sound font.
        if self.__loader is None or self.path not in self.__loader:
            try:
                self.__loader = sf.sf2_loader(self.path)
                all_instruments = self.__loader.all_instruments()
                for bank in all_instruments:
                    b = int(bank)
                    self.__banks.append(b)
                    self.__all_instruments[b] = dict()
                    for i in all_instruments[bank]:
                        self.__all_instruments[b][int(i)] = all_instruments[bank][i]
                # Get the banks.
                self.__banks = [int(b) for b in self.__all_instruments]
                self.bank = IntRange(len(self.__banks), callback=self.__set_bank)
                self.__set_bank(0)
            except ValueError:
                return b''
        bank = self.bank.get()
        instrument = self.instrument.get()
        if bank != self.__bank or instrument != self.__instrument:
            self.__instrument = instrument
            self.__bank = bank
            self.__loader.change(bank=bank, channel=0, preset=instrument)
        duration = SoundFont.BPM / 60 * SoundFont.BEAT
        return self.__loader.export_note(self.note.to_str(),
                                         duration=duration,
                                         decay=self.decay.get(),
                                         volume=self.volume.get(),
                                         channel=0,
                                         start_time=0,
                                         sample_width=2,
                                         channels=2,
                                         frame_rate=44100,
                                         name=None,
                                         format='wav',
                                         effects=None,
                                         bpm=SoundFont.BPM,
                                         export_args={},
                                         get_audio=True).raw_data

    def __set_bank(self, bank: int) -> None:
        self.__instruments = self.__all_instruments[bank]
        instruments = list(self.__instruments.keys())
        self.__bank = bank
        self.__instrument = instruments[0]
        self.__loader.change(bank=bank, channel=0, preset=instruments[0])
        self.instrument = IntRange(len(instruments), callback=self.__set_instrument, to_str=self.__instrument_str)

    def __set_instrument(self, instrument: int) -> None:
        print(instrument)
        self.__instrument = instrument
        self.__loader.change(bank=self.__bank, channel=0, preset=self.__instrument)

    def __instrument_str(self) -> str:
        return self.__all_instruments[self.bank.get()][self.instrument.get()]



s = SoundFont()
s.path = "D:/SoundFonts/ms_basic.sf3"
s.bank.set(0)
s.instrument.set(44)
s.note.set(59)
a = s.get()
from time import sleep
import pygame.mixer

pygame.mixer.init(allowedchanges=pygame.AUDIO_ALLOW_CHANNELS_CHANGE)
sound = pygame.mixer.Sound(a)
sound.play()
sleep(sound.get_length())

from typing import Dict, List, Optional
from sf2_loader.read_sf2.read_sf2 import sf2_loader
from pydub import AudioSegment
from cacophony.waveform_generator import WaveformGenerator
from cacophony.options import zero127
from cacophony.callbacker import Callbacker
from cacophony.globals import NOTES, FRAMERATE, SAMPLE_WIDTH, CHANNELS
from cacophony.waveform_generator_type import WaveformGeneratorType


class SoundFont(WaveformGenerator):
    __SOUND_FONTS: Dict[str, sf2_loader] = dict()

    def __init__(self):
        self.path: Callbacker[str] = Callbacker(callback=self.__load_soundfont)
        self.__loader: Optional[sf2_loader] = None
        self.__all_instruments: Dict[int, Dict[int, str]] = dict()
        self.bank: List[int] = list()
        self.__bank: int = 0
        self.__instrument: int = 0
        self.instrument: Dict[int, str] = dict()
        self.beat: List[str] = ["1/64", "1/32", "1/16", "1/8", "1/6", "1/4", "1/3", "1/2", "1", "2", "3", "4", "5", "6", "7", "8"]
        self.decay: List[str] = self.beat[:]
        self.decay.insert(0, "0")
        self.volume: List[int] = zero127()
        self.note: List[str] = NOTES[:]

    def get(self, bank: int, instrument: int, note: Optional[str], bpm: int, beat: str, volume: int, decay: str) -> bytes:
        if self.__loader is None:
            return b''
        # Change banks and presets.
        if bank != self.__bank or instrument != self.__instrument:
            self.__loader.change(bank=bank, channel=0, preset=instrument)
        if note is None:
            return AudioSegment.silent(duration=SoundFont.__get_duration(bpm=bpm, beat=beat) * 1000, frame_rate=FRAMERATE)
        return self.__loader.export_note(note,
                                         duration=SoundFont.__get_duration(bpm=bpm, beat=beat),
                                         decay=SoundFont.__get_duration(bpm=bpm, beat=decay),
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

    def get_type(self) -> WaveformGeneratorType:
        return WaveformGeneratorType.notes

    def __load_soundfont(self, path: Optional[str]) -> None:
        if path is None:
            return
        elif path in SoundFont.__SOUND_FONTS:
            self.__loader = SoundFont.__SOUND_FONTS[path]
        else:
            try:
                self.__loader = sf2_loader(path)
                SoundFont.__SOUND_FONTS[path] = self.__loader
            except ValueError:
                self.path.set(None)
                return
        all_instruments = self.__loader.all_instruments()
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

    @staticmethod
    def __get_duration(bpm: int, beat: str) -> float:
        # Get the duration.
        if "/" in beat:
            bs = beat.split("/")
            b = round(float(bs[0]) / float(bs[1]), 3)
        else:
            b = round(float(beat), 3)
        return 60.0 / bpm * b

from typing import Dict, List, Optional
import sf2_loader as sf
from pydub import AudioSegment
from cacophony.waveform_generator import WaveformGenerator
from cacophony.options import zero127
from cacophony.callbacker import Callbacker
from cacophony.globals import NOTES, FRAMERATE, SAMPLE_WIDTH, CHANNELS


class SoundFont(WaveformGenerator):
    def __init__(self):
        self.__loader = None
        self.__all_instruments: Dict[int, Dict[int, str]] = dict()
        self.banks: List[int] = list()
        self.__bank: int = 0
        self.__instrument: int = 0
        self.instruments: Dict[int, str] = dict()
        self.path: Callbacker[str] = Callbacker(callback=self.__load_soundfont)
        self.beats: List[str] = ["0", "1/64", "1/32", "1/16", "1/8", "1/6", "1/4", "1/3", "1/2", "1", "2", "3", "4", "5", "6", "7", "8"]
        self.decays: List[str] = self.beats[:]
        self.volumes: List[int] = zero127()
        self.notes: List[str] = NOTES[:]

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

    def __load_soundfont(self, path: Optional[str]) -> None:
        if path is None:
            return
        try:
            self.__loader = sf.sf2_loader(path)
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
        self.banks.clear()
        self.banks.extend(list(self.__all_instruments.keys()))
        # Set the current bank.
        self.__bank = self.banks[0]
        # Set the instruments.
        self.instruments.clear()
        self.instruments.update({k: v for k, v in self.__all_instruments[self.__bank].items()})
        instrument_keys = list(self.instruments.keys())
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

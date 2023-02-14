from typing import Optional


class Note:
    def __init__(self, note: Optional[str], start_beat: float, beat_length: float, volume: int):
        self.note: Optional[str] = note
        self.start_beat: float = start_beat
        self.beat_length: float = beat_length
        self.volume: int = volume

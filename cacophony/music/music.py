from __future__ import annotations
from struct import pack
from typing import List
from pydub import AudioSegment
from cacophony.music.track import Track


class Music:
    """
    A music has tracks.
    """

    def __init__(self, bpm: int, tracks: List[Track] = None):
        """
        :param bpm: The beats per minute.
        :param tracks: A list of tracks. If None, the list is empty.
        """

        self.bpm: int = bpm
        if tracks is None:
            self.tracks: List[Track] = list()
        else:
            self.tracks = tracks

    def audio(self) -> AudioSegment:
        """
        Generate audio from each track.

        :return: The combined audio as an AudioSegment.
        """

        a: AudioSegment = self.tracks[0].audio_segment(bpm=self.bpm)
        for i in range(1, len(self.tracks)):
            a = a.overlay(AudioSegment(self.tracks[i].audio_segment(bpm=self.bpm)))
        return a

    def serialize(self) -> bytes:
        bs = bytearray()
        bs.extend(pack(">ii", self.bpm, len(self.tracks)))
        for track in self.tracks:
            bs.extend(track.serialize())
        return bytes(bs)

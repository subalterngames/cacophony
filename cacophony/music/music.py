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
        """
        :return: A serialize bytestring of the music.
        """

        bs = bytearray()
        bs.extend(pack(">ii", self.bpm, len(self.tracks)))
        for track in self.tracks:
            bs.extend(track.serialize())
        return bytes(bs)

    @staticmethod
    def deserialize(bs: bytes) -> Music:
        """
        :param bs: A serialized bytestring of the music.

        :return: Deserialized music.
        """

        bpm = int.from_bytes(bs[0: 4], "big")
        num_tracks = int.from_bytes(bs[4: 8], "big")
        tracks = []
        index = 8
        # Deserialize the tracks.
        for i in range(num_tracks):
            tracks.append(Track.deserialize(bs=bs, index=index))
            # Advance the index.
            track_length: int = int.from_bytes(bs[index: index + 4], "big")
            index += track_length
        return Music(bpm=bpm, tracks=tracks)

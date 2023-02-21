from __future__ import annotations
from typing import List, Union
from pathlib import Path
from pydub import AudioSegment
from h5py import File, Group
from cacophony.music.track import Track
from cacophony.util import get_string_path, get_path


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
            a = a.overlay(self.tracks[i].audio_segment(bpm=self.bpm))
        return a

    def serialize(self, path: Union[str, Path]) -> None:
        """
        Serialize the music to an HDF5 file.

        :param path: The file path.
        :return: A serialize bytestring of the music.
        """

        p: Path = get_path(path)
        # Create the directory.
        if not p.parent.exists():
            p.parent.mkdir(parents=True)
        # Open the file.
        with File(get_string_path(path), "w") as f:
            # Serialize the bpm.
            f.create_dataset(name="bpm", shape=[1], data=[self.bpm], dtype=int)
            # Serialize the tracks.
            tracks_group: Group = f.create_group(name="tracks")
            for track in self.tracks:
                track.serialize(tracks_group=tracks_group)

    @staticmethod
    def deserialize(path: Union[str, Path]) -> Music:
        """
        :param path: The file path.

        :return: Deserialized music.
        """

        p: Path = get_path(path)
        if not p.exists():
            raise FileNotFoundError(f"File not found: {get_string_path(path)}")
        # Open the file.
        with File(get_string_path(path), "r") as f:
            # Deserialize the bpm.
            bpm = int(f["bpm"][0])
            # Deserialize the tracks.
            tracks: List[Track] = list()
            tracks_group: Group = f["tracks"]
            for track_id in tracks_group:
                tracks.append(Track.deserialize(tracks_group=tracks_group, track_id=track_id))
        return Music(bpm=bpm, tracks=tracks)

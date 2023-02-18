from __future__ import annotations
from io import BytesIO
from typing import List
from pydub import AudioSegment
from h5py import Group
import numpy as np
from cacophony.music.note import Note
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.synthesizer.chiptune import Chiptune
from cacophony.synthesizer.clatter import Clatter
from cacophony.synthesizer.soundfont import SoundFont
from cacophony.music.globals import FRAMERATE, SAMPLE_WIDTH


class Track:
    """
    A track has notes.
    """

    def __init__(self, track_id: int, synthesizer: Synthesizer, notes: List[Note] = None):
        """
        :param track_id: The track ID.
        :param synthesizer: The track's synthesizer.
        :param notes: The notes. If None, the list is empty.
        """

        self.track_id: int = track_id
        self.synthesizer: Synthesizer = synthesizer
        if notes is None:
            self.notes: List[Note] = list()
        else:
            self.notes = notes

    def audio_segment(self, bpm: int) -> AudioSegment:
        """
        Synthesize every note and generate an AudioSegment.

        :param bpm: The beats per minute.

        :return: An AudioSegment.
        """

        audio: AudioSegment = AudioSegment.silent(0, frame_rate=FRAMERATE)
        channels = self.synthesizer.get_channels()
        # Iterate through each note.
        for note in self.notes:
            # Generate audio.
            a = self.synthesizer.audio(note=note, bpm=bpm)
            # Create an audio segment.
            audio_segment = AudioSegment.from_raw(BytesIO(a),
                                                  sample_width=SAMPLE_WIDTH,
                                                  channels=channels,
                                                  frame_rate=FRAMERATE)
            # Get the start time.
            t0 = self.synthesizer.get_duration(bpm=bpm, beat=note.start)
            duration = self.synthesizer.get_duration(bpm=bpm, beat=note.duration)
            # Add new audio.
            if t0 >= audio.duration_seconds:
                audio += audio_segment
            # Overlay all audio.
            elif t0 + duration <= audio.duration_seconds:
                audio = audio.overlay(audio_segment, position=int(t0 * 1000))
            # Overlay some audio and add the remainder.
            else:
                s = int((audio.duration_seconds - t0) * 1000)
                audio = audio.overlay(audio_segment[:-s],
                                      position=int(t0 * 1000))
                audio += audio_segment[-s:]
        return audio

    def serialize(self, tracks_group: Group) -> None:
        """
        :param tracks_group: The HDF5 group that this track belongs to.
        """

        track_group: Group = tracks_group.create_group(name=str(self.track_id))
        # Serialize the synthesizer.
        self.synthesizer.serialize(track_group=track_group)
        # Serialize the notes.
        note_bytes: np.ndarray = np.zeros(shape=(len(self.notes), 2), dtype=np.uint8)
        note_floats: np.ndarray = np.zeros(shape=(len(self.notes), 2), dtype=np.float32)
        for i, note in enumerate(self.notes):
            note_bytes[i][0] = note.note
            note_bytes[i][1] = note.volume
            note_floats[i][0] = round(note.start, 6)
            note_floats[i][1] = round(note.duration, 6)
        track_group.create_dataset(name="note_bytes", data=note_bytes, dtype=np.uint8)
        track_group.create_dataset(name="note_floats", data=note_floats, dtype=np.float32)

    @staticmethod
    def deserialize(tracks_group: Group, track_id: str) -> Track:
        """
        :param tracks_group: The group of all tracks.
        :param track_id: The track ID as a string.

        :return: A Track.
        """

        track_group: Group = tracks_group[track_id]
        synthesizer_group: Group = track_group["synthesizer"]
        synthesizer_type: str = str(synthesizer_group.attrs["type"])
        if synthesizer_type == "Chiptune":
            synthesizer = Chiptune.deserialize(synthesizer_group)
        elif synthesizer_type == "Clatter":
            synthesizer = Clatter.deserialize(synthesizer_group)
        elif synthesizer_type == "SoundFont":
            synthesizer = SoundFont.deserialize(synthesizer_group)
        else:
            raise Exception(f"Unknown synthesizer type: {synthesizer_type}")
        # Get the notes.
        notes: List[Note] = list()
        note_bytes: np.ndarray = np.array(track_group["note_bytes"], dtype=np.uint8)
        note_floats: np.ndarray = np.array(track_group["note_floats"], dtype=np.float32)
        for i in range(note_bytes.shape[0]):
            notes.append(Note(note=int(note_bytes[i][0]),
                              start=round(float(note_floats[i][0]), 6),
                              duration=round(float(note_floats[i][1]), 6),
                              volume=int(note_bytes[i][1])))
        # Get the track.
        return Track(track_id=int(track_id), synthesizer=synthesizer, notes=notes)

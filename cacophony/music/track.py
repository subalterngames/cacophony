from __future__ import annotations
from struct import pack
from io import BytesIO
from typing import List
from pydub import AudioSegment
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

    def __init__(self, synthesizer: Synthesizer, notes: List[Note] = None):
        """
        :param synthesizer: The track's synthesizer.
        :param notes: The notes. If None, the list is empty.
        """

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

    def serialize(self) -> bytes:
        """
        :return: A serialized bytestring of this track.
        """

        bs = bytearray()
        # Serialize the synthesizer.
        s = self.synthesizer.serialize()
        # Get the length of the serialized synthesizer.
        bs.extend(pack(">i", len(s)))
        # Add the serialized synthesizer.
        bs.extend(s)
        # Get the length of the notes.
        bs.extend(pack(">i", len(self.notes) * 10))
        # Serialize the notes.
        for note in self.notes:
            bs.extend(note.serialize())
        return bytes(bs)

    @staticmethod
    def deserialize(bs: bytes, index: int) -> Track:
        """
        :param bs: The save file bytestring.
        :param index: The starting index for the serialized track.

        :return: A Track.
        """

        # Get the synthesizer ID.
        synth_id: int = int(bs[index + 4])
        if synth_id == 0:
            synthesizer = Chiptune.deserialize(bs=bs, index=index + 4)
        elif synth_id == 1:
            synthesizer = Clatter.deserialize(bs=bs, index=index + 4)
        elif synth_id == 2:
            synthesizer = SoundFont.deserialize(bs=bs, index=index + 4)
        else:
            raise Exception(f"Unknown synthesizer ID: {synth_id}")
        # Get the length of the synthesizer.
        synth_length: int = int.from_bytes(bs[index: index + 4], "big")
        # Get the length of the notes.
        notes_length = int.from_bytes(bs[index + synth_length: index + synth_length + 4], "big")
        # Get the notes.
        notes: List[Note] = list()
        for i in range(index + synth_length + 4, index + synth_length + 4 + notes_length, step=10):
            notes.append(Note.deserialize(bs=bs, index=i))
        # Get the track.
        return Track(synthesizer=synthesizer, notes=notes)

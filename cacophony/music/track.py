from io import BytesIO
from typing import List
from pydub import AudioSegment
from cacophony.music.note import Note
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.waveform_generators.globals import FRAMERATE, SAMPLE_WIDTH


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

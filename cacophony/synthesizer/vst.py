from typing import Optional, List, Callable
from pathlib import Path
import numpy as np
from cython_vst_loader.vst_host import VstHost
from cython_vst_loader.vst_plugin import VstPlugin
from cython_vst_loader.vst_event import VstNoteOnMidiEvent, VstNoteOffMidiEvent
from pydub import AudioSegment
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.callbacker.file_path import FilePath
from cacophony.callbacker.int_list import IntList, zero_127
from cacophony.callbacker.float_list import FloatList
from cacophony.music.globals import FRAMERATE, SAMPLE_WIDTH


class VST(Synthesizer):
    """
    A VST synthesizer.
    """

    def __init__(self, path: str = "", channel_index: int = 1, buffer_size_index: int = 2, beat_index: int = 5,
                 gain_index: int = 127, use_volume: bool = True, volume_index: int = 127):
        """
        :param path: The path to the VST .dll or .so file.
        :param beat_index: The index of the beat.
        :param gain_index: An index for gain values.
        :param use_volume: If True, use the value of `volume` for all new notes. If False, use the note's volume value.
        :param volume_index: An index for volume values.
        """

        self._path = ""
        self._plugin: Optional[VstPlugin] = None
        # Set the buffer size.
        self.buffer_size: IntList = IntList(values=[256, 512, 1024], index=buffer_size_index, tts="",
                                            callback=self._set_buffers)
        self._buffers: List[np.ndarray] = list()
        self._pointers: List[int] = list()
        self._host: VstHost = VstHost(FRAMERATE, self.buffer_size.get())
        # Set the channel.
        self.channel: IntList = zero_127(index=channel_index, tts="")
        # The path to the file. When this is set, we'll try loading the VST.
        self.path: FilePath = FilePath(suffixes=[".dll", ".so"], value=path, callback=self._load, tts="")
        # Try to load the VST plugin.
        self._load()
        # Remember the names of the parameters that we generated.
        self._generated_parameters: List[str] = list()
        # Should we use double precision?
        self._dtype = np.float32
        # How should we process the audio?
        self._process: Optional[Callable[[List[int], List[int], int], None]] = None
        super().__init__(beat_index=beat_index, gain_index=gain_index, use_volume=use_volume, volume_index=volume_index)

    def get_channels(self) -> int:
        return 2

    def get_help_text(self) -> str:
        text = "VST plugin. "
        if self._plugin is not None:
            text += Path(self.path.value).name
        return text

    def get(self, note: int, volume: int, duration: float) -> bytes:
        # Get the number of buffers.
        num = int(FRAMERATE / (self.buffer_size.get() * duration))
        # Get the audio buffer.
        buffer_size = self.buffer_size.get()
        arr = np.zeros(shape=(len(self._buffers), buffer_size, num), dtype=self._dtype)
        # Note on.
        event_note_on = VstNoteOnMidiEvent(3, note, volume, self.channel.get())
        for i in range(num):
            # Do the event.
            self._plugin.process_events([event_note_on])
            # Process the audio.
            self._process([], self._pointers, buffer_size)
            # Add the chunk.
            for j in range(len(self._buffers)):
                arr[j][i] = self._buffers[i]
        # Note off.
        self._plugin.process_events([VstNoteOffMidiEvent(3, note, self.channel.get())])
        # Overlay everything.
        q = np.int16(arr.reshape(arr.shape[0], arr.shape[1] * arr.shape[2]) * 32767)
        segment = AudioSegment(q[0], frame_rate=FRAMERATE, sample_width=SAMPLE_WIDTH, channels=1)
        for i in range(1, q.shape[0]):
            segment = segment.overlay(AudioSegment(q[i], frame_rate=FRAMERATE, sample_width=SAMPLE_WIDTH, channels=1))
        return segment.raw_data

    def _set_buffers(self) -> None:
        """
        Set the cached buffer size.
        """

        if self._plugin is None:
            return
        buffer_size = self.buffer_size.get()
        self._buffers.clear()
        self._pointers.clear()
        for i in range(self._plugin.get_num_output_channels()):
            self._buffers.append(np.zeros(buffer_size, dtype=self._dtype))
            self._pointers.append(VST._numpy_array_to_pointer(self._buffers[i]))

    def _load(self) -> None:
        """
        Load the VST plugin.
        """

        try:
            self._plugin = VstPlugin(self.path.value.encode('utf-8'), self._host)
        except Exception:
            return
        # We didn't load a new VST.
        if self._path == self.path.value:
            return
        # Enable/disable double precision.
        if self._plugin.allows_double_precision():
            self._dtype = np.float64
            self._process = self._plugin.process_replacing
        else:
            self._dtype = np.float32
            self._process = self._plugin.process_double_replacing
        # Set the buffers to use the correct data type.
        self._set_buffers()
        # Delete any existing parameters from a different VST.
        for generated_parameter in self._generated_parameters:
            delattr(self, generated_parameter)
        self._generated_parameters.clear()
        # Remember the path.
        self._path = self.path.value
        # Generate a default range of floats.
        arr = np.arange(0, 1.01, 0.01)
        arrf: List[float] = [float(f) for f in arr]
        # Generate attributes.
        for i in range(self._plugin.get_num_parameters()):
            # Get the parameter value.
            v = self._plugin.get_parameter_value(i)
            # Get the nearest value.
            # Source: https://www.geeksforgeeks.org/find-the-nearest-value-and-the-index-of-numpy-array/
            index = int(np.absolute(arr - v).argmin())
            fl: FloatList = FloatList(values=arrf[:],
                                      tts="",
                                      index=index,
                                      callback=self._set_parameter)
            # Set the kwargs.
            fl.set_kwargs({"index": i, "value": fl.get()})
            # Generate the attribute.
            setattr(self, self._plugin.get_parameter_name(i).decode("utf-8"), fl)

    def _set_parameter(self, index: int, value: float) -> None:
        """
        Set a VST plugin parameter.
        """

        self._plugin.set_parameter_value(index, value)

    @staticmethod
    def _numpy_array_to_pointer(arr: np.ndarray) -> int:
        """
        :return: The numpy array pointer.
        """

        if arr.ndim != 1:
            raise Exception('expected a 1d numpy array here')
        pointer, _ = arr.__array_interface__['data']
        return pointer

from io import BytesIO
import librosa
from time import sleep
from pathlib import Path
from pydub import AudioSegment
import wave
import numpy as np
import pygame.mixer
import pygame.midi
import scipy.fftpack as syfp
from scipy.io.wavfile import write


def karplus_strong(wavetable, n_samples, div):
    # https://www.math.drexel.edu/~dp399/musicmath/Karplus-Strong.html
    """Synthesizes a new waveform from an existing wavetable, modifies last sample by averaging."""
    samples = []
    current_sample = 0
    previous_value = 0
    size = wavetable.size // div
    while len(samples) < n_samples:
        wavetable[current_sample] = 0.5 * (wavetable[current_sample] + previous_value)
        samples.append(wavetable[current_sample])
        previous_value = samples[-1]
        current_sample += 1
        current_sample = current_sample % size
    return np.array(samples)


frequency = 24
a = np.float64(AudioSegment.from_file("impact.wav").get_array_of_samples())
b = karplus_strong(a, 44100 * 4, frequency)
fade_curve = np.linspace(1.0, 0.0, b.shape[0])
#b = b * fade_curve
b = b.astype(np.int16).tobytes()
a = AudioSegment.from_file("impact.wav").raw_data
wavetable = (2 * np.random.randint(0, 2, 44100 // 70) - 1).astype(np.float32)
c = karplus_strong(wavetable, 44100 * 4, 1) * 0.25
c = np.int16(c * 32767).tobytes()
pygame.mixer.init(allowedchanges=pygame.AUDIO_ALLOW_CHANNELS_CHANGE)
for audio in [a, b, c]:
    sound = pygame.mixer.Sound(audio)
    sound.play()
    sleep(sound.get_length())
    sleep(1)

exit()
write("karplus.wav", 44100, b)
y, sr = librosa.load("karplus.wav", sr=44100)
pitches, magnitudes = librosa.core.piptrack(y=y, sr=sr, ref=np.mean)
# https://stackoverflow.com/a/72586943
max_indexes = np.argmax(magnitudes, axis=0)
pitches = pitches[max_indexes, range(magnitudes.shape[1])]
pygame.midi.init()
m = pygame.midi.Output(pygame.midi.get_default_output_id())
m.set_instrument(0)
m.note_on(pygame.midi.frequency_to_midi(pitches.mean()), 127)
sleep(1)

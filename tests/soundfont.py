from time import sleep
import pygame.mixer
from cacophony.soundfont import SoundFont


s = SoundFont()
s.path.set("D:/SoundFonts/ms_basic.sf3")
print(s.instruments)
audio = bytearray()
for note in ["E2", "G2", "E2"]:
    a = s.get(bank=0, instrument=57, note=note, volume=127, beat="1", bpm=110, decay="0")
    audio.extend(a)
pygame.mixer.init(allowedchanges=pygame.AUDIO_ALLOW_CHANNELS_CHANGE)
sound = pygame.mixer.Sound(audio)
sound.play()
sleep(sound.get_length())

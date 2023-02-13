from time import sleep
import pygame.mixer
from cacophony.clatter import Clatter


c = Clatter()
a = c.get(primary_impact_material="ceramic", primary_size=2, primary_mass=0.1, primary_amp=0.2, primary_resonance=0.2,
          secondary_impact_material="stone", secondary_size=4, secondary_mass=100, secondary_amp=0.1, secondary_resonance=0.1,
          speed=1.2, scrape_material=None, duration=None, random_seed=None)
pygame.mixer.init(allowedchanges=pygame.AUDIO_ALLOW_CHANNELS_CHANGE)
sound = pygame.mixer.Sound(a)
sound.play()
sleep(sound.get_length())

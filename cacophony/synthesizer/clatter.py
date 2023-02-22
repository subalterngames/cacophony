from __future__ import annotations
from itertools import permutations
from typing import List
from pathlib import Path
import clr
from requests import get
from bs4 import BeautifulSoup
import numpy as np
from cacophony.music.note import Note
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.paths import USER_DIRECTORY
from cacophony.callbacker.value import Value


def __download_clatter() -> None:
    """
    Download Clatter.Core.dll if it doesn't already exist.
    """

    resp = get("https://alters-mit.github.io/clatter/").text
    soup = BeautifulSoup(resp, "html.parser")
    for link in soup.find_all('a', href=True):
        if "Clatter.Core.dll" in link['href']:
            resp = get(link['href'])
            if resp.status_code == 200:
                _CLATTER_PATH.write_bytes(resp.content)
            return


_CLATTER_PATH: Path = USER_DIRECTORY.joinpath("clatter/Clatter.Core.dll").resolve()
if not _CLATTER_PATH.parent.exists():
    _CLATTER_PATH.parent.mkdir(parents=True)
if not _CLATTER_PATH.exists():
    __download_clatter()
# Import Clatter.
clr.AddReference(str(_CLATTER_PATH))
from System import Random, Enum
from Clatter.Core import ImpactMaterialData, ImpactMaterial, ImpactMaterialUnsized, ClatterObjectData, Impact, ScrapeMaterial, ScrapeMaterialData, Scrape


class Clatter(Synthesizer):
    """
    A [Clatter](https://alters-mit.github.io/clatter/) impact audio synthesizer.

    - Each note value sets the primary and secondary impact materials.
    - The mass is derived from the "size bucket" of the impact material.
    - The volume value sets the primary amp, primary resonance, secondary amp, and secondary resonance values.
    - The note beat duration sets the speed.
    """

    __IMPACT_MATERIALS: List[ImpactMaterial] = Enum.GetValues(ImpactMaterial)
    __MASSES: List[float] = [0.123, 0.261, 1.305, 12.008, 81.325, 183.7]
    __AMPS_AND_RESONANCES: List[tuple] = list(permutations([round(__a, 1) for __a in np.arange(0, 1, step=0.2)], 4))
    __MAX_SPEED: float = 5

    def __init__(self, has_seed: bool = False, seed: int = 0, beat_index: int = 5, gain_index: int = 127, use_volume: bool = True, volume_index: int = 127):
        """
        :param has_seed: If True, Clatter will use the `seed` value. If False, the seed is random per sound.
        :param seed: A user-defined random seed.
        :param beat_index: The index of the beat.
        :param gain_index: An index for gain values.
        :param use_volume: If True, use the value of `volume` for all new notes. If False, use the note's volume value.
        :param volume_index: An index for volume values.
        """

        self.has_seed: Value[bool] = Value(value=has_seed,
                                           tts="Set whether the same random seed is used every note or whether each sound uses a new random seed.")
        self.seed: Value[int] = Value(value=seed,
                                      tts="Set the random seed. This is ignored if the previous widget is not selected.")
        super().__init__(beat_index=beat_index, gain_index=gain_index, use_volume=use_volume, volume_index=volume_index)

    def get_channels(self) -> int:
        return 1

    def get_help_text(self) -> str:
        return "Clatter."

    def _audio(self, note: Note, volume: int, duration: float) -> bytes:
        # Get the impact materials.
        n: int = note.note % len(Clatter.__IMPACT_MATERIALS)
        primary_impact_material: ImpactMaterial = Clatter.__IMPACT_MATERIALS[n]
        secondary_impact_material: ImpactMaterial = Clatter.__IMPACT_MATERIALS[(len(Clatter.__IMPACT_MATERIALS) - n) % len(Clatter.__IMPACT_MATERIALS)]
        # Load the impact materials.
        ImpactMaterialData.Load(primary_impact_material)
        ImpactMaterialData.Load(secondary_impact_material)
        # Parse the material to get the size.
        primary_size: int = int(primary_impact_material.ToString()[-1])
        secondary_size: int = int(secondary_impact_material.ToString()[-1])
        # Get the masses.
        primary_mass: float = Clatter.__MASSES[primary_size]
        secondary_mass: float = Clatter.__MASSES[secondary_size]
        # Get the amps and resonances.
        ar = Clatter.__AMPS_AND_RESONANCES[volume % len(Clatter.__AMPS_AND_RESONANCES)]
        primary_amp: float = ar[0] + 0.1
        primary_resonance: float = ar[1]
        secondary_amp: float = ar[2] + 0.1
        secondary_resonance: float = ar[3]
        # Get the speed.
        speed: float = note.duration % Clatter.__MAX_SPEED
        # Get the objects.
        primary = ClatterObjectData(0, primary_impact_material, primary_amp, primary_resonance, primary_mass)
        secondary = ClatterObjectData(1, secondary_impact_material, secondary_amp, secondary_resonance, secondary_mass)
        # Generate audio.
        if self.has_seed.value:
            rng = Random(self.seed.value)
        else:
            rng = Random()
        impact = Impact(primary, secondary, rng)
        impact.GetAudio(speed)
        return bytes(impact.samples.ToInt16Bytes())

from itertools import permutations
from typing import Dict, List
from pathlib import Path
import clr
from pkg_resources import resource_filename
from requests import get
from bs4 import BeautifulSoup
import numpy as np
from cacophony.music.note import Note
from cacophony.synthesizer.synthesizer import Synthesizer


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


_CLATTER_PATH: Path = Path(resource_filename(__name__, "Clatter.Core.dll")).absolute()
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
    IMPACT_MATERIALS: Dict[str, ImpactMaterial] = {k.ToString(): k for k in Enum.GetValues(ImpactMaterial)}
    SCRAPE_MATERIALS: Dict[str, ScrapeMaterial] = {k.ToString(): k for k in Enum.GetValues(ScrapeMaterial)}

    def get_channels(self) -> int:
        return 1

    def _audio(self, note: Note, duration: float) -> bytes:
        # Get the impact materials.
        n: int = note.note % len(Clatter.__IMPACT_MATERIALS)
        primary_impact_material: ImpactMaterial = Clatter.__IMPACT_MATERIALS[n]
        secondary_impact_material: ImpactMaterial = Clatter.__IMPACT_MATERIALS[len(Clatter.__IMPACT_MATERIALS) - n]
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
        ar = Clatter.__AMPS_AND_RESONANCES[note.volume % len(Clatter.__AMPS_AND_RESONANCES)]
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
        impact = Impact(primary, secondary, Random())
        impact.GetAudio(speed)
        return bytes(impact.samples.ToInt16Bytes())

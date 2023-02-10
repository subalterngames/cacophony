from typing import Dict, List, Optional
from pathlib import Path
import clr
from pkg_resources import resource_filename
from requests import get
from bs4 import BeautifulSoup
from cacophony.waveform_generator import WaveformGenerator
from cacophony.options import zero1, rangef


def __download_clatter() -> None:
    resp = get("https://alters-mit.github.io/clatter/").text
    soup = BeautifulSoup(resp, "html.parser")
    for link in soup.find_all('a', href=True):
        if "Clatter.Core.dll" in link['href']:
            resp = get(link['href'])
            if resp.status_code == 200:
                CLATTER_PATH.write_bytes(resp.content)
            return


CLATTER_PATH: Path = Path(resource_filename(__name__, "Clatter.Core.dll")).absolute()
if not CLATTER_PATH.exists():
    __download_clatter()
# Import Clatter.
clr.AddReference(str(CLATTER_PATH))
from System import Random
from Clatter.Core import ImpactMaterialData, ImpactMaterialUnsized, ClatterObjectData, Impact, ScrapeMaterial, ScrapeMaterialData, Scrape


class Clatter(WaveformGenerator):
    __IMPACT_MATERIALS: Dict[str, ImpactMaterialUnsized] = {"ceramic": ImpactMaterialUnsized.ceramic,
                                                            "glass": ImpactMaterialUnsized.glass,
                                                            "metal": ImpactMaterialUnsized.metal,
                                                            "wood_hard": ImpactMaterialUnsized.wood_hard,
                                                            "wood_medium": ImpactMaterialUnsized.wood_medium,
                                                            "wood_soft": ImpactMaterialUnsized.wood_soft,
                                                            "cardboard": ImpactMaterialUnsized.cardboard,
                                                            "paper": ImpactMaterialUnsized.paper,
                                                            "plastic_hard": ImpactMaterialUnsized.plastic_hard,
                                                            "plastic_soft_foam": ImpactMaterialUnsized.plastic_soft_foam,
                                                            "rubber": ImpactMaterialUnsized.rubber,
                                                            "fabric": ImpactMaterialUnsized.fabric,
                                                            "leather": ImpactMaterialUnsized.leather,
                                                            "stone": ImpactMaterialUnsized.stone}
    __SCRAPE_MATERIALS: Dict[str, ScrapeMaterial] = {"plywood": ScrapeMaterial.plywood,
                                                     "ceramic": ScrapeMaterial.ceramic,
                                                     "pvc": ScrapeMaterial.pvc,
                                                     "rough_wood": ScrapeMaterial.rough_wood,
                                                     "acrylic": ScrapeMaterial.acrylic,
                                                     "sanded_acrylic": ScrapeMaterial.sanded_acrylic,
                                                     "vinyl": ScrapeMaterial.vinyl,
                                                     "poplar_wood": ScrapeMaterial.poplar_wood,
                                                     "bass_wood": ScrapeMaterial.bass_wood,
                                                     "polycarbonate": ScrapeMaterial.polycarbonate,
                                                     "polyethylene": ScrapeMaterial.polyethylene,
                                                     "sandpaper": ScrapeMaterial.sandpaper}

    def __init__(self):
        self.primary_impact_materials: List[str] = list(Clatter.__IMPACT_MATERIALS.keys())
        self.primary_sizes: List[int] = list(range(6))
        self.primary_masses: List[float] = rangef(start=0.1, end=200, step=0.1)
        self.primary_amps: List[float] = zero1()
        self.primary_resonances: List[float] = zero1()
        self.secondary_impact_materials: List[str] = self.primary_impact_materials[:]
        self.secondary_sizes: List[int] = self.primary_sizes[:]
        self.secondary_masses: List[float] = self.primary_masses[:]
        self.secondary_amps: List[float] = self.primary_amps[:]
        self.secondary_resonances: List[float] = self.primary_resonances[:]
        self.scrape_materials: List[str] = list(Clatter.__SCRAPE_MATERIALS.keys())
        self.speeds: List[float] = rangef(start=0.1, end=5, step=0.1)
        self.durations: List[float] = rangef(start=0.1, end=10, step=0.1)

    def get(self, primary_impact_material: str, primary_size: int, primary_mass: float, primary_amp: float, primary_resonance: float,
            secondary_impact_material: str, secondary_size: int, secondary_mass: float, secondary_amp: float, secondary_resonance: float,
            speed: float, scrape_material: Optional[str], duration: Optional[float], random_seed: Optional[int]) -> bytes:
        # Get the primary object.
        pm = ImpactMaterialData.GetImpactMaterial(Clatter.__IMPACT_MATERIALS[primary_impact_material], primary_size)
        ImpactMaterialData.Load(pm)
        primary = ClatterObjectData(0, pm, primary_amp, primary_resonance, primary_mass)
        # Load the scrape material.
        if scrape_material is not None:
            scrape_mat: ScrapeMaterial = Clatter.__SCRAPE_MATERIALS[scrape_material]
            ScrapeMaterialData.Load(scrape_mat)
            is_scrape = True
        else:
            is_scrape = False
            scrape_mat = None
        # Get the secondary object.
        sm = ImpactMaterialData.GetImpactMaterial(Clatter.__IMPACT_MATERIALS[secondary_impact_material], secondary_size)
        ImpactMaterialData.Load(sm)
        secondary = ClatterObjectData(1, sm,  secondary_amp, secondary_resonance, secondary_mass, scrape_mat)
        # Load the random number generator.
        if random_seed is None:
            rng = Random()
        else:
            rng = Random(random_seed)
        # Generate scrape audio.
        if is_scrape:
            num_events = Scrape.GetNumScrapeEvents(duration)
            scrape_audio: bytearray = bytearray()
            scrape = Scrape(scrape_mat, primary, secondary, rng)
            for i in range(num_events):
                scrape.GetAudio(speed)
                scrape_audio.extend(bytes(scrape.samples.ToInt16Bytes()))
            return bytes(scrape_audio)
        # Generate impact audio.
        else:
            impact = Impact(primary, secondary, rng)
            impact.GetAudio(speed)
            return bytes(impact.samples.ToInt16Bytes())

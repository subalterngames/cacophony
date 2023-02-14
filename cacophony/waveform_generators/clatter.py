from typing import Dict, List, Optional
from pathlib import Path
import clr
from pkg_resources import resource_filename
from requests import get
from bs4 import BeautifulSoup
from cacophony.waveform_generators.waveform_generator import WaveformGenerator
from cacophony.waveform_generators.waveform_generator_type import WaveformGeneratorType
from cacophony.waveform_generators.options import zero1, rangef


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
        """
        (no parameters)
        """

        """:field
        A list of possible impact materials for the primary object.
        """
        self.primary_impact_material: List[str] = list(Clatter.__IMPACT_MATERIALS.keys())
        """:field
        A list of possible sizes (0-5) for the primary object.
        """
        self.primary_size: List[int] = list(range(6))
        """:field
        A list of possible mass values for the primary object.
        """
        self.primary_mass: List[float] = rangef(start=0.1, end=100, step=0.1)
        """:field
        A list of possible amp values (0-1) for the primary object.
        """
        self.primary_amp: List[float] = zero1()
        """:field
        A list of possible resonance values (0-1) for the primary object.
        """
        self.primary_resonance: List[float] = zero1()
        """:field
        A list of possible impact materials for the secondary object.
        """
        self.secondary_impact_material: List[str] = self.primary_impact_material[:]
        """:field
        A list of possible sizes (0-5) for the secondary object.
        """
        self.secondary_size: List[int] = self.primary_size[:]
        """:field
        A list of possible mass values for the secondary object.
        """
        self.secondary_mass: List[float] = self.primary_mass[:]
        """:field
        A list of possible amp values (0-1) for the secondary object.
        """
        self.secondary_amp: List[float] = self.primary_amp[:]
        """:field
        A list of possible resonance values (0-1) for the secondary object.
        """
        self.secondary_resonance: List[float] = self.primary_resonance[:]
        """:field
        A list of possible scrape materials.
        """
        self.scrape_material: List[str] = list(Clatter.__SCRAPE_MATERIALS.keys())
        """:field
        A list of possible speeds.
        """
        self.speed: List[float] = rangef(start=0.1, end=5, step=0.1)
        """:field
        A list of possible durations.
        """
        self.duration: List[float] = rangef(start=0.1, end=10, step=0.1)

    def get(self, primary_impact_material: str, primary_size: int, primary_mass: float, primary_amp: float, primary_resonance: float,
            secondary_impact_material: str, secondary_size: int, secondary_mass: float, secondary_amp: float, secondary_resonance: float,
            speed: float, scrape_material: Optional[str], duration: Optional[float], random_seed: Optional[int]) -> bytes:
        """
        :param primary_impact_material: The primary object's impact material.
        :param primary_size: The primary object's size (0-5).
        :param primary_mass: The primary object's mass.
        :param primary_amp: The primary object's amp (0-1).
        :param primary_resonance: The primary object's resonance (0-1).
        :param secondary_impact_material: The secondary object's impact material.
        :param secondary_size: The secondary object's size (0-5).
        :param secondary_mass: The secondary object's mass.
        :param secondary_amp: The secondary object's amp (0-1).
        :param secondary_resonance: The secondary object's resonance (0-1).
        :param speed: The speed of the collision.
        :param scrape_material: The scrape material. If None, this is an impact.
        :param duration: The duration of the scrape. Ignored if scrape_material == None.
        :param random_seed: The random seed. If None, the seed is random.

        :return: A waveform bytestring.
        """

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

    def get_type(self) -> WaveformGeneratorType:
        return WaveformGeneratorType.wav_creator

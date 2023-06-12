from pathlib import Path
from PIL import Image
import numpy as np

icon = bytearray()
source: Image.Image = Image.open(str(Path.home().joinpath("cacophony/icon/icon.png").resolve()))
for size in [64, 32, 16]:
    icon.extend(np.array(source.resize((size, size), Image.Resampling.LANCZOS)).tobytes())
Path.home().joinpath("cacophony/data/icon").resolve().write_bytes(icon)

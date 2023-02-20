from pathlib import Path
from pkg_resources import resource_filename


USER_DIRECTORY: Path = Path.home().joinpath("cacophony")
DATA_DIRECTORY: Path = Path(resource_filename(__name__, "data"))
TEMP_DIRECTORY: Path = USER_DIRECTORY.joinpath("temp")

from typing import List, Callable
import sys
import importlib
import inspect
from pkg_resources import resource_filename
from pkgutil import iter_modules
from cacophony.paths import USER_DIRECTORY
from cacophony.synthesizer.synthesizer import Synthesizer


def get_synthesizers() -> List[Callable]:
    """
    :return: Constructor calls. A list of predefined synthesizers plus all user-defined synthesizers.
    """

    synths: List[Callable] = []
    cacophony_synths_dir = resource_filename(__name__, "")
    # Source: https://julienharbulot.com/python-dynamical-import.html
    for _, module_name, _ in iter_modules([str(cacophony_synths_dir)]):
        # Source: https://stackoverflow.com/a/55067404
        for name, cls in inspect.getmembers(importlib.import_module(f"cacophony.synthesizer.{module_name}"),
                                            inspect.isclass):
            # This is a synthesizer.
            if Synthesizer in cls.__bases__:
                synths.append(cls)
    # Get user-defined synthesizers.
    user_synths_dir = USER_DIRECTORY.joinpath("synthesizers").resolve()
    sys.path.insert(1, str(user_synths_dir))
    for f in user_synths_dir.iterdir():
        if f.is_file() and f.suffix == ".py":
            # Source: https://stackoverflow.com/a/55067404
            for name, cls in inspect.getmembers(importlib.import_module(f.stem), inspect.isclass):
                # This class was defined in this file.
                if cls.__module__ == f.stem:
                    # This is a synthesizer.
                    if Synthesizer in cls.__bases__:
                        synths.append(cls)
    return synths

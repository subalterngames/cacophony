from pathlib import Path
from typing import Union


def get_path(path: Union[str, Path]) -> Path:
    """
    :param path: A path as either a string or a `Path`.

    :return: The path as a `Path`.
    """

    if isinstance(path, str):
        return Path(path)
    elif isinstance(path, Path):
        return path
    else:
        raise Exception(path)


def get_string_path(path: Union[str, Path]) -> str:
    """
    :param path: A path as either a string or a `Path`.

    :return: The path as a string.
    """

    if isinstance(path, str):
        p = path
    elif isinstance(path, Path):
        p = str(path.resolve())
    else:
        raise Exception(path)
    return p.replace("\\", "/")

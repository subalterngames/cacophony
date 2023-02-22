from pathlib import Path
from typing import Union, List
from configparser import ConfigParser
from cacophony.render.input_key import InputKey
from cacophony.paths import USER_DIRECTORY, DATA_DIRECTORY


def get_config() -> ConfigParser:
    """
    :return: The config parser.
    """

    parser = ConfigParser()
    local_config_path = USER_DIRECTORY.joinpath("config.init")
    # Read a user-defined config file.
    if local_config_path.exists():
        parser.read(str(local_config_path))
    # Read the default config file.
    else:
        parser.read(str(DATA_DIRECTORY.joinpath("config.ini")))
    return parser


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


def tooltip(keys: List[InputKey], predicate: str, boop: str = "or") -> str:
    """
    A convenient tooltip wrapper.

    :param keys: The input keys.
    :param predicate: The predicate i.e. what the keys will do.
    :param boop: Boolean operator.

    :return: Te tooltip text.
    """

    from cacophony.render.globals import INPUT_KEYS
    inputs = []
    for key in keys:
        for v in INPUT_KEYS[key]:
            if isinstance(v, str):
                inputs.append(v)
            elif isinstance(v, tuple):
                # Midi input.
                if isinstance(v[0], int):
                    inputs.append(f"MIDI-in {str(v)}")
                elif isinstance(v[0], str):
                    inputs.append(" and ".join(v))
    text = f" {boop} ".join(inputs) + f" to {predicate}"
    if text[-1] not in "?!.":
        text += "."
    return text


def note_on(midi_event: list) -> bool:
    """
    :param midi_event: The MIDI event.

    :return: True if this is a note-on event.
    """

    return 144 <= midi_event[0] <= 159


def note_off(midi_event: list) -> bool:
    """
    :param midi_event: The MIDI event.

    :return: True if this is a note-off event.
    """

    return 128 <= midi_event[0] <= 143


def get_duration(bpm: int, beat: float) -> float:
    """
    :param bpm: The beats per minute.
    :param beat: The duration in terms of beats.

    :return: The duration in terms of seconds.
    """

    return 60.0 / bpm * beat

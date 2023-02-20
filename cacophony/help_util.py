from typing import List
from cacophony.render.input_key import InputKey
from cacophony.render.globals import INPUT_KEYS


def tooltip(keys: List[InputKey], predicate: str, boop: str = "or") -> str:
    """
    A convenient tooltip wrapper.

    :param keys: The input keys.
    :param predicate: The predicate i.e. what the keys will do.
    :param boop: Boolean operator.

    :return: Te tooltip text.
    """

    inputs = []
    for key in keys:
        for v in INPUT_KEYS[key]:
            if isinstance(v, str):
                inputs.append(v)
            elif isinstance(v, tuple):
                inputs.append(f"MIDI-in {str(v)}")
    text = f" {boop} ".join(inputs) + f" to {predicate}"
    if text[-1] not in "?!.":
        text += "."
    return text

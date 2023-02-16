from enum import IntEnum


class ChiptunePCM(IntEnum):
    """
    Chiptune PCM data type.
    """

    sine = 1
    triangle = 2
    saw = 4
    pulse = 8
    noise = 16

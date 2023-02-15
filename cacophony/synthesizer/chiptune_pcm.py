from enum import Enum


class ChiptunePCM(Enum):
    """
    Chiptune PCM data type.
    """

    sine = 1
    triangle = 2
    saw = 4
    pulse = 8
    noise = 16

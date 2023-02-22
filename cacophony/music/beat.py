from typing import Dict
from enum import Enum
from fractions import Fraction


class Beat(Enum):
    """
    Enum value for fractions of a beat.
    """

    eighth = 1 / 8
    sixth = 1 / 6
    fourth = 1 / 4
    third = 1 / 3
    half = 1 / 2
    one = 1
    one_and_half = 1.5
    two = 2
    three = 3
    four = 4
    five = 5
    six = 6


FRACTION_TO_BEAT: Dict[str, Beat] = {str(Fraction(__beat.value)): __beat for __beat in Beat}

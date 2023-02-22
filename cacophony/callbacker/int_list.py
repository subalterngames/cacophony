from typing import List
import numpy as np
from cacophony.callbacker.indexed_list import IndexedList


class IntList(IndexedList[int]):
    """
    An indexed list of integer values.
    """

    pass


def zero_127(index: int = 0) -> IntList:
    """
    :param index: The index.

    :return: An indexed list of integers from 0 to 127.
    """

    return IntList(values=list(range(128)), index=index)

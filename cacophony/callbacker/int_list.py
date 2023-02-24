from typing import Callable
from cacophony.callbacker.indexed_list import IndexedList


class IntList(IndexedList[int]):
    """
    An indexed list of integer values.
    """

    pass


def zero_127(tts: str, index: int = 0, callback: Callable = None, kwargs: dict = None) -> IntList:
    """
    :param tts: Text-to-speech text.
    :param index: The index.
    :param callback: An optional callback that is invoked when `index` is set.
    :param kwargs: An optional dictionary of keyword arguments.

    :return: An indexed list of integers from 0 to 127.
    """

    return IntList(values=list(range(128)), tts=tts, index=index, callback=callback, kwargs=kwargs)

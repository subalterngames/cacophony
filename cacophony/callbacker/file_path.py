from typing import Callable, List
from cacophony.callbacker.value import Value


class FilePath(Value[str]):
    """
    A nothing-wrapper for a string value to indicate that this will be used to open a file.
    """

    def __init__(self, suffixes: List[str], value: str, tts: str, callback: Callable = None, kwargs: dict = None):
        """
        :param suffixes: A list of file suffixes.
        :param value: The initial value.
        :param tts: Text-to-speech text.
        :param callback: An optional callback that is invoked when `value` is set.
        :param kwargs: An optional dictionary of keyword arguments.
        """

        super().__init__(callback=callback, value=value, tts=tts, kwargs=kwargs)
        self.suffixes: List[str] = suffixes

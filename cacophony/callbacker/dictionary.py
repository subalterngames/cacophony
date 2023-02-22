from __future__ import annotations
from typing import List, Callable, Dict
from overrides import final
from cacophony.callbacker.int_list import IntList


class Dictionary(IntList):
    """
    A list of key-value pairs values with an index that wraps around the collection. The values are always strings.
    """

    def __init__(self, values: Dict[int, str], tts: str, index: int = 0, callback: Callable[[], None] = None):
        """
        :param values: A list of int values.
        :param tts: Text-to-speech text.
        :param index: The current index.
        :param callback: An optional callback that is invoked when `index` is set.
        """

        super().__init__(values=list(values.keys()), tts=tts, index=index, callback=callback)
        self._dict_values: List[str] = list(values.values())

    @final
    def get_str(self) -> str:
        return self._dict_values[self.index]

    @final
    def get_strs(self) -> List[str]:
        return [self._dict_values[i] for i in range(len(self.values))]
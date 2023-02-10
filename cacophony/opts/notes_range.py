from typing import Callable
from cacophony.int_range import IntRange
from cacophony.globals import NOTES


class NotesRange(IntRange):
    def __init__(self, index: int = 0, callback: Callable[[int], None] = None):
        super().__init__(start=0, length=len(NOTES), index=index, callback=callback, to_str=self.__to_str)

    def __to_str(self) -> str:
        return NOTES[self.get()]


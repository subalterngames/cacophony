from typing import List, Callable, Optional
from cacophony.render.panel.panel_type import PanelType


class OpenFileState:
    """
    The state of the open file panel.
    """

    def __init__(self, suffixes: List[str], previous_focus: PanelType, callback: Callable[[str], None] = None):
        """
        :param suffixes: A list of file suffixes that we're searching for.
        :param previous_focus: The panel that previously have focus.
        :param callback: A callback when the path is set.
        """

        self.suffixes: List[str] = suffixes
        self.previous_focus: PanelType = previous_focus
        self.callback: Optional[Callable[[str], None]] = callback

from typing import List, Callable, Optional
from cacophony.render.panel.panel_type import PanelType


class OpenFileState:
    """
    The state of the open file panel.
    """

    def __init__(self, suffixes: List[str] = None,
                 previous_focus: PanelType = PanelType.main_menu,
                 previous_active: List[PanelType] = None,
                 callback: Callable[[str], None] = None):
        """
        :param suffixes: A list of file suffixes that we're searching for.
        :param previous_focus: The panel that previously had focus.
        :param previous_active: The panels that were previously active.
        :param callback: A callback when the path is set.
        """

        if suffixes is None:
            self.suffixes: List[str] = list()
        else:
            self.suffixes = suffixes
        self.previous_focus: PanelType = previous_focus
        if previous_active is None:
            self.previous_active: List[PanelType] = list()
        else:
            self.previous_active = previous_active
        self.callback: Optional[Callable[[str], None]] = callback

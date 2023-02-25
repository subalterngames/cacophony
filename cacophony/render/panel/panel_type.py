from enum import IntEnum


class PanelType(IntEnum):
    """
    Enum values describing the type of panel.

    We'll use this to easily reference one panel from another without having to pass the object itself.
    """

    undefined = -1
    main_menu = 0
    tracks_list = 1
    piano_roll = 2
    synthesizer_panel = 3

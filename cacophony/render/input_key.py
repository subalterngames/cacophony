from enum import IntFlag


class InputKey(IntFlag):
    """
    Enum values for input keys.
    """

    up = 1
    right = 2
    down = 4
    left = 8
    next_panel = 16
    previous_panel = 32
    select = 64
    play = 128
    panel_help = 256
    widget_help = 512
    app_help = 1024

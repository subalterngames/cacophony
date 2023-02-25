from enum import IntFlag


class InputKey(IntFlag):
    """
    Enum values for input keys.
    """

    up = 0
    right = 1
    down = 2
    left = 3
    next_panel = 4
    previous_panel = 5
    select = 6
    play = 7
    panel_help = 8
    widget_help = 9
    app_help = 10
    undo = 11
    cancel = 12
    open_file = 13
    stop_tts = 14
    new_file = 15
    save_file = 16
    quit_program = 17
    add = 18
    subtract = 19

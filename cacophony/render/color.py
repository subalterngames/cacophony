from enum import IntEnum


class Color(IntEnum):
    border_no_focus = 0
    border_focus = 1
    parameter_key = 2
    parameter_value = 3
    panel_title_no_focus = 4
    panel_title_focus = 5
    panel_background = 6
    parameter_value_background_focus = 7
    note_panel_focus = 8
    note_panel_no_focus = 9
    note_panel_selected_focus = 10
    note_panel_selected_no_focus = 11
    piano_roll_note_line_focus = 12
    piano_roll_note_line_no_focus = 13
    piano_roll_note_name_focus = 14
    piano_roll_note_name_no_focus = 15
    parameter_boolean_true = 16
    parameter_boolean_false = 17
    start_time = 18
    end_time = 19

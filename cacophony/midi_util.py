def note_on(midi_event: list) -> bool:
    """
    :param midi_event: The MIDI event.

    :return: True if this is a note-on event.
    """

    return 144 <= midi_event[0] <= 159


def note_off(midi_event: list) -> bool:
    """
    :param midi_event: The MIDI event.

    :return: True if this is a note-off event.
    """

    return 128 <= midi_event[0] <= 143

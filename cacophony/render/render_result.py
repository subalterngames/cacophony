from typing import List
from cacophony.render.input_key import InputKey
from cacophony.render.globals import INPUTS


class RenderResult:
    """
    The user input result after a render.

    `self.pressed`, `self.held`, and `self.midi` are the "raw" inputs.

    `self.inputs_pressed` and `self.inputs_held` contain the converted `InputKey` values.
    """

    def __init__(self, pressed: List[str], held: List[str], midi: List[List[int]]):
        """
        :param pressed: A list of keys pressed on this frame.
        :param held: A list of keys held on this frame.
        :param midi: A list of midi events on this frame (without the timestamp).
        """

        self.pressed: List[str] = pressed
        self.held: List[str] = held
        self.midi: List[List[int]] = midi
        self.inputs_pressed: List[InputKey] = list()
        self.inputs_held: List[InputKey] = list()
        for k in INPUTS:
            if isinstance(k, str):
                if k in self.pressed:
                    self.inputs_pressed.append(INPUTS[k])
                if k in self.held:
                    self.inputs_held.append(INPUTS[k])
            # Handle multi-key input.
            elif isinstance(k, tuple):
                pressed = True
                held = True
                for kk in k:
                    if kk not in self.pressed:
                        pressed = False
                    if kk not in self.held:
                        held = False
                if pressed:
                    self.inputs_pressed.append(INPUTS[k])
                if held:
                    self.inputs_held.append(INPUTS[k])
        for m in self.midi:
            k = (m[0], m[1], m[2])
            if k in INPUTS:
                self.inputs_pressed.append(INPUTS[k])

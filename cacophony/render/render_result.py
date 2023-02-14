from typing import List
from cacophony.render.input_key import InputKey
from cacophony.render.globals import INPUTS


class RenderResult:
    def __init__(self, pressed: List[str], held: List[str], midi: List[List[int]]):
        self.pressed: List[str] = pressed
        self.held: List[str] = held
        self.midi: List[List[int]] = midi
        self.inputs_pressed: List[InputKey] = list()
        self.inputs_held: List[InputKey] = list()
        for k in self.pressed:
            if k in INPUTS:
                self.inputs_pressed.append(INPUTS[k])
        for k in self.held:
            if k in INPUTS:
                self.inputs_held.append(INPUTS[k])
        for m in self.midi:
            k = (m[0], m[1], m[2])
            if k in INPUTS:
                self.inputs_pressed.append(INPUTS[k])

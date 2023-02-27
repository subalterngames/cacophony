from time import time
from typing import List
from cacophony.render.input_key import InputKey
from cacophony.render.globals import INPUTS, SCROLL_DT


class RenderResult:
    """
    The user input result after a render.

    `self.pressed`, `self.held`, and `self.midi` are the "raw" inputs.

    `self.inputs_pressed` and `self.inputs_held` contain the converted `InputKey` values.
    """

    _SCROLL_T0: float = 0
    _SCROLL_INPUTS: List[InputKey] = [InputKey.up, InputKey.down, InputKey.left, InputKey.right]

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
            # MIDI input.
            if k in INPUTS:
                self.inputs_pressed.append(INPUTS[k])
        self.inputs_scroll: List[InputKey] = list()
        for ik in RenderResult._SCROLL_INPUTS:
            if ik in self.inputs_pressed:
                self.inputs_scroll.append(ik)
        # Reset the scroll time.
        if len(self.inputs_scroll) > 0:
            RenderResult._SCROLL_T0 = time()
        # Time to check the held scroll keys.
        elif time() - RenderResult._SCROLL_T0 > SCROLL_DT:
            for ik in RenderResult._SCROLL_INPUTS:
                if ik in self.inputs_held:
                    self.inputs_scroll.append(ik)
        # Reset the scroll time.
        if len(self.inputs_scroll) > 0:
            RenderResult._SCROLL_T0 = time()

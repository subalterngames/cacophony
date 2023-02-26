from typing import List, Tuple, Optional
from overrides import final
import pygame
import pygame.mixer
import pygame.midi
from pygame import Surface, Rect
from cacophony.render.commands.command import Command
from cacophony.render.globals import WINDOW_PIXEL_WIDTH, WINDOW_PIXEL_HEIGHT, MIDI_INPUT_DEVICE_ID
from cacophony.render.render_result import RenderResult
from cacophony.render.input_key import InputKey


class Renderer:
    """
    Blit surfaces to the display surface.
    """

    _INITIALIZED: bool = False
    _MIDI_IN: Optional[pygame.midi.Input] = None

    def __init__(self):
        """
        (no parameters)
        """

        # Initialize pygame.
        if not Renderer._INITIALIZED:
            Renderer._INITIALIZED = True
            pygame.init()
            pygame.display.set_mode((WINDOW_PIXEL_WIDTH, WINDOW_PIXEL_HEIGHT))
            pygame.mixer.init(allowedchanges=pygame.AUDIO_ALLOW_CHANNELS_CHANGE)
            pygame.midi.init()
            midi_id = pygame.midi.get_default_input_id() if MIDI_INPUT_DEVICE_ID == "default" else int( MIDI_INPUT_DEVICE_ID)
            Renderer._MIDI_IN = pygame.midi.Input(midi_id)
        self._done: bool = False
        self._held: List[str] = list()
        self._undo_stack: List[Tuple[Surface, List[Rect]]] = list()
        self._undoing: bool = False

    def render(self, commands: List[Command]) -> RenderResult:
        """
        Process a list of render commands.

        :param commands: The commands.

        :return: The [`RenderResult`](render_result.md)
        """

        blits: List[Tuple[Surface, Rect]] = list()
        # Do each command.
        for command in commands:
            blits.append(command.do())
        # Get the previous surface.
        previous_surface = pygame.display.get_surface().convert()
        rects = pygame.display.get_surface().blits(blits, True)
        if len(rects) > 0:
            self._undo_stack.append((previous_surface, rects))
        # Update the screen.
        pygame.display.update(rects)
        pressed = []
        # Listen for events.
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                exit()
            elif event.type == pygame.KEYDOWN:
                k = pygame.key.name(event.key)
                pressed.append(k)
                if k not in self._held:
                    self._held.append(k)
            elif event.type == pygame.KEYUP:
                k = pygame.key.name(event.key)
                if k in self._held:
                    self._held.remove(k)
        # Get MIDI events.
        midi_events = Renderer._MIDI_IN.read(10)
        result = RenderResult(pressed=pressed, held=self._held, midi=[m[0] for m in midi_events])
        # Undo.
        if InputKey.undo in result.inputs_held and not self._undoing and len(self._undo_stack) > 0:
            self._undoing = True
            # Do a visual undo.
            surface, rects = self._undo_stack.pop(-1)
            pygame.display.get_surface().blits([(surface, rect, rect) for rect in rects], True)
            pygame.display.update(rects)
        # Stop undoing.
        elif InputKey.undo not in result.inputs_held and self._undoing:
            self._undoing = False
        return result

    def do(self) -> RenderResult:
        """
        Do something! By default, this will call `self.render([])` in a loop until `self._done == True`.

        :return: The [`RenderResult`](render_result.md).
        """

        result = self.render([])
        while not self._done:
            result = self.render([])
        return result

    @final
    def clear_undo_redo(self) -> None:
        """
        Clear the undo-redo history.
        """

        self._undo_stack.clear()
        self._undoing = False

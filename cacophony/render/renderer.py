from typing import List, Tuple
import pygame
from pygame import Surface, Rect
from cacophony.render.commands.command import Command
from cacophony.render.globals import WINDOW_PIXEL_WIDTH, WINDOW_PIXEL_HEIGHT
from cacophony.render.render_result import RenderResult
from cacophony.render.input_key import InputKey
from cacophony.text_to_speech import TextToSpeech
from cacophony.help_util import tooltip


class Renderer:
    """
    Blit surfaces to the display surface.
    """

    _INITIALIZED: bool = False

    def __init__(self):
        """
        (no parameters)
        """

        # Initialize pygame.
        if not Renderer._INITIALIZED:
            Renderer._INITIALIZED = True
            pygame.init()
            pygame.display.set_mode((WINDOW_PIXEL_WIDTH, WINDOW_PIXEL_HEIGHT))
        self._done: bool = False
        self._held: List[str] = list()
        self.__app_help_text: str = Renderer.__get_app_help_text()

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
        rects = pygame.display.get_surface().blits(blits, True)
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
        result = RenderResult(pressed=pressed, held=self._held, midi=[])
        if InputKey.app_help in result.inputs_pressed:
            TextToSpeech.say(self.__app_help_text)
        return result

    def do(self) -> None:
        """
        Do something! By default, this will call `self.render([])` in a loop until `self._done == True`.
        """

        while not self._done:
            self.render([])

    @staticmethod
    def __get_app_help_text() -> str:
        """
        :return: The help text for the whole app.
        """

        text = "This is Cacophony. "
        tooltips = [tooltip(keys=[InputKey.next_panel, InputKey.previous_panel], predicate="cycle through panels.", boop="and"),
                    tooltip(keys=[InputKey.panel_help], predicate="ask me to tell you what the current panel does."),
                    tooltip(keys=[InputKey.widget_help], predicate="ask me to tell you what the current widget does."),
                    tooltip(keys=[InputKey.app_help], predicate="ask me to say this message again.")]
        text += " ".join(tooltips)
        return text

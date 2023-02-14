from typing import List, Tuple
import pygame
from pygame import Surface, Rect
from cacophony.render.commands.command import Command
from cacophony.render.globals import WINDOW_PIXEL_WIDTH, WINDOW_PIXEL_HEIGHT
from cacophony.render.render_result import RenderResult


class Renderer:
    _INITIALIZED: bool = False

    def __init__(self):
        if not Renderer._INITIALIZED:
            Renderer._INITIALIZED = True
            pygame.init()
            pygame.display.set_mode((WINDOW_PIXEL_WIDTH, WINDOW_PIXEL_HEIGHT))
        self._done: bool = False
        self._held: List[str] = list()

    def render(self, commands: List[Command]) -> RenderResult:
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
        return RenderResult(pressed=pressed, held=self._held, midi=[])


    def do(self) -> None:
        while not self._done:
            self.render([])

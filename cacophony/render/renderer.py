from typing import List, Tuple
import pygame
from pygame import Surface, Rect
from cacophony.render.render_command import RenderCommand
from cacophony.render.globals import WINDOW_PIXEL_WIDTH, WINDOW_PIXEL_HEIGHT


class Renderer:
    _INITIALIZED: bool = False

    def __init__(self):
        if not Renderer._INITIALIZED:
            Renderer._INITIALIZED = True
            pygame.init()
            pygame.display.set_mode((WINDOW_PIXEL_WIDTH, WINDOW_PIXEL_HEIGHT))
        self._done: bool = False

    def render(self, commands: List[RenderCommand]) -> None:
        blits: List[Tuple[Surface, Rect]] = list()
        # Do each command.
        for command in commands:
            blits.append(command.do())
        rects = pygame.display.get_surface().blits(blits, True)
        # Update the screen.
        pygame.display.update(rects)
        # Listen for events.
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                exit()

    def do(self) -> None:
        while not self._done:
            self.render([])


r = Renderer()
from cacophony.render.text import Text
r.render([Text(text="Hello world", position=(2, 3), color=(200, 0, 200))])
r.do()
from time import time
from pathlib import Path
from typing import List, Optional
from platform import system
from pygame.display import get_surface
from pygame import Surface, Rect
from cacophony.render.panel.panel import Panel
from cacophony.render.commands.command import Command
from cacophony.render.commands.arrow import Arrow
from cacophony.render.commands.blit import Blit
from cacophony.render.commands.line import Line
from cacophony.render.commands.text import Text
from cacophony.render.macros.parent_rect import get_parent_rect
from cacophony.render.globals import COLORS, WINDOW_GRID_WIDTH, WINDOW_GRID_HEIGHT, SCROLL_DT
from cacophony.render.color import Color
from cacophony.render.render_result import RenderResult
from cacophony.render.input_key import InputKey
from cacophony.util import tooltip
from cacophony.paths import USER_DIRECTORY
from cacophony.cardinal_direction import CardinalDirection


class OpenFile(Panel):
    """
    Open a file.
    """

    # The initial root directory is a copy of the user directory.
    _ROOT_DIRECTORY: Path = Path(USER_DIRECTORY).absolute()

    def __init__(self, suffixes: List[str]):
        """
        :param suffixes: A list of file suffixes that we're searching for.
        """

        super().__init__(title="Open",
                         position=(0, 0),
                         size=(WINDOW_GRID_WIDTH // 2, WINDOW_GRID_HEIGHT // 2),
                         pivot=(0.5, 0.5),
                         anchor=(0.5, 0.5),
                         parent_rect=None)
        self._open_file_rect: Rect = get_parent_rect(position=self._position, size=self._size, pivot=self._pivot,
                                                     anchor=self._anchor)
        self._previous_surface: Surface = get_surface().convert()
        self._suffixes: List[str] = suffixes
        self._directory: Optional[Path] = Path(OpenFile._ROOT_DIRECTORY)
        self.path: Optional[Path] = None
        self.done: bool = False
        self._page_index: int = 0
        self._element_index: int = 0
        self._pages: List[List[Path]] = self._get_pages()
        self._scroll_t0 = 0
        # List the system drives.
        if system() == "Windows":
            import win32api
            drives = win32api.GetLogicalDriveStrings()
            drives = drives.split('\000')[:-1]
            self._drives: List[Path] = [Path(drive) for drive in drives]
        else:
            self._drives = []

    def get_panel_help(self) -> str:
        return "Open file. " + tooltip(keys=[InputKey.up, InputKey.down], predicate="scroll", boop="and") + " " + \
               tooltip(keys=[InputKey.left], predicate="move up a directory") + " " + \
               tooltip(keys=[InputKey.right], predicate="open directory") + " " + \
               tooltip(keys=[InputKey.select], predicate="open file") + " " + \
               tooltip(keys=[InputKey.cancel], predicate="close this window")

    def get_widget_help(self) -> str:
        if self._directory is not None and len(self._pages) > 0 and len(self._pages[self._page_index]) > 0 and (not self._pages[self._page_index][self._element_index].is_dir()):
            return self._pages[self._page_index][self._element_index].name
        else:
            return "No file selected."

    def _render_panel(self, focus: bool) -> List[Command]:
        commands = super()._render_panel(focus=focus)
        if self.done:
            commands.append(Blit(surface=self._previous_surface, position=(0, 0)))
            return commands
        x = 1
        y = 1
        w = self._size[0] - 2
        # Get the title.
        if self._directory is not None:
            commands.append(Text(text=str(self._directory.resolve()).replace("\\", "/"),
                                 position=(x, y),
                                 size=w,
                                 truncate_left=False,
                                 parent_rect=self._open_file_rect,
                                 text_color=COLORS[Color.border_no_focus],
                                 background_color=COLORS[Color.panel_background]))
            y += 1
        commands.append(Line(length=w,
                             color=COLORS[Color.border_no_focus],
                             position=(x, y),
                             parent_rect=self._open_file_rect,
                             vertical=False))
        y += 1
        # Add the files.
        if len(self._pages) > 0 and len(self._pages[self._page_index]) > 0:
            for i, path in enumerate(self._pages[self._page_index]):
                if not focus:
                    c = Color.border_no_focus
                elif i == self._element_index:
                    c = Color.parameter_value
                else:
                    c = Color.border_focus
                p = str(path.resolve()).replace("\\", "/")
                if path.is_dir() and p[-1] != "/":
                    p += "/"
                commands.append(Text(text=p,
                                     position=(x, y),
                                     size=w,
                                     truncate_left=False,
                                     parent_rect=self._open_file_rect,
                                     text_color=COLORS[c],
                                     background_color=COLORS[Color.panel_background]))
                y += 1
        # Blit arrows.
        arrow_x = self._size[0] - 4
        arrow_color = COLORS[Color.border_focus if focus else Color.border_no_focus]
        if self._page_index > 0:
            commands.append(Arrow(color=arrow_color,
                                  direction=CardinalDirection.north,
                                  position=(arrow_x, 0),
                                  parent_rect=self._open_file_rect))
        if self._page_index < len(self._pages) - 1:
            commands.append(Arrow(color=arrow_color,
                                  direction=CardinalDirection.south,
                                  position=(arrow_x, self._size[1] - 2),
                                  parent_rect=self._open_file_rect))
        return commands

    def _do_result(self, result: RenderResult) -> bool:
        if InputKey.left in result.inputs_pressed:
            if self._directory is None:
                return False
            # Top-level directory.
            if self._directory.parent == self._directory:
                # List the drives.
                if len(self._drives) > 0 and system() == "Windows":
                    self._directory = None
                    self._pages.clear()
                    self._pages.append(self._drives[:])
                    self._element_index = 0
                    self._page_index = 0
                    return True
            # Go up a directory.
            else:
                self._directory = Path(self._directory.parent)
                self._pages = self._get_pages()
                return True
        elif InputKey.right in result.inputs_pressed:
            # Go down a directory.
            if self._pages[self._page_index][self._element_index].is_dir():
                self._directory = Path(self._pages[self._page_index][self._element_index])
                self._pages = self._get_pages()
                return True
        # Cancel and restore balance to the Force.
        elif InputKey.cancel in result.inputs_pressed:
            self.done = True
            return True
        # Empty page.
        if len(self._pages) == 0 or len(self._pages[self._page_index]) == 0:
            return False
        if InputKey.up in result.inputs_pressed:
            return self._scroll(up=True)
        elif InputKey.down in result.inputs_pressed:
            return self._scroll(up=False)
        # Select a file.
        elif InputKey.select in result.inputs_pressed:
            p = self._pages[self._page_index][self._element_index]
            if p.is_file():
                self.path = p
                self.done = True
                return True
        # Hold down the key and scroll.
        if time() - self._scroll_t0 > SCROLL_DT:
            if InputKey.up in result.inputs_held:
                return self._scroll(up=True)
            elif InputKey.down in result.inputs_held:
                return self._scroll(up=False)
        return False

    def _get_pages(self) -> List[List[Path]]:
        """
        :return: A list of lists of paths that will fit in the panel.
        """

        self._element_index = 0
        self._page_index = 0
        num_files_per_page: int = self._size[1] - 4
        pages: List[List[Path]] = []
        page: List[Path] = list()
        for f in self._directory.iterdir():
            try:
                if f.suffix in self._suffixes or f.is_dir():
                    if f.is_dir():
                        try:
                            list(f.iterdir())
                        except PermissionError:
                            continue
                    page.append(f)
                    if len(page) >= num_files_per_page:
                        pages.append(page[:])
                        page.clear()
            except PermissionError:
                continue
        if len(page) > 0:
            pages.append(page)
        return pages

    def _scroll(self, up: bool) -> bool:
        """
        :param up: If True, scroll up. If False, scroll down.

        :return: True if we scrolled.
        """

        if up:
            if self._element_index == 0 and self._page_index > 0:
                self._page_index -= 1
                self._element_index = len(self._pages[self._page_index]) - 1
                self._scroll_t0 = time()
                return True
            elif self._element_index > 0:
                self._element_index -= 1
                self._scroll_t0 = time()
                return True
        else:
            if self._element_index == len(self._pages[self._page_index]) - 1 and self._page_index < len(self._pages) - 1:
                self._page_index += 1
                self._element_index = 0
                self._scroll_t0 = time()
                return True
            elif self._element_index < len(self._pages[self._page_index]) - 1:
                self._element_index += 1
                self._scroll_t0 = time()
                return True
        return False

from abc import ABC, abstractmethod
from typing import Tuple, Dict, List, Union
from cacophony.waveform_generator import WaveformGenerator
from cacophony.callbacker import Callbacker
from cacophony.cardinal_direction import CardinalDirection
from cacophony.render.renderer import Renderer
from cacophony.render.color import Color
from cacophony.render.globals import WINDOW_GRID_WIDTH, WINDOW_GRID_HEIGHT, COLORS, CLEAR_COLOR
from cacophony.render.commands.text import Text
from cacophony.render.commands.rectangle import Rectangle
from cacophony.render.commands.border import Border
from cacophony.render.commands.arrow import Arrow


class _Options(ABC):
    def __init__(self, index: int = 0):
        self.__options: list = self._get_options()
        self.__index: int = index

    @property
    def value(self):
        if self.__index >= len(self.__options):
            return None
        else:
            return self.__options[self.__index]

    def increment(self):
        self.__index += 1
        if self.__index >= len(self.__options):
            self.__index = 0
        return self.value

    def decrement(self):
        self.__index -= 1
        if self.__index < 0:
            self.__index = len(self.__options) - 1
        return self.value

    @abstractmethod
    def _get_options(self) -> list:
        raise Exception()


class _OptionsList(_Options):
    def __init__(self, options: list, index: int = 0):
        self.__options: list = options
        super().__init__(index=index)

    def _get_options(self) -> list:
        return self.__options


class _OptionsDict(_Options):
    def __init__(self, options: dict, index: int = 0):
        self.__options: dict = options
        super().__init__(index=index)

    @property
    def value(self):
        v = super().value
        if v is None:
            return None
        else:
            return self.__options[v]

    def _get_options(self) -> list:
        return list(self.__options.keys())


class WaveformGeneratorArgs(Renderer):
    def __init__(self, wfg: WaveformGenerator, focus: bool):
        super().__init__()
        self._wfg: WaveformGenerator = wfg
        self._focus: bool = focus
        # Build lists of parameters.
        self.parameters: Dict[str, Union[str, bool, int, float, _Options, Callbacker]] = dict()
        for k in self._wfg.__dict__:
            # Ignore private or class fields.
            if k[0] == "_" or k.startswith(self._wfg.__class__.__name__):
                continue
            v = self._wfg.__dict__[k]
            # Add options.
            if isinstance(v, list):
                self.parameters[k] = _OptionsList(v, index=0)
            elif isinstance(v, dict):
                self.parameters[k] = _OptionsDict(v, index=0)
            elif isinstance(v, Callbacker):
                self.parameters[k] = v
            else:
                raise Exception(k, v)
        self.parameter_focus_index: int = 3
        self.__parameter_keys: List[str] = list(self.parameters.keys())

    def do(self) -> None:
        self._show_panel()
        super().do()

    def _show_panel(self) -> None:
        border_color = COLORS[Color.border_focus] if self._focus else COLORS[Color.border_no_focus]
        background_color = COLORS[Color.panel_background]
        panel_size: Tuple[int, int] = (WINDOW_GRID_WIDTH // 5, WINDOW_GRID_HEIGHT)
        # These coordinates are used for UI elements within the panel.
        x = WINDOW_GRID_WIDTH - panel_size[0] + 2
        w = panel_size[0] - 3
        # 1. Clear the previous panel by drawing a rectangle with the background color.
        # 2. Add a border around the panel.
        # 3. Add the title bar.
        # 4. Add the title text.
        commands = [Rectangle(position=(0, 0), size=panel_size, pivot=(1, 0), anchor=(1, 0), color=background_color),
                    Border(position=(0, 0), size=panel_size, pivot=(1, 0), anchor=(1, 0), color=border_color),
                    Rectangle(position=(0, 0), size=(panel_size[0], 1), pivot=(1, 0), anchor=(1, 0), color=border_color),
                    Text(text=self._wfg.__class__.__name__,
                         position=(x, 0),
                         size=w,
                         text_color=COLORS[Color.panel_title_focus],
                         background_color=COLORS[Color.border_focus])]
        # Get the initial y value for the text.
        y = 2
        for i in range(len(self.__parameter_keys)):
            # Highlight the parameter that has the focus.
            if self._focus and i == self.parameter_focus_index:
                parameter_key_color = background_color
                parameter_key_background_color = COLORS[Color.parameter_key]
                parameter_value_background_color = COLORS[Color.parameter_value_background_focus]
            else:
                parameter_key_color = COLORS[Color.parameter_key]
                parameter_key_background_color = background_color
                parameter_value_background_color = background_color
            # Get the key.
            k = self.__parameter_keys[i]
            # Show the title.
            commands.append(Text(text=k.replace("_", " ").title(),
                                 position=(x, y),
                                 size=w,
                                 text_color=parameter_key_color,
                                 background_color=parameter_key_background_color))
            y += 1
            # Show the value.
            v = self.parameters[k]
            if isinstance(v, _Options):
                v_text = str(v.value)
                commands.append(Text(text=v_text,
                                     position=(x, y),
                                     size=len(v_text),
                                     text_color=COLORS[Color.parameter_value],
                                     background_color=parameter_value_background_color))
                # Arrows.
                if self._focus and i == self.parameter_focus_index:
                    commands.extend([Arrow(position=(x - 1, y),
                                           direction=CardinalDirection.west,
                                           color=COLORS[Color.parameter_value]),
                                     Arrow(position=(x + len(v_text), y),
                                           direction=CardinalDirection.east,
                                           color=COLORS[Color.parameter_value])])

            elif isinstance(v, Callbacker):
                commands.append(Text(text=str(v.get()),
                                     position=(x, y),
                                     size=w,
                                     text_color=COLORS[Color.parameter_value],
                                     background_color=parameter_value_background_color))
            else:
                raise Exception(v)
            y += 2
        self.render(commands)

from cacophony.render.renderer import Renderer
from cacophony.render.macros.panel import panel


r = Renderer()
commands = panel(title="Hello world",
                 position=(0, 0),
                 pivot=(0, 0),
                 anchor=(0, 0),
                 size=(32, 16),
                 focus=True)
commands.extend(panel(title="Bottom right",
                      position=(0, 0),
                      pivot=(1, 1),
                      anchor=(1, 1),
                      size=(64, 24),
                      focus=False))
r.render(commands)
r.do()

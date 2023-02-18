from cacophony.render.renderer import Renderer
from cacophony.render.panel.panel import Panel


r = Renderer()
panel_0 = Panel(title="Hello world",
                position=(0, 0),
                pivot=(0, 0),
                anchor=(0, 0),
                size=(32, 16))
panel_1 = Panel(title="Bottom right",
                position=(0, 0),
                pivot=(1, 1),
                anchor=(1, 1),
                size=(64, 24))
commands = panel_0.blit(focus=True)
commands.extend(panel_1.blit(focus=False))
r.render(commands)
r.do()

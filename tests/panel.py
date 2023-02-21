from cacophony.render.renderer import Renderer
from cacophony.render.panel.panel import Panel


r = Renderer()
result = r.render([])
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
commands = panel_0.render(focus=True, result=result)
commands.extend(panel_1.render(focus=False, result=result))
r.render(commands)
r.do()

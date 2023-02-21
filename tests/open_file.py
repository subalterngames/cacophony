from cacophony.render.renderer import Renderer
from cacophony.render.panel.open_file import OpenFile


r = Renderer()
result = r.render([])
panel = OpenFile(suffixes=[".py"])
while not panel.done:
    result = r.render(panel.render(result, True))
print(panel.path)

from cacophony.render.renderer import Renderer
from cacophony.render.text import Text
from cacophony.render.border import Border


r = Renderer()
r.render([Text("Hello world", position=(0, 0), pivot=(0, 0), anchor=(0, 0), color=(255, 0, 0)),
          Text("Top right", position=(0, 0), pivot=(1, 0), anchor=(1, 0), color=(0, 255, 0)),
          Text("Bottom right", position=(0, 0), pivot=(1, 1), anchor=(1, 1), color=(0, 0, 255)),
          Text("Bottom left", position=(0, 0), pivot=(0, 1), anchor=(0, 1), color=(100, 22, 255)),
          Border(position=(0, 0), pivot=(0.5, 0.5), anchor=(0.5, 0.5), size=(16, 8), color=(255, 100, 100)),
          Text("Center", position=(0, 0), pivot=(0.5, 0.5), anchor=(0.5, 0.5), color=(255, 255, 255))
          ])
r.do()

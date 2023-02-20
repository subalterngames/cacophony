from cacophony.render.renderer import Renderer
from cacophony.render.panel.main_menu import MainMenu


r = Renderer()
m = MainMenu()
r.render(m.blit(focus=True))
r.do()

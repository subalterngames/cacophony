from pathlib import Path
from PIL import Image, ImageFont, ImageDraw

def get_x(text: str) -> float:
    box = font.getbbox(text)
    return w / 2 - (box[2] - box[1]) / 2

splash = """   ____                      _
  / ___|__ _  ___ ___  _ __ | |__   ___  _ __  _   _
 | |   / _` |/ __/ _ \| '_ \| '_ \ / _ \| '_ \| | | |
 | |__| (_| | (_| (_) | |_) | | | | (_) | | | | |_| |
  \____\__,_|\___\___/| .__/|_| |_|\___/|_| |_|\__, |
                      |_|                      |___/

"""
num_splash_lines = splash.count("\n")

font_path = Path("../data/NotoSansMono-Regular.ttf")
font_size = 20
font = ImageFont.truetype(str(font_path.resolve()), font_size)
char_box = font.getbbox("A")
char_width = char_box[2] - char_box[0]

num_chars = 52
w = num_chars * char_width
h = font_size * (num_splash_lines + 5)
print(w, h)
image = Image.new("RGB", (w, h), color=(0, 0, 0))
draw = ImageDraw.Draw(image)
x = -char_width
for i, line in enumerate(splash.split("\n")):
    draw.text((x, font_size * i), line, font=font, fill=(245, 169, 184))
y = font_size * (num_splash_lines + 1)
by_text = "By Esther Alter"
draw.text((get_x(by_text), y), by_text, font=font, fill=(91, 206, 250))
subaltern_games_text = "Subaltern Games, LLC"
y += font_size * 2
draw.text((get_x(subaltern_games_text), y), subaltern_games_text, font=font, fill=(200, 200, 200))
image.save("../data/splash.png")

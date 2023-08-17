from pathlib import Path

"""
Set the speech dispatcher version for Ubuntu 22.
This is a terrible hack. 
If someone has a better idea of how to do this, please tell me.
"""

# Yikes.
path = Path("../Cargo.toml")
# Uhhh...
text = path.read_text()
# WTF?!
text = text.replace("speech_dispatcher_0_9", "speech_dispatcher_0_11")
# At last, we're done.
path.write_text(text)

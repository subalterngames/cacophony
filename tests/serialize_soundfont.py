from os import remove
from h5py import File
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.synthesizer.soundfont import SoundFont

name = "test.hdf5"
synth = SoundFont(path="D:/SoundFonts/ms_basic.sf3", bank_index=0, channel_index=0, preset_index=3)
print("created")
with File(name, "w") as f:
    g = f.create_group("track")
    synth.serialize(g)
with File(name, "r") as f:
    g = f["track"]["synthesizer"]
    synth: SoundFont = Synthesizer.deserialize(g)
    print("deserialized")
remove(name)

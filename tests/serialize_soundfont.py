from os import remove
from h5py import File
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.synthesizer.soundfont import SoundFont
from cacophony.callbacker.dictionary import Dictionary, IntList
name = "test.hdf5"
synth = SoundFont(path="D:/SoundFonts/ms_basic.sf3", bank_index=0, channel_index=0, preset_index=3)
for k in synth.__dict__:
    if k[0] != "_":
        print(k, synth.__dict__[k], isinstance(synth.__dict__[k], IntList))
print("created")
getattr(synth, "bank").index = 1
print(synth.bank.get())
with File(name, "w") as f:
    g = f.create_group("track")
    synth.serialize(g)
with File(name, "r") as f:
    g = f["track"]["synthesizer"]
    synth: SoundFont = Synthesizer.deserialize(g)
    print("deserialized")
remove(name)

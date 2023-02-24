# Add a Synthesizer to Cacophony

Cacophony is designed to make it as easy as possible to add new types of synthesizers by allowing users to write subclasses of `Synthesizer`.

These Python classes should *not* be understood as a new audio plugin file type. No one wants new file standards in established industries, least of all me. Instead, a `Synthesizer` is a wrapper that can accommodate a lot of different types of existing plugins/file standards/etc.

So, to answer the inevitable questions of "Does Cacophony support *x*?" the answer is: probably, sort of, but not out of the box, but if you know Python programming, it shouldn't be too hard to set add *x* to Cacophony.

What Cacophony *doesn't* support, and will never support, is the option to add a custom/existing plugin UI. Synthesizer UIs are and always will be a scrollable list of fields in a sub-panel of the program.

## 0. Setup

1. Clone this repo.
2. `cd cacophony`
3. `pip3 install -e .`
4. `mkdir synthesizers`
5. `cd synthesizers`

All synthesizer scripts *must* be located in the directory you just created: `~/cacophony/synthesizers/`

## 1. The `Synthesizer` class

Your synthesizer must be a subclass of Cacophony's `Synthesizer` class. So, start by writing something like this:

```python
from cacophony.synthesizer.synthesizer import Synthesizer

class MySynth(Synthesizer):
    # TODO
```

## 2. The constructor

The constructor *must* have at least four parameters: `beat_index`, `gain_index`, `use_volume`, and `volume_index`. (Why the `_index` suffix? Glad you asked. I'll explain this later.)

The default values will set the defaults for a new synthesizer in the program. So, if `volume_index=127`, then a new `MySynth` will default to 127 volume.

Call `super().__init__()` to set the parameters:

```python
from cacophony.synthesizer.synthesizer import Synthesizer

class MySynth(Synthesizer):
    def __init__(self, beat_index: int = 5, gain_index: int = 127, use_volume: bool = True, volume_index: int = 127):
        super().__init__(beat_index=beat_index, gain_index=gain_index, use_volume=use_volume, volume_index=volume_index)
```

## 3. Object fields and constructor parameters

In Cacophony, fields are represented in the UI as having a range of options or having a default value. Sometimes, there is a callback. For example, when you set the path of a `SoundFont`, it needs to actually load the file and set default bank and preset values.

The `Synthesizer` fields must be specialized types that the UI will recognize as a range of options.

**TODO**

### 3a. `Callbacker`

```python
from cacophony.callbacker.callbacker import Callbacker
```

`Callbacker` is the abstract base class of all valid field types. It has three fields:

1. `tts` a *text-to-speech* string. This will be spoken by Casey in addition to the name of field and the UI controls. Example: `tts="Set the track's coolness."`
2. `callback` an optional Callable to invoke. `Callbacker` doesn't actually define when it will be invoked; that's handled by subclasses. If None, there is no callback.
3. `kwargs` a dictionary of keyword arguments. When `callback` is invoked, these arguments are passed into it. If None, there are no arguments.

Like I just explained, this is an abstract class, so you'd need to create a subclass to use it. The rest of the types listed here are all subclasses of `Callbacker`.

### 3b. `IndexedList[T]`

```python
from cacophony.callbacker.indexed_list import IndexedList
```

An index list of type T. There is a list of values and an integer index.

An `IndexedList` looks like this:

**TODO**

Constructor fields:

- `values` a list of type T.
- `tts` inherited from `Callbacker`.
- `index` the initial index.
- `callable` inherited from `Callbacker`.
- `kwargs` inherited from `Callbacker`.

Whenever Cacophony sets the `index` (e.g. by cycling the UI widget), the `callback` (if it exists) is invoked.

Remember way back, 300 words ago, when I said that I'd explain why the fields have `_index` suffixes? That's because we're setting the *index* of the `IndexedList` of beats, the `IndexedList` of gains, etc.

When you save the file, Cacophony stores the *index* of the IndexedList, which is all the information it needs to rebuild the list later.

`IndexedList` has a type parameter. If you want a list of strings, it should be `IndexedList[str]`.

This is an example of how to add an indexed list of *pre-defined* filenames:

```python
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.callbacker.indexed_list import IndexedList

class MySynth(Synthesizer):
    def __init__(self, filename_index: int = 0, beat_index: int = 5, gain_index: int = 127, use_volume: bool = True, volume_index: int = 127):
        super().__init__(beat_index=beat_index, gain_index=gain_index, use_volume=use_volume, volume_index=volume_index)
        self.paths: IndexedList[str] = IndexedList(values=["plugin.vst", "another_plugin.vst3"], 
                                                   index=filename_index,
                                                   tts="Set the plugin file.",
                                                   callback=self.filename_callback)
        
    def filename_callback(self):
        print("Do something fun here.")

```

`IndexedList` has two methods you might want to override:

1. `get_str()` returns the stringified value. By default, this is just `str(self.values[self.index])`
2. `get_strs()` returns the stringified list of `self.values`. By default, this is just `[str(v) for v in self.values]`

### 3c. `IntList`

```python
from cacophony.callbacker.int_list import IntList
```

This is just a convenient wrapper for `IndexedList[int]`.

If you want a range of integers from 0 to 127, there's a wrapper function eagerly waiting your attention:

```python
from cacophony.callbacker.int_list import IntList, zero_127

coolness: IntList = zero_127(index=60, tts="Set the coolness of the music.")
```

### 3d. `FloatList`

```python
from cacophony.callbacker.float_list import FloatList
```

This is just a convenient wrapper for `IndexedList[float]`.

### 3e. `EnumList`

```python
from cacophony.callbacker.enum_list import EnumList
```

This is an `IndexedList` for enum values. It overrides `get_str()` and `get_strs()` to print the enum values prettily.

### 3f. `Dictionary`

```python
from cacophony.callbacker.dictionary import Dictionary
```

This is an `IndexedList` where the values in the constructor are of type `Dict[int, str]`. `self.values` is a list of integers. But `get_str()` and `get_strs()` will print the *string value* of the dictionary, not the integer key:

```python
from cacophony.callbacker.dictionary import Dictionary

coolness: Dictionary = Dictionary(values={0: "not cool", 1: "a little cool", 2: "extremely cool"},
                                  tts="Set the coolness of the music.",
                                  index=1)
print(coolness.get())  # 1
print(coolness.get_str())  # a little cool
```

### 3g. `Value[T]`

```python
from cacophony.callbacker.value import Value
```

A value of type T with a callback. 

A `Value` looks  like this:

**TODO**

A `Value[bool]` looks like this:

**TODO**

Constructor fields:

- `value` the default value.
- `tts` inherited from `Callbacker`.
- `callable` inherited from `Callbacker`.
- `kwargs` inherited from `Callbacker`.

This sets an arbitrary integer field:

```python
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.callbacker.value import Value

class MySynth(Synthesizer):
    def __init__(self, random_seed: int = 0, beat_index: int = 5, gain_index: int = 127, use_volume: bool = True, volume_index: int = 127):
        super().__init__(beat_index=beat_index, gain_index=gain_index, use_volume=use_volume, volume_index=volume_index)
        self.random_seed: Value[int] = Value(value=random_seed, tts="Set a random seed", callback=self.random_seed_callback)
        
    def random_seed_callback(self):
        print("Do something fun here.")
```

### 3h. `FilePath`

```python
from cacophony.callbacker.file_path import FilePath
```

This is a specialized `Value[str]` subclass that tells Cacophony to open the file directory prompt sub-panel.

## 4. Functions

At long last, we've arrived at the interesting part.

If you still want to make a custom `Synthesizer`, you need to override three methods:

### 4a. `get_channels()`

Returns an integer. How many audio channels? Depends.

This is an example of how to dynamically set the number of channels from the UI:

```python
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.callbacker.int_list import IntList

class MySynth(Synthesizer):
    def __init__(self, channel_index: int = 0, beat_index: int = 5, gain_index: int = 127, use_volume: bool = True, volume_index: int = 127):
        super().__init__(beat_index=beat_index, gain_index=gain_index, use_volume=use_volume, volume_index=volume_index)
        self.channel: IntList[int] = IntList(values=list(range(256)), 
                                             tts="The number of audio channels.",
                                             index=channel_index)

    def get_channels(self) -> int:
        return self.channel.get()
```

### 4b. `get_help_text()`

What should Casey say when the user asks for panel help?

```python
from cacophony.synthesizer.synthesizer import Synthesizer
from cacophony.callbacker.int_list import IntList

class MySynth(Synthesizer):
    def __init__(self, channel_index: int = 0, beat_index: int = 5, gain_index: int = 127, use_volume: bool = True, volume_index: int = 127):
        super().__init__(beat_index=beat_index, gain_index=gain_index, use_volume=use_volume, volume_index=volume_index)
        self.channel: IntList[int] = IntList(values=list(range(256)),
                                             tts="The number of audio channels.",
                                             index=channel_index)

    def get_channels(self) -> int:
        return self.channel.get()
    
    def get_help_text(self) -> str:
        return "My first synthesizer."
```

### 4c. `get(note, volume, duration)`

This is it. The big one. Override this function to return audio. There are three parameters:

1. `note` a MIDI note value.
2. `volume` a volume value (0-127)
3. `duration` a duration in *seconds*

Returns: `bytes`. Presumably, the bytes are audio samples.

I asked Casey and they said that they don't care how you generate your audio data as long as you do it within `get()`. You want to open a socket and return a chunk of a stream? Great, do it. Want to open standard out instead? No problem. Want to get the return value from a .dll? You can do that. You can do whatever you want with a synthesizer, as long as you can conceptualize it as having a single function that returns audio samples.

**TODO**
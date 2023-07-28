# How to run

If you want to compile Cacophony, [read this](compile.md).

## Windows

Double-click the executable or run it in a terminal: `./cacophony.exe`

## MacOS

The first time you try running Cacophony you might be greeted with an ugly error about Cacophony.app being damaged. The app isn't damaged; the problem is that Apple is evil.

To "fix" the .app:

1. Move Cacophony.app into `/home/<username>/`. To see this directory open Finder and press: `⌘ + left shift + h`.
2. In a terminal: `cd ~` and press enter.
3. In the same terminal: `xattr -r -d com.apple.quarantine Cacophony.app` and press enter.

You can how double-click the .app and move it to wherever you want.

## Linux

The first time you try running Cacophony you may need to set up MIDI input on your machine: `sudo usermod -a -G audio replacethiswithyourusername`

1. In a terminal: `cd <directory>/cacophony` (Replace `<directory>` with the actual path)
2. `./cacophony`
# Setup

These are instructions for how to set up Cacophony the first time you run it.

If you want to compile Cacophony, [read this](compile.md).

To run Cacophony, [read this](run.md).

## Windows

You don't need to do anything extra.

***MIDI Input:*** Plug in your MIDI controller and wait for the drivers to install.

## MacOS

The first time you try running Cacophony you might be greeted with an ugly error about Cacophony.app being damaged. The app isn't damaged; the problem is that Apple is evil.

To "fix" the .app:

1. Move Cacophony.app into `/home/<username>/`. To see this directory open Finder and press: `⌘ + left shift + h`.
2. In a terminal: `cd ~` and press enter.
3. In the same terminal: `xattr -r -d com.apple.quarantine Cacophony.app` and press enter.

***MIDI Input:*** Plug in your MIDI controller.

## Linux

You don't need to do anything extra.

***MIDI Input:*** The first time you try running Cacophony you may need to set up MIDI input on your machine: `sudo usermod -a -G audio replacethiswithyourusername`
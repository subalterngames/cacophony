# Design Manifesto

The overall goal is that Cacophony should feel as close as possible to improvising on an actual instrument and writing notes on physical paper with a physical pencil. There should be as little as possible between you and writing music.

## 1. Clean interface

Cacophony's ASCII-esque interface is inspired by roguelikes moreso than, say, vim. I like ASCII interfaces because they require the designer to be highly economical about what information is on the screen at any given time.

Every DAW I've ever seen is exceedingly, jaw-droppingly ugly. I'm stating this as a programmer who routinely uses bloated IDEs like Unity3D; Unity may be horrendous, but DAW interfaces are like an order of magnitude worse.

Many DAWs are attempting to simulate actual studio setups or synthesizers. That's probably useful if you want to actually make a career out of making music, but I don't want to make a career out of making music, I just want to make music. So, Cacophony doesn't make any attempt to emulate any "real-world" audio production verbs.

## 2. Accessibility

Cacophony is *very* screen-reader friendly, which is great in two respects: First, that it makes the app more accessibile. Second, that it results in a cleaner interface.

To avoid cluttering up the interface with tooltips and labels, nearly everything can be described by the text-to-speech engine. The actualy text-to-speech engine is external to the application. This probably makes the application more suspectible to bitrot but that might just be a fundamental feature of accessible software. Audio-only tooltips are usually frowned upon because there's no way to know if the user has audio on. However, this is a music-making program so we *do* know that the user has audio on.

Cacophony's interface should be visually easy to read; the default interface is fairly high-contrast with a large font.

## 3. Ergonomic

I don't want to juggle a qwerty keyboard, MIDI keyboard, *and* a mouse. I don't want to click and drag a mouse to do basic tasks like set the gain. I don't want to click through several windows to listen to each instrument.

Cacophony is qwerty-and-MIDI input *only*. Between this an the helpful screen-reader, the best-case scenario for Cacophony is that it reaches a state at which experienced users hardly ever need to look at the screen.

## 4. No live recording

Note input is modeled after MuseScore and doesn't include live recording. I really like this form of input, but I don't need the result to be pristine sheet music that I can print to PDF, hence Cacophony's piano roll rectangles. (Cacophony *can* export to .mid which can then be imported into MuseScore.)

## 5. First-class Linux support

[I'm working on it!](roadmap.md)

## 6. Easy to localize

All of Cacophony's strings--spoken and displayed--are in a .csv file where each column is a language. The default font, Noto Sans, was designed to support a wide range of languages. So, it *should* be easy to localize Cacophony, though [this hasn't been tested yet](roadmap.md).
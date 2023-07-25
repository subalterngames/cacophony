use crate::TtsString;
use common::config::{parse, parse_bool};
use ini::Ini;
use tts::Tts;

/// Text-to-speech.
pub struct TTS {
    /// The text-to-speech engine.
    tts: Option<Tts>,
    /// A queue of text-to-speech strings. Casey is saying the first element, if any.
    speech: Vec<TtsString>,
    /// If true, return subtitle text.
    pub show_subtitles: bool,
}

impl TTS {
    pub fn new(config: &Ini) -> Self {
        let section = config.section(Some("TEXT_TO_SPEECH")).unwrap();
        // Get subtitles.
        let show_subtitles = parse_bool(section, "subtitles");
        // Try to load the text-to-speech engine.
        let tts = match Tts::default() {
            Ok(mut t) => {
                // Try to set the voice.
                if let Ok(voices) = t.voices() {
                    if t.set_voice(&voices[parse::<usize>(section, "voice_id")])
                        .is_ok()
                    {}
                }
                // Try to set the rate.
                let rate_key = if cfg!(windows) {
                    "rate_windows"
                } else if cfg!(target_os = "macos") {
                    "rate_macos"
                } else {
                    "rate_linux"
                };
                if t.set_rate(parse(section, rate_key)).is_ok() {}
                Some(t)
            }
            Err(_) => None,
        };
        Self {
            show_subtitles,
            tts,
            speech: vec![],
        }
    }

    /// Stop speaking.
    pub fn stop(&mut self) {
        if self.is_speaking() {
            if let Some(tts) = &mut self.tts {
                if tts.stop().is_ok() {}
            }
            self.speech.clear();
        }
    }

    /// Update the subtitle state.
    pub fn update(&mut self) {
        // We're done speaking but we have more to say.
        if !self.speech.is_empty() && !self.is_speaking() {
            // Remove the first element.
            self.speech.remove(0);
            // Start speaking the next element.
            if !self.speech.is_empty() {
                self.say(&self.speech[0].spoken.clone());
            }
        }
    }

    /// Returns the subtitles string of the current TTS string.
    pub fn get_subtitles(&self) -> Option<&str> {
        if self.speech.is_empty() {
            None
        } else {
            Some(&self.speech[0].seen)
        }
    }

    /// Returns true if Casey is speaking.
    fn is_speaking(&self) -> bool {
        match &self.tts {
            Some(tts) => tts.is_speaking().unwrap_or(false),
            None => false,
        }
    }

    /// Say something and show subtitles.
    fn say(&mut self, text: &str) {
        if let Some(tts) = &mut self.tts {
            if tts.speak(text, true).is_ok() {}
        }
    }
}

impl Enqueable<TtsString> for TTS {
    fn enqueue(&mut self, text: TtsString) {
        // Start speaking the first element.
        if !self.is_speaking() {
            self.say(&text.spoken)
        }
        // Push this element. We need it for subtitles.
        self.speech.push(text);
    }
}

impl Enqueable<String> for TTS {
    fn enqueue(&mut self, text: String) {
        self.enqueue(TtsString::from(text));
    }
}

impl Enqueable<&str> for TTS {
    fn enqueue(&mut self, text: &str) {
        self.enqueue(TtsString::from(text));
    }
}

impl Enqueable<Vec<TtsString>> for TTS {
    fn enqueue(&mut self, text: Vec<TtsString>) {
        if text.is_empty() {
            return;
        }
        // Start speaking the first element.
        if !self.is_speaking() {
            self.say(&text[0].spoken.clone())
        }
        self.speech.extend(text);
    }
}

/// This is something that can be enqueued into a vec of TTS strings.
pub trait Enqueable<T> {
    /// Enqueue something to the text-to-speech strings.
    fn enqueue(&mut self, text: T);
}

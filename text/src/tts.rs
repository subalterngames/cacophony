use common::config::{parse, parse_bool};
use common::ini::Ini;
use tts::Tts;
use crate::TtsString;

/// Text-to-speech.
pub struct TTS {
    /// The text-to-speech engine.
    tts: Option<Tts>,
    /// What text, if any, Casey is saying.
    pub speech: Option<TtsString>,
    /// If true, return subtitle text.
    show_subtitles: bool,
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
            speech: None,
        }
    }

    /// Say something. Optionally sets subtitles.
    pub fn say(&mut self, text: TtsString) {
        self.speech = None;
        if let Some(tts) = &mut self.tts {
            // Speak!
            if tts.speak(&text.spoken, true).is_ok() {
                // Remember what we're saying for subtitles.
                if self.show_subtitles {
                    self.speech = Some(text);
                }
            }
        }
    }

    pub fn say_str(&mut self, text: &str) {
        self.say(TtsString { spoken: text.to_string(), seen: text.to_string() })
    }

    /// Stop speaking.
    pub fn stop(&mut self) {
        if self.is_speaking() {
            if let Some(tts) = &mut self.tts {
                if tts.stop().is_ok() {}
            }
            self.speech = None;
        }
    }

    /// Update the subtitle state.
    pub fn update(&mut self) {
        if self.speech.is_some() && !self.is_speaking() {
            self.speech = None;
        }
    }

    /// Returns true if Casey is speaking.
    fn is_speaking(&self) -> bool {
        match &self.tts {
            Some(tts) => match tts.is_speaking() {
                Ok(speaking) => speaking,
                Err(_) => false,
            },
            None => false,
        }
    }
}

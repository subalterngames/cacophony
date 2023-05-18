use cacophony_core::config::{parse, parse_bool};
use ini::Ini;
use tts::Tts;

/// Text-to-speech.
pub struct TTS {
    /// The text-to-speech engine.
    tts: Option<Tts>,
    /// What text, if any, Casey is saying.
    speech: Option<String>,
    /// If true, return subtitle text.
    subtitles: bool,
}

impl TTS {
    pub fn new(config: &Ini) -> Self {
        let section = config.section(Some("TEXT_TO_SPEECH")).unwrap();
        // Get subtitles.
        let subtitles = parse_bool(section, "subtitles");
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
                if t.set_rate(parse(section, "rate")).is_ok() {}
                Some(t)
            }
            Err(_) => None,
        };
        Self {
            subtitles,
            tts,
            speech: None,
        }
    }

    /// Returns true if Casey can speak.
    pub fn can_speak(&self) -> bool {
        self.tts.is_some()
    }

    /// Say something. Optionally sets subtitles.
    pub fn say(&mut self, text: &str) {
        self.speech = None;
        if let Some(tts) = &mut self.tts {
            // Speak!
            if tts.speak(text, true).is_ok() {
                // Remember what we're saying for subtitles.
                if self.subtitles {
                    self.speech = Some(text.to_string());
                }
            }
        }
    }

    /// Stop speaking.
    pub fn stop_speaking(&mut self) {
        if let Some(tts) = &mut self.tts {
            if let Ok(speaking) = tts.is_speaking() {
                if speaking {
                    if tts.stop().is_ok() {}
                    self.speech = None;
                }
            }
        }
    }
}

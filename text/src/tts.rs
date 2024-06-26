use crate::TtsString;
use common::config::{parse, parse_bool};
use ini::Ini;
use tts::{Gender, Tts, Voice};

/// Text-to-speech.
pub struct TTS {
    /// The text-to-speech engine.
    tts: Option<Tts>,
    /// A queue of text-to-speech strings. Casey is saying the first element, if any.
    speech: Vec<TtsString>,
    /// If true, return subtitle text.
    pub show_subtitles: bool,
    /// If true, Casey is speaking.
    pub speaking: bool,
}

impl TTS {
    pub fn new(config: &Ini) -> Self {
        let section = config.section(Some("TEXT_TO_SPEECH")).unwrap();
        // Get subtitles.
        let show_subtitles = parse_bool(section, "subtitles");
        // Try to load the text-to-speech engine.
        let tts = match Tts::default() {
            Ok(mut tts) => {
                // Try to set the voice.
                if let Ok(voices) = tts.voices() {
                    // Try to parse the voice ID as an index.
                    let voice_id = section.get("voice_id").unwrap();
                    match voice_id.parse::<usize>() {
                        Ok(index) => if tts.set_voice(&voices[index]).is_ok() {},
                        // Try to parse the voice ID as a language.
                        Err(_) => {
                            let language = if cfg!(target_os = "linux") {
                                match voice_id.split('-').next() {
                                    Some(language) => language,
                                    None => voice_id,
                                }
                            } else {
                                voice_id
                            };
                            // Get all voices in this language.
                            let voices_lang: Vec<&Voice> =
                                voices.iter().filter(|v| v.language() == language).collect();
                            if voices_lang.is_empty() {
                                println!(
                                    "No voices found with language {}. Using the default instead.",
                                    voice_id
                                );
                                if tts.set_voice(&voices[0]).is_ok() {}
                            } else {
                                // Try to get the gender.
                                match section.get("gender") {
                                    Some(gender) => {
                                        // Convert the gender to an enum value.
                                        let gender = match gender {
                                            "f" => Some(Gender::Female),
                                            "m" => Some(Gender::Male),
                                            _ => None,
                                        };
                                        // Try to get the first voice.
                                        match voices_lang.iter().find(|v| v.gender() == gender) {
                                            // Set the first voice in this language with this gender.
                                            Some(voice) => if tts.set_voice(voice).is_ok() {},
                                            // Set the first voice in this language.
                                            None => if tts.set_voice(voices_lang[0]).is_ok() {},
                                        }
                                    }
                                    // Set the first voice in this language.
                                    None => if tts.set_voice(voices_lang[0]).is_ok() {},
                                }
                            }
                        }
                    }
                }
                // Try to set the rate.
                let rate_key = if cfg!(windows) {
                    "rate_windows"
                } else if cfg!(target_os = "macos") {
                    "rate_macos"
                } else {
                    "rate_linux"
                };
                let _ = tts.set_rate(parse(section, rate_key));
                Some(tts)
            }
            Err(error) => {
                println!("{}", error);
                None
            }
        };
        Self {
            show_subtitles,
            tts,
            speech: vec![],
            speaking: false,
        }
    }

    /// Stop speaking.
    pub fn stop(&mut self) {
        if self.speaking {
            if let Some(tts) = &mut self.tts {
                let _ = tts.stop();
                self.speaking = false;
            }
            self.speech.clear();
        }
    }

    /// Update the subtitle state.
    pub fn update(&mut self) {
        // We're done speaking but we have more to say.
        if !self.speech.is_empty() && !self.speaking {
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

    /// Say something and show subtitles.
    fn say(&mut self, text: &str) {
        if let Some(tts) = &mut self.tts {
            if tts.speak(text, true).is_ok() {
                self.speaking = true;
            }
        }
    }
}

impl Enqueable<TtsString> for TTS {
    fn enqueue(&mut self, text: TtsString) {
        // Start speaking the first element.
        if !self.speaking {
            self.say(&text.spoken)
        }
        // Push this element. We need it for subtitles.
        self.speech.push(text);
    }
}

impl Enqueable<&TtsString> for TTS {
    fn enqueue(&mut self, text: &TtsString) {
        // Start speaking the first element.
        if !self.speaking {
            self.say(&text.spoken)
        }
        // Push this element. We need it for subtitles.
        self.speech.push(text.clone());
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
        if !self.speaking {
            self.say(&text[0].spoken)
        }
        self.speech.extend(text);
    }
}

impl Enqueable<Vec<&TtsString>> for TTS {
    fn enqueue(&mut self, text: Vec<&TtsString>) {
        if text.is_empty() {
            return;
        }
        // Start speaking the first element.
        if !self.speaking {
            self.say(&text[0].spoken)
        }
        self.speech
            .extend(text.iter().map(|&t| t.clone()).collect::<Vec<TtsString>>());
    }
}

/// This is something that can be enqueued into a vec of TTS strings.
pub trait Enqueable<T> {
    /// Enqueue something to the text-to-speech strings.
    fn enqueue(&mut self, text: T);
}

#[cfg(test)]
mod tests {
    use crate::Enqueable;
    use common::get_test_config;

    use super::TTS;

    #[test]
    fn test_tts() {
        const TTS_STRING: &str = "Hello world!";

        let config = get_test_config();
        let mut tts = TTS::new(&config);
        assert!(tts.tts.is_some());
        tts.enqueue(TTS_STRING);
        assert_eq!(tts.speech.len(), 1);
        assert!(tts.show_subtitles);
        assert_eq!(tts.get_subtitles().unwrap(), TTS_STRING);
        tts.update();
        assert!(tts.speaking);
        tts.stop();
        tts.update();
        assert!(!tts.speaking);
    }
}

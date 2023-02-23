from typing import Optional
import pyttsx3
import pygame.mixer
from cacophony.paths import TEMP_DIRECTORY
from cacophony.util import get_config


class TextToSpeech:
    """
    Non-blocking text-to-speech player.

    This will save a wav file (ugh) and reload it (ugh) to play it with pygame.mixer (why??) so that it's non-blocking.
    """

    _INITIALIZED: bool = False
    _ENGINE: Optional[pyttsx3.Engine] = None
    _CHANNEL: Optional[pygame.mixer.Channel] = None
    _SOUND: Optional[pygame.mixer.Sound] = None
    _WAV_PATH: str = str(TEMP_DIRECTORY.joinpath("tts.wav").resolve())
    if not TEMP_DIRECTORY.exists():
        TEMP_DIRECTORY.mkdir(parents=True)

    @staticmethod
    def say(text: str) -> None:
        """
        Say something!

        :param text: Say this text.
        """

        if len(text) == 0:
            return
        # Initialize.
        if not TextToSpeech._INITIALIZED:
            if not pygame.mixer.get_init():
                pygame.mixer.init(allowedchanges=pygame.AUDIO_ALLOW_CHANNELS_CHANGE)
            TextToSpeech._INITIALIZED = True
            # Initialize text-to-speech.
            TextToSpeech._ENGINE = pyttsx3.init()
            # Get the voice ID.
            voice_id: int = int(get_config()["TEXT_TO_SPEECH"]["voice_id"])
            # Set the voice.
            TextToSpeech._ENGINE.setProperty("voice",
                                             TextToSpeech._ENGINE.getProperty('voices')[voice_id].id)
            TextToSpeech._ENGINE.setProperty("rate",
                                             int(get_config()["TEXT_TO_SPEECH"]["rate"]))
            # Set a channel.
            TextToSpeech._CHANNEL = pygame.mixer.find_channel()
        # Continue the current audio.
        TextToSpeech.stop()
        # Generate a temporary wav file. This is apparently the only way to get non-blocking text-to-speech audio!
        TextToSpeech._ENGINE.save_to_file(text, TextToSpeech._WAV_PATH)
        TextToSpeech._ENGINE.runAndWait()
        # Load the temporary wav file.
        TextToSpeech._SOUND = pygame.mixer.Sound(TextToSpeech._WAV_PATH)
        # Play the audio.
        TextToSpeech._CHANNEL.play(TextToSpeech._SOUND)

    @staticmethod
    def stop() -> None:
        """
        :return: Stop ongoing speech.
        """

        if TextToSpeech._CHANNEL.get_busy():
            TextToSpeech._CHANNEL.stop()

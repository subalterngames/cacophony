[ -f /etc/os-release ]
. /etc/os-release
INSTALL=
SPEECH_DISPATCHER=
if [ "$NAME" = "Debian GNU/Linux" ]; then
    INSTALL="apt install clang cmake speech-dispatcher libspeechd-dev pkg-config libssl-dev librust-alsa-sys-dev"
    if [ "$VERSION_ID" = "11" ]; then
        SPEECH_DISPATCHER="9"
    elif [ "$VERSION_ID" = "12" ]; then
        SPEECH_DISPATCHER="11"
    else
        echo "Invalid version: Debian $VERSION_ID"
    fi
fi
sudo $INSTALL
cargo build --release --features speech_dispatcher_0_$SPEECH_DISPATCHER

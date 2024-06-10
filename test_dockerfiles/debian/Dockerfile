ARG DISTRO

ARG VERSION=

FROM debian:bookworm AS debian12

ENV SPEECH_DISPATCHER=11

FROM debian:bullseye AS debian11

ENV SPEECH_DISPATCHER=9

FROM debian${VERSION}

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR $HOME/cacophony

COPY Cargo.toml Cargo.toml

COPY Cargo.lock Cargo.lock

COPY audio/ audio/

COPY common/ common/

COPY data/ data/

COPY input/ input/

COPY io/ io/

COPY render/ render/

COPY src/ src/

COPY test_files/ test_files/

COPY text/ text/

RUN mkdir cacophony && \
apt update && \
apt install clang cmake curl speech-dispatcher libspeechd-dev pkg-config libssl-dev librust-alsa-sys-dev -y && \
curl https://sh.rustup.rs -sSf | bash -s -- -y

ENTRYPOINT cargo test --all --features speech_dispatcher_0_${SPEECH_DISPATCHER}

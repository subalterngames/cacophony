FROM archlinux:latest

ENV PATH="/root/.cargo/bin:${PATH}"

RUN mkdir cacophony

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

COPY text/ text/

RUN pacman -Syu --noconfirm clang make pkgconfig rust speech-dispatcher alsa-lib openssl

RUN cargo build --features speech_dispatcher_0_11
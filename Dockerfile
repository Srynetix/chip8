FROM ubuntu:focal

RUN apt-get update \
    && apt-get install -y \
        gcc \
        libasound2-dev \
        libgl-dev \
        libx11-dev \
        libxi-dev \
        wget \
    && (wget -q -O - https://sh.rustup.rs | bash -s -- --default-toolchain none -y) \
    && rm -rf /var/lib/apt/lists/*

RUN bash -c "source $HOME/.cargo/env && rustup toolchain install nightly-2021-09-11-x86_64-unknown-linux-gnu"

WORKDIR /app
ADD . /app
RUN bash -c "source $HOME/.cargo/env && cargo build --all"
ENTRYPOINT [ "/app/target/debug/chip8-gui"]

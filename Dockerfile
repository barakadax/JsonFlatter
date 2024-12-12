FROM rust:1.83.0-slim-bookworm@sha256:c5bf976be6d358b7dc6113fe0ef179077244dff8fdd9c3bec1bcd14677d1f902  

RUN apt update -y && \
    apt upgrade -y && \
    apt clean && \
    apt-get update -y && \
    apt-get upgrade -y && \
    apt-get clean && \
    apt install libssl-dev -y && \
    apt install pkg-config -y && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . .

EXPOSE 1234

RUN cargo build --release && \
    rm -rf src Cargo.toml Cargo.lock

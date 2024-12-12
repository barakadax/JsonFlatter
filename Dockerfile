FROM rust:1.83.0-slim-bookworm@sha256:c5bf976be6d358b7dc6113fe0ef179077244dff8fdd9c3bec1bcd14677d1f902  

RUN apt update -y
RUN apt upgrade -y
RUN apt-get update -y
RUN apt-get upgrade -y
RUN apt install libssl-dev -y
RUN apt install pkg-config -y

WORKDIR /app

COPY . .

CMD ["cargo", "build"]

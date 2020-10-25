FROM rust:1.46.0 AS build
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup component add rustfmt --toolchain 1.46.0-x86_64-unknown-linux-gnu
RUN apt -y update && apt -y install llvm clang

RUN USER=root cargo new rockd-service
WORKDIR /usr/src/rockd-service
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo install --target x86_64-unknown-linux-gnu --path .
CMD ["rockd-service"]

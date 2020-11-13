FROM rust:1.46.0 AS build
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup component add rustfmt --toolchain 1.46.0-x86_64-unknown-linux-gnu

RUN USER=root cargo new nnout
WORKDIR /usr/src/nnout
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo install --target x86_64-unknown-linux-gnu --path .
CMD ["nnout"]

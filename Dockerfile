FROM rust:1.55 AS build

WORKDIR /usr/radon

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root

# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps
COPY Cargo.toml ./
RUN rustup override set nightly
COPY ./src ./src
RUN cargo build --release

# RUN cargo install --target x86_64-unknown-linux-musl --path .
RUN rustup target add x86_64-unknown-linux-musl

FROM scratch
COPY --from=build /usr/radon/target/release/radon .
USER 1000
ENTRYPOINT ["./radon"]

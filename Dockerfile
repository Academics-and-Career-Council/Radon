FROM rust:1.55 AS build

WORKDIR /usr/radon

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root

# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps
COPY Cargo.toml Cargo.lock ./ /usr/radon/
RUN rustup override set nightly
RUN cargo build --release

# Copy the source and build the application.
COPY ./src ./src

# RUN cargo install --target x86_64-unknown-linux-musl --path .
RUN rustup target add x86_64-unknown-linux-musl

FROM blockloop/nginx-scratch:latest
COPY nginx.conf /usr/local/nginx/conf/
COPY --from=build /usr/radon/target/release/radon .
COPY docker-entrypoint.sh .
RUN ["chmod", "+x", "docker-entrypoint.sh"]
ENTRYPOINT ["docker-entrypoint.sh"]
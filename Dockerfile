FROM ekidd/rust-musl-builder:nightly-2021-02-13 AS build

ADD --chown=rust:rust . ./

RUN cargo build --release

# Copy the source and build the application.
COPY ./src ./src

# RUN cargo install --target x86_64-unknown-linux-musl --path .
RUN rustup target add x86_64-unknown-linux-musl

FROM blockloop/nginx-scratch:latest
COPY nginx.conf /usr/local/nginx/conf/
COPY --from=build \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/radon \
    /usr/local/bin/
COPY docker-entrypoint.sh .
RUN ["chmod", "+x", "docker-entrypoint.sh"]
ENTRYPOINT ["docker-entrypoint.sh"]


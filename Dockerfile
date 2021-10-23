FROM ekidd/rust-musl-builder:nightly-2021-02-13 AS build

ADD --chown=rust:rust . ./

RUN cargo build --release

FROM scratch
COPY --from=build \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/radon \
    /usr/local/bin/
EXPOSE 8000
ENTRYPOINT [ "/usr/local/bin/radon" ]

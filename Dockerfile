FROM rust:1-alpine3.16

# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN apk add --no-cache musl-dev

WORKDIR /app
COPY ./ /app

RUN cargo build --release
RUN strip target/release/mailtutan

FROM alpine:3.16
RUN apk add --no-cache libgcc

COPY --from=0 /app/target/release/mailtutan .

ENTRYPOINT ["/mailtutan"]

# fetch the vendor with the builder platform to avoid qemu issues
FROM --platform=$BUILDPLATFORM rust:1-alpine3.16 AS vendor

WORKDIR /code
RUN cargo init
RUN cargo init --bin mailtutan
RUN cargo init --lib mailtutan-lib
RUN cargo init --lib mailtutan-web
COPY mailtutan/Cargo.toml /code/mailtutan/Cargo.toml
COPY mailtutan-lib/Cargo.toml /code/mailtutan-lib/Cargo.toml
COPY mailtutan-web/Cargo.toml /code/mailtutan-web/Cargo.toml
COPY mailtutan-web/Cargo.lock /code/mailtutan-web/Cargo.lock
COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock

# https://docs.docker.com/engine/reference/builder/#run---mounttypecache
RUN --mount=type=cache,target=$CARGO_HOME/git,sharing=locked \
  --mount=type=cache,target=$CARGO_HOME/registry,sharing=locked \
  mkdir -p /code/.cargo \
  && cargo vendor > /code/.cargo/config.toml

FROM rust:1-alpine3.16 AS builder

RUN apk add --no-cache musl-dev

ENV USER=root

WORKDIR /code

COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock
COPY mailtutan /code/mailtutan
COPY mailtutan-lib /code/mailtutan-lib
COPY mailtutan-web /code/mailtutan-web
COPY --from=vendor /code/.cargo /code/.cargo
COPY --from=vendor /code/vendor /code/vendor

RUN cargo build --release --offline
RUN strip /code/target/release/mailtutan

FROM gcr.io/distroless/static-debian11
FROM alpine:3.16

COPY --from=builder /code/target/release/mailtutan /mailtutan

CMD ["/mailtutan"]

FROM rust:1.70-alpine3.18 as build

WORKDIR /usr/src/sheef-planing
COPY . .

RUN apk add musl-dev
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

WORKDIR /usr/src/sheef-planing/rusty
RUN trunk build --release


WORKDIR /usr/src/sheef-planing
RUN cargo install --path .

FROM alpine:3.18

COPY --from=build /usr/local/cargo/bin/sheef_planing /usr/local/bin/sheef-planing

CMD ["sheef-planing"]
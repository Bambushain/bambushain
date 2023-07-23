FROM rust:1.70-alpine3.18 as build

WORKDIR /usr/src/sheef-planing
COPY . .

RUN apk add musl-dev
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN trunk build --release
RUN cargo install --path .

FROM alpine:3.18

ENV FRONTEND_DIR=/usr/local/share/sheef-planing/web/

COPY --from=build /usr/src/sheef-planing/dist /usr/local/share/sheef-planing/web/dist
COPY --from=build /usr/local/cargo/bin/sheef-planing /usr/local/bin/sheef-planing

CMD ["sheef-planing"]
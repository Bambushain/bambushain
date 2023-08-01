FROM rust:1.70-alpine3.18 as build

WORKDIR /usr/src/pandaparty
COPY . .

RUN apk add musl-dev
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN trunk build --release
RUN cargo install --path .

FROM alpine:3.18

ENV FRONTEND_DIR=/usr/local/share/pandaparty/web/

COPY --from=build /usr/src/pandaparty/dist /usr/local/share/pandaparty/web/dist
COPY --from=build /usr/local/cargo/bin/pandaparty /usr/local/bin/pandaparty

CMD ["pandaparty"]
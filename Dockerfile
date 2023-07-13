FROM rust:1.70-alpine3.18 as build

WORKDIR /usr/src/sheef-planing
COPY . .

RUN apk add musl-dev
RUN cargo install --path .

FROM alpine:3.18

COPY --from=build /usr/local/cargo/bin/sheef_planing /usr/local/bin/sheef-planing

CMD ["sheef-planing"]
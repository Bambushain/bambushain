FROM registry.ulbricht.casa/docker-images/rust-docker-base-image:latest AS backend-api

COPY . /build/bamboo

WORKDIR /build/bamboo

RUN cargo build --bin backend-api --features backend-api --release

FROM library/alpine:latest

COPY --from=backend-api /build/bamboo/target/release/backend-api /backend-api

CMD ["/backend-api"]

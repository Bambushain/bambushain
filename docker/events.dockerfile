FROM registry.ulbricht.casa/docker-images/rust-docker-base-image:latest AS backend-events

COPY . /build/bamboo

WORKDIR /build/bamboo

RUN cargo build --bin backend-events --features backend-events --release

FROM library/alpine:latest

COPY --from=backend-events /build/bamboo/target/release/backend-events /backend-events

CMD ["/backend-events"]

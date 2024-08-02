FROM registry.ulbricht.casa/docker-images/rust-docker-base-image:latest AS backend-mailer

COPY . /build/bamboo

WORKDIR /build/bamboo

RUN cargo build --bin backend-mailer --features backend-mailer --release

FROM library/alpine:latest

COPY --from=backend-mailer /build/bamboo/target/release/backend-mailer /backend-mailer

CMD ["/backend-mailer"]

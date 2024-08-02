FROM registry.ulbricht.casa/docker-images/rust-docker-base-image:latest AS backend-pandas

COPY . /build/bamboo

WORKDIR /build/bamboo

RUN cargo build --bin backend-pandas --features backend-pandas --release

FROM registry.ulbricht.casa/docker-images/trunk-docker-base-image:latest AS frontend-pandas

COPY . /build/bamboo

WORKDIR /build/bamboo

RUN trunk build --config frontend-pandas.toml --release

FROM library/alpine:latest

ENV FRONTEND_DIR=/bamboo/pandas/

COPY --from=frontend-pandas dist-pandas /bamboo/pandas
COPY --from=backend-pandas /build/bamboo/target/release/backend-pandas /backend-pandas

CMD ["/backend-pandas"]

FROM harbor.ulbricht.casa/imanuel/trunk-docker-base-image:latest as build

WORKDIR /usr/src/bamboo
COPY . .

RUN cargo install --path .

WORKDIR /usr/src/bamboo/frontend
RUN trunk build --release

FROM library/alpine:3.19

ENV FRONTEND_DIR=/usr/local/share/bamboo/web/

COPY --from=build /usr/src/bamboo/dist /usr/local/share/bamboo/web/dist
COPY --from=build /usr/local/cargo/bin/bamboo /usr/local/bin/bamboo

CMD ["bamboo"]

FROM harbor.ulbricht.casa/imanuel/trunk-docker-base-image:latest as build

WORKDIR /usr/src/bamboo
COPY . .

RUN trunk build --release
RUN cargo install --path .

FROM library/alpine:3.18

ENV FRONTEND_DIR=/usr/local/share/bamboo/web/

COPY --from=build /usr/src/bamboo/dist /usr/local/share/bamboo/web/dist
COPY --from=build /usr/local/cargo/bin/bamboo /usr/local/bin/bamboo

CMD ["bamboo"]

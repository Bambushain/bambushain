FROM harbor.ulbricht.casa/imanuel/trunk-docker-base-image:latest as build

WORKDIR /usr/src/pandaparty
COPY . .

RUN trunk build --release
RUN cargo install --path .

FROM harbor.ulbricht.casa/proxy/library/alpine:3.18

ENV FRONTEND_DIR=/usr/local/share/pandaparty/web/

COPY --from=build /usr/src/pandaparty/dist /usr/local/share/pandaparty/web/dist
COPY --from=build /usr/local/cargo/bin/pandaparty /usr/local/bin/pandaparty

CMD ["pandaparty"]

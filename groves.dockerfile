FROM library/alpine:latest

ENV FRONTEND_DIR=/usr/local/share/groves-server/web/

RUN mkdir -p /usr/local/share/groves-server/web/
RUN cp -r /builds/creastina/bambushain/dist-groves /usr/local/share/groves-server/web/dist
RUN cp -r /builds/creastina/bambushain/target/release/groves-server /usr/local/bin/groves-server

CMD ["groves-server"]

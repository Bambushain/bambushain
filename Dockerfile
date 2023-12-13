FROM library/alpine:latest

ENV FRONTEND_DIR=/usr/local/share/bamboo/web/

COPY /builds/creastina/bambushain/dist /usr/local/share/bamboo/web/dist
COPY /builds/creastina/bambushain/target/release/bamboo /usr/local/bin/bamboo

CMD ["bamboo"]

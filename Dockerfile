FROM library/alpine:latest

ENV FRONTEND_DIR=/usr/local/share/bamboo/web/

RUN ls -la

COPY dist /usr/local/share/bamboo/web/dist
COPY target/release/bamboo /usr/local/bin/bamboo

CMD ["bamboo"]

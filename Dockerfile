FROM library/alpine:latest

ENV FRONTEND_DIR=/usr/local/share/bamboo/web/

COPY dist /usr/local/share/bamboo/web/dist
COPY bamboo /usr/local/bin/bamboo

CMD ["bamboo"]

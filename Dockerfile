FROM library/alpine:latest

ENV FRONTEND_DIR=/usr/local/share/bamboo/web/

RUN mkdir -p /usr/local/share/bamboo/web/
RUN cp -r /builds/creastina/bambushain/dist /usr/local/share/bamboo/web/dist
RUN cp -r /builds/creastina/bambushain/target/release/bamboo /usr/local/bin/bamboo

CMD ["bamboo"]

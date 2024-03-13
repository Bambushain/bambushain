FROM library/alpine:latest

ENV FRONTEND_DIR=/usr/local/share/pandas-server/web/

RUN mkdir -p /usr/local/share/pandas-server/web/
RUN cp -r /builds/creastina/bambushain/dist-pandas /usr/local/share/pandas-server/web/dist
RUN cp -r /builds/creastina/bambushain/arm64/pandas-server /usr/local/bin/pandas-server

CMD ["pandas-server"]

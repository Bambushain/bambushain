FROM library/alpine:latest

ENV FRONTEND_DIR=/usr/local/share/pandas-server/web/

RUN mkdir -p /usr/local/share/pandas-server/web/
RUN cp -r /builds/creastina/bambushain/pandas/dist /usr/local/share/pandas-server/web/dist
RUN cp -r /builds/creastina/bambushain/pandas/target/release/pandas-server /usr/local/bin/pandas-server

CMD ["pandas-server"]

FROM redocly/redoc:v2.0.0

COPY docs.yaml /usr/share/nginx/html/openapi.yaml
COPY favicon.png /usr/share/nginx/html/favicon.png
ENV SPEC_URL openapi.yaml
ENV PAGE_TITLE "Sheef Planing OpenAPI"

EXPOSE 80
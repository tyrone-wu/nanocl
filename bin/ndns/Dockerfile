FROM alpine:3.17.2

RUN apk --update add dnsmasq \
  && rm -rf /var/cache/apk/* \
  && rm -rf /tmp/* \
  && rm -rf /var/log/* \
  && rm -rf /var/tmp/*

EXPOSE 53/tcp 53/udp

COPY ./bin/ndns/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

LABEL org.opencontainers.image.source https://github.com/nxthat/nanocl
LABEL org.opencontainers.image.description Nanocl Dns

COPY ./bin/ndns/dnsmasq.conf /dnsmasq.conf

ENTRYPOINT ["/entrypoint.sh"]

FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

RUN set -x \
    && apt update \
    && apt install -y npm \
    && npm install --global xpm@0.5.0 \
    && xpm install --global @xpack-dev-tools/qemu-arm@2.8.0-8.1

RUN ln -s `find /root -name qemu-system-gnuarmeclipse` /usr/local/bin/

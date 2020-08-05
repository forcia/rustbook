FROM ubuntu:20.04

RUN apt-get update
RUN apt-get install -y git sudo curl file gcc

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN . $HOME/.cargo/env \
    && rustup target add x86_64-unknown-linux-musl \
    && export USER=root \
    && cargo --version \
    && cargo new hello \
    && cd hello \
    && cargo build \
    && cargo build --target=x86_64-unknown-linux-musl

FROM centos:7

RUN mkdir -p target/debug/
RUN mkdir -p target/x86_64-unknown-linux-musl/debug/
COPY --from=0 /hello/target/debug/hello ./target/debug/hello
COPY --from=0 /hello/target/x86_64-unknown-linux-musl/debug/hello ./target/x86_64-unknown-linux-musl/debug/hello
RUN ./target/debug/hello; exit 0
RUN ./target/x86_64-unknown-linux-musl/debug/hello

FROM alpine:3.11

RUN mkdir -p target/debug/
RUN mkdir -p target/x86_64-unknown-linux-musl/debug/
COPY --from=0 /hello/target/debug/hello ./target/debug/hello
COPY --from=0 /hello/target/x86_64-unknown-linux-musl/debug/hello ./target/x86_64-unknown-linux-musl/debug/hello
RUN ./target/debug/hello;  exit 0
RUN ldd ./target/debug/hello; exit 0
RUN ./target/x86_64-unknown-linux-musl/debug/hello

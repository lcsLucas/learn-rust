##### Builder
FROM rust:1.69.0-slim

ARG user=lucas
ARG uid=1000

WORKDIR /usr/rust/src

RUN adduser --disabled-password --gecos '' --uid ${uid} --shell /bin/sh ${user}

USER ${user}

ENV HOME /home/${user}
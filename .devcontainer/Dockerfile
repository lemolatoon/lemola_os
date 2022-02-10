FROM rust:latest 

RUN apt-get update -y
RUN apt-get upgrade -y

RUN rustup update

RUN rustup install nightly

RUN rustup default nightly

RUN rustup --version

WORKDIR /lemola_os

RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

# RUN cargo build -Z build-std

RUN rustup component add rustfmt
RUN rustup component add clippy


RUN apt-get install bsdmainutils
RUN apt-get install less

RUN apt-get install qemu-kvm -y

ENV DISPLAY=host.docker.internal:0


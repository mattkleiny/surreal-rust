FROM mcr.microsoft.com/devcontainers/rust:latest

RUN apt-get update && apt-get -y install libgl1-mesa-dev libasound2-dev
RUN rustup component add --toolchain nightly-x86_64-unknown-linux-gnu rustfmt

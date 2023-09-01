FROM mcr.microsoft.com/devcontainers/rust:latest

RUN sudo apt-get update &&
    sudo apt-get -y install --no-install-recommends \
    libgl1-mesa-dev \
    libasound2-dev \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

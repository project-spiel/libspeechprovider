FROM ubuntu:lunar

ARG DEBIAN_FRONTEND=noninteractive
ARG USERNAME=ubuntu

RUN apt-get update \
    && apt-get install -y \
    git \
    sudo \
    meson \
    gi-docgen \
    libgirepository1.0-dev \
    python3-tap \
    python3-gi \
    && apt-get clean -y \
    && apt-get autoremove -y \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

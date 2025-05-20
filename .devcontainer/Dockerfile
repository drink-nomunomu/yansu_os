FROM --platform=linux/amd64 mirror.gcr.io/library/ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH
ENV DISPLAY=host.docker.internal:0

# 必要なパッケージをまとめてインストール（X11とQEMUのGUI関連パッケージを追加）
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    git \
    build-essential \
    clang \
    pkg-config \
    python3-minimal \
    python3-venv \
    python3-pip \
    ninja-build \
    meson \
    wget \
    netcat-openbsd \
    libglib2.0-dev \
    libpixman-1-dev \
    libslirp-dev \
    libgtk-3-dev \
    libsdl2-dev \
    libspice-server-dev \
    libx11-dev \
    libxcursor-dev \
    xauth \
    && rm -rf /var/lib/apt/lists/*

# Rustのインストール（必要最小限のコンポーネント）
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path --profile minimal && \
    rm -rf $RUSTUP_HOME/downloads && \
    rm -rf $RUSTUP_HOME/tmp

# QEMU 9.0.0のインストール（グラフィック対応）
WORKDIR /tmp
RUN wget https://download.qemu.org/qemu-9.0.0.tar.xz && \
    tar xJf qemu-9.0.0.tar.xz && \
    cd qemu-9.0.0 && \
    mkdir build && cd build && \
    ../configure --target-list=x86_64-softmmu --enable-slirp \
    --disable-docs --disable-guest-agent --disable-guest-agent-msi \
    --enable-gtk --enable-sdl --enable-vnc \
    && ninja && \
    ninja install && \
    cd / && \
    rm -rf /tmp/qemu-9.0.0 /tmp/qemu-9.0.0.tar.xz && \
    apt-get clean && \
    apt-get autoremove -y

# ワークスペースの設定
WORKDIR /workspace
CMD ["bash"]

FROM --platform=linux/amd64 mirror.gcr.io/library/ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

# 必要なパッケージをまとめてインストール
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    git \
    build-essential \
    clang \
    pkg-config \
    python3 \
    python3-venv \
    python3-pip \
    sudo \
    passwd \
    ninja-build \
    meson \
    wget \
    libglib2.0-dev \
    libpixman-1-dev \
    libslirp-dev \
    && rm -rf /var/lib/apt/lists/*

# Rustのインストール
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path --default-toolchain stable && \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME

# QEMU 9.0.0のインストール
WORKDIR /tmp
RUN wget https://download.qemu.org/qemu-9.0.0.tar.xz && \
    tar xJf qemu-9.0.0.tar.xz && \
    cd qemu-9.0.0 && \
    mkdir build && cd build && \
    ../configure --target-list=x86_64-softmmu --enable-slirp \
    --disable-docs --disable-guest-agent --disable-guest-agent-msi \
    --disable-gtk --disable-opengl && \
    ninja && \
    ninja install && \
    cd /tmp && \
    rm -rf qemu-9.0.0 qemu-9.0.0.tar.xz && \
    apt-get clean

# vscodeユーザーの作成
RUN groupadd -g 2000 vscode && \
    useradd -m -s /bin/bash -u 2000 -g 2000 -G sudo vscode && \
    echo "vscode ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/vscode && \
    chmod 0440 /etc/sudoers.d/vscode

WORKDIR /workspace
USER vscode
CMD ["bash"]
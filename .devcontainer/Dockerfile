FROM --platform=linux/amd64 ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

# vscode user作成
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=${USER_UID}
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME 

# 必要なパッケージのインストール
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
    libglib2.0-dev \
    libpixman-1-dev \
    libslirp-dev \
    && rm -rf /var/lib/apt/lists/*

# Rustのインストール
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path --default-toolchain stable && \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME

# QEMU 9.0.0のインストール（必要最小限の設定）
WORKDIR /tmp
RUN apt-get update && \
    apt-get install -y --no-install-recommends wget && \
    wget https://download.qemu.org/qemu-9.0.0.tar.xz && \
    tar xJf qemu-9.0.0.tar.xz && \
    cd qemu-9.0.0 && \
    ./configure --target-list=x86_64-softmmu --enable-slirp --disable-docs --disable-guest-agent --disable-guest-agent-msi --disable-gtk --disable-opengl && \
    make -j$(nproc) && \
    make install && \
    cd .. && \
    rm -rf qemu-9.0.0 qemu-9.0.0.tar.xz && \
    apt-get purge -y wget && \
    apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /workspace
USER vscode
CMD ["bash"]

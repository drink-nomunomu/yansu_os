FROM --platform=linux/amd64 ubuntu:24.04
# 環境変数の設定
ENV DEBIAN_FRONTEND=noninteractive
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH
# パッケージリストの更新とタイムゾーン設定
RUN apt-get update && \
    apt-get install -y --no-install-recommends tzdata && \
    ln -fs /usr/share/zoneinfo/Asia/Tokyo /etc/localtime && \
    dpkg-reconfigure --frontend noninteractive tzdata
# 基本的なツールのインストール
RUN apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    wget \
    git \
    build-essential \
    pkg-config \
    python3 \
    python3-pip \
    ninja-build \
    libglib2.0-dev \
    libpixman-1-dev \
    libslirp-dev \
    libnfs-dev \
    libssh-dev \
    libfdt-dev \
    libgtk-3-dev \
    libaio-dev \
    libseccomp-dev \
    libcap-ng-dev \
    libusb-1.0-0-dev \
    libsnappy-dev \
    libzstd-dev \
    libbz2-dev \
    liblzo2-dev \
    zlib1g-dev \
    libnuma-dev \
    libjpeg-dev \
    libssl-dev \
    libncurses-dev \
    && rm -rf /var/lib/apt/lists/*
# Rustのインストール
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path --default-toolchain stable && \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME
# QEMU 9.0.0のインストール（ソースからビルド）
WORKDIR /tmp
RUN wget https://download.qemu.org/qemu-9.0.0.tar.xz && \
    tar xvJf qemu-9.0.0.tar.xz && \
    cd qemu-9.0.0 && \
    ./configure --target-list=x86_64-softmmu,aarch64-softmmu,arm-softmmu --enable-slirp --enable-gtk && \
    make -j$(nproc) && \
    make install && \
    cd .. && \
    rm -rf qemu-9.0.0 qemu-9.0.0.tar.xz
# cargo-watchとその他便利なツールのインストール
RUN cargo install cargo-watch cargo-edit cargo-audit cargo-expand
# ボリュームマウントポイントの作成
RUN mkdir -p /mnt/volume
# 作業ディレクトリの設定
WORKDIR /workspace
# デフォルトコマンド
CMD ["bash"]

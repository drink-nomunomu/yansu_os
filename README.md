# yansu_os

[［作って学ぶ］ OSのしくみ⁠─メモリ管理、マルチタスク、ハードウェア制御](https://gihyo.jp/book/2025/978-4-297-14859-1)で必要なライブラリをインストールした諸々

## devcontainer起動方法

```
cd yansu_os
```
vscodeのコマンドパレット(cmd + shift+ p)から「コンテナで再度開く」を選択

### install tools

```
# cargo --version
cargo 1.86.0 (adf9b6ad1 2025-02-28)

# rustc --version
rustc 1.86.0 (05f9846f8 2025-03-31)

# rustup --version
rustup 1.28.2 (e4f3ad6f8 2025-04-28)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.86.0 (05f9846f8 2025-03-31)`

# make --version
GNU Make 4.3
Built for x86_64-pc-linux-gnu
Copyright (C) 1988-2020 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

# clang --version
Ubuntu clang version 18.1.3 (1ubuntu1)
Target: x86_64-pc-linux-gnu
Thread model: posix
InstalledDir: /usr/bin

# nc
usage: nc [-46CDdFhklNnrStUuvZz] [-I length] [-i interval] [-M ttl]
          [-m minttl] [-O length] [-P proxy_username] [-p source_port]
          [-q seconds] [-s sourceaddr] [-T keyword] [-V rtable] [-W recvlimit]
          [-w timeout] [-X proxy_protocol] [-x proxy_address[:port]]
          [destination] [port]

# qemu-system-x86_64 --version
QEMU emulator version 9.0.0
Copyright (c) 2003-2024 Fabrice Bellard and the QEMU Project developers
```

### qemuコマンドをインストールする手順

```
$ brew install --cask xquartz
```
ログインし直す
```
$ echo $DISPLAY
/private/tmp/com.apple.launchd.09tYobfzHt/org.xquartz:0
```
と表示されること

xquartzの設定で「接続を認証」「ネットワーク・クライアントからの接続を許可」をクリックしておく

```
$ HOST_IP=$(ifconfig en0 | grep inet | awk '$1=="inet" {print $2}')
```
```
$ xhost + $HOST_IP
```



### 備考
- 2GB程度のコンテナイメージ



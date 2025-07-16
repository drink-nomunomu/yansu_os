# YansuOS Documentation

YansuOSは、Rustで記述されたUEFIベースの実験的オペレーティングシステムです。

## ドキュメント構成

- [概要](overview.md) - YansuOSの基本概念と目的
- [アーキテクチャ](architecture.md) - システム構成と設計思想
- [コード解説](code-walkthrough.md) - ソースコードの詳細解説
- [ビルド・開発ガイド](development.md) - 開発環境のセットアップと使用方法

## クイックスタート

YansuOSをビルドして実行するには：

```bash
cd repo/yansu
cargo build --target x86_64-unknown-uefi
./scripts/launch_qemu.sh target/x86_64-unknown-uefi/debug/yansu.efi
```
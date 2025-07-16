# YansuOS 開発ガイド

## 開発環境のセットアップ

### 必要なソフトウェア

1. **Rust開発環境**
   ```bash
   # Rustのインストール（rustup経由）
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # UEFIターゲットの追加
   rustup target add x86_64-unknown-uefi
   ```

2. **QEMU（エミュレータ）**
   ```bash
   # macOSの場合
   brew install qemu
   
   # Ubuntuの場合
   sudo apt install qemu-system-x86
   ```

3. **その他のツール**
   - Git（ソースコード管理）
   - エディタ（VS Code + rust-analyzer推奨）

### プロジェクト構造

```
yansu_os/
├── repo/
│   └── yansu/              # メインプロジェクト
│       ├── Cargo.toml      # プロジェクト設定
│       ├── src/            # ソースコード
│       ├── scripts/        # ビルド・実行スクリプト
│       ├── third_party/    # サードパーティライブラリ
│       └── target/         # ビルド出力
└── docs/                   # ドキュメント
```

## ビルドとテスト

### 基本的なビルド

```bash
cd repo/yansu

# デバッグビルド
cargo build --target x86_64-unknown-uefi

# リリースビルド
cargo build --target x86_64-unknown-uefi --release
```

### QEMUでの実行

```bash
# デバッグビルドの実行
./scripts/launch_qemu.sh target/x86_64-unknown-uefi/debug/yansu.efi

# リリースビルドの実行
./scripts/launch_qemu.sh target/x86_64-unknown-uefi/release/yansu.efi
```

### テストの実行

```bash
# ユニットテストの実行
cargo test --target x86_64-unknown-uefi
```

## 開発フロー

### 1. コードの変更

ソースコードを編集後、以下の手順で確認：

```bash
# 構文チェック
cargo check --target x86_64-unknown-uefi

# ビルド
cargo build --target x86_64-unknown-uefi

# 実行テスト
./scripts/launch_qemu.sh target/x86_64-unknown-uefi/debug/yansu.efi
```

### 2. デバッグ

デバッグ情報は以下の方式で出力されます：

- **シリアル出力**: `log/com1.txt`にログが保存
- **画面出力**: QEMU画面に直接表示
- **マクロ**: `println!`, `info!`, `warn!`, `error!`を使用

### 3. ログの確認

```bash
# リアルタイムでログを確認
tail -f log/com1.txt

# ログファイルの内容表示
cat log/com1.txt
```

## 設定ファイル

### Cargo.toml

プロジェクトの基本設定：

```toml
[package]
name = "yansu"
version = "0.1.0"
edition = "2021"

[dependencies]

[[bin]]
name = "yansu"
test = false
```

### rust-toolchain.toml

Rustツールチェーンの設定（プロジェクトルートに配置）

## コーディング規約

### 1. Rustの基本規約に従う

- `cargo fmt`でコードフォーマット
- `cargo clippy`で静的解析
- 適切なドキュメントコメントの記述

### 2. 安全性の考慮

- `unsafe`ブロックの使用は最小限に
- 適切なエラーハンドリング
- メモリ安全性の確保

### 3. モジュール分割

- 機能ごとにモジュールを分離
- 公開インターフェースの明確化
- 依存関係の最小化

## デバッグ技法

### 1. シリアル出力を活用

```rust
use yansu::println;
println!("Debug info: {:#x}", value);
```

### 2. hexdump機能の使用

```rust
use yansu::print::hexdump;
hexdump(&data);
```

### 3. グラフィカルデバッグ

画面に直接デバッグ情報を表示：

```rust
use yansu::uefi::VramTextWriter;
let mut writer = VramTextWriter::new(&mut vram);
writeln!(writer, "Debug: {}", value).unwrap();
```

## よくある問題と解決法

### 1. ビルドエラー

**問題**: ターゲットが見つからない
```
error: couldn't find a target to build
```

**解決**: UEFIターゲットを追加
```bash
rustup target add x86_64-unknown-uefi
```

### 2. QEMUが起動しない

**問題**: OVMFファームウェアが見つからない

**解決**: third_party/ovmfディレクトリにOVMFファイルが存在することを確認

### 3. 画面が表示されない

**問題**: グラフィック初期化に失敗

**解決**: 
- ログファイルでエラーメッセージを確認
- UEFI Graphics Output Protocolの利用可能性を確認

## 拡張開発

### 新機能の追加

1. 新しいモジュールの作成
2. `lib.rs`へのモジュール追加
3. 適切なテストの記述
4. ドキュメントの更新

### パフォーマンス最適化

- リリースビルドでの動作確認
- プロファイリングツールの活用
- メモリ使用量の監視

## トラブルシューティング

問題が発生した場合は以下を確認：

1. ログファイル（`log/com1.txt`）の内容
2. Cargoビルドエラーメッセージ
3. QEMUの出力メッセージ
4. システム要件の充足
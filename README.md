# axum playground

axum 検証用リポジトリ

## バージョン

- Rust: 1.54.0
- axum: 0.2.3

## ファイル構成

```sh
.
├── README.md
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain
├── src
│   ├── main.rs
│   ├── routes.rs
│   ├── controllers.rs
│   └── models.rs
└── target/
```

## ローカル環境構築

### パッケージ・ライブラリのインストール

```sh
# パッケージのインストール
cargo install --path .
# ライブリロード用のライブラリをインストールする
cargo install cargo-watch
```

### axum の起動

```sh
cargo watch -x run
```

### 動作確認

```sh
curl http://127.0.0.1:8000
```

## ドキュメント

- https://docs.rs/axum/

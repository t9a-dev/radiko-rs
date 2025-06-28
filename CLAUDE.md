# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

radiko-rsは、日本のラジオストリーミングサービス「radiko」へのアクセスを提供するRustライブラリです。HLSストリーミングのエンドポイントへのアクセスを認証トークンを用いて行います。

## Core Architecture

```
src/
├── lib.rs              # メインのライブラリエントリポイント
├── client.rs           # RadikoClient - HTTP認証付きクライアント
├── api/
│   ├── auth.rs         # RadikoAuthManager - 認証トークン管理
│   ├── endpoint.rs     # RadikoEndpoint - APIエンドポイント定義
│   ├── stream.rs       # RadikoStream - ストリーミング機能
│   └── program.rs      # 番組情報API（未実装）
└── traits/
    └── auth.rs         # 認証関連のトレイト定義
```

### Key Components

- **RadikoAuthManager**: radiko APIの2段階認証を処理し、X-Radiko-Authtokenを生成・管理
- **RadikoClient**: 認証済みHTTPクライアントを提供
- **RadikoEndpoint**: 各APIエンドポイントのURL生成を担当
- **RadikoStream**: HLSストリーミングURLの検証機能

### Authentication Flow

1. `auth1` APIでauthトークンとキー情報を取得
2. `playerCommon.js`から認証キーを抽出
3. `auth2` APIで部分キーを送信して認証完了
4. X-Radiko-Authtokenを含むHTTPクライアントを作成

## Common Commands

```bash
# Build the project
cargo build

# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Check for compilation errors without building
cargo check

# Format code (if rustfmt is available)
cargo fmt

# Lint code (if clippy is available)
cargo clippy
```

## Development Notes

- テストの多くは実際のradiko APIへのネットワークアクセスを行うため、ネットワーク接続が必要
- `memo.md`にはストリーミングエンドポイントの確認方法が記載されている
- `examples/radiko/`にHTTPリクエストの例が含まれている
- プロジェクトはRust 2024 editionを使用

## Key Dependencies

- `reqwest`: HTTP client
- `tokio`: Async runtime
- `anyhow`: Error handling
- `base64`: Base64 encoding for authentication
- `regex`: Pattern matching for auth key extraction

## Known Limitations

- `get_playlist_create_url_endpoint`は固定URLを返すが、radikoの仕様変更で定期的に変わる可能性がある
- 一部の機能（番組情報API等）は未実装
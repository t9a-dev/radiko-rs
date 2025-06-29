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
│   ├── program.rs      # 番組情報API
│   └── mod.rs          # APIモジュール定義
├── dto/
│   ├── program_xml.rs  # XML解析用データ構造
│   └── mod.rs          # DTOモジュール定義
├── models/
│   ├── program.rs      # 番組情報モデル
│   ├── search.rs       # 検索関連モデル
│   └── mod.rs          # モデルモジュール定義
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
- `tokio`: Async runtime with full features
- `anyhow`: Error handling
- `base64`: Base64 encoding for authentication
- `regex`: Pattern matching for auth key extraction
- `serde`/`serde_json`: Serialization/deserialization
- `quick-xml`: XML parsing with serialize feature
- `chrono`/`chrono-tz`: Date/time handling with timezone support
- `strum`: Enum utilities

## Project Structure Details

### Module Organization
- **api/**: radiko API との通信を処理するモジュール群
- **dto/**: XMLからデータを解析するためのデータ転送オブジェクト
- **models/**: アプリケーション内で使用されるデータモデル
- **client.rs**: 認証済みHTTPクライアントのラッパー

### Example Usage
`examples/radiko/`ディレクトリには実際のHTTP リクエスト例が含まれています：
- `auth.http`: 認証フロー
- `stream.http`: ストリーミングエンドポイント
- `search.http`: 番組検索
- `weekly_program.http`: 週間番組表

## Known Limitations

- `get_playlist_create_url_endpoint`は固定URLを返すが、radikoの仕様変更で定期的に変わる可能性がある
- `memo.md`に記載されている通り、ストリーミングエンドポイントの確認にはブラウザでの手動確認が必要
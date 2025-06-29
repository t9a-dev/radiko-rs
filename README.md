# WIP(作業中)

# radiko_rs

radikoの非公式Rust SDKです。

## 概要

radiko_rsは、日本のインターネットラジオサービス「radiko」のAPIにアクセスするためのRustライブラリです。認証トークンを用いてHLSストリーミングエンドポイントにアクセスし、番組情報の取得やストリーミングURLの生成を行います。

## 特徴

- 2段階認証による安全なAPI接続
- 番組情報の取得と検索
- HLSストリーミングURLの生成
- 非同期処理対応
- 型安全なAPI設計

## インストール

```toml
[dependencies]
radiko_rs = "0.1.0"
```

## 使用方法

### 基本的な使用例

```rust
use radiko_rs::{RadikoClient, RadikoAuthManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 認証マネージャーを初期化
    let auth_manager = RadikoAuthManager::new().await?;
    
    // 認証済みクライアントを作成
    let client = RadikoClient::new(auth_manager).await?;
    
    // 番組情報を取得
    let programs = client.get_weekly_program("TBS").await?;
    
    Ok(())
}
```

### ストリーミングURLの取得

```rust
use radiko_rs::RadikoStream;

let stream = RadikoStream::new(client);
let stream_url = stream.get_stream_url("TBS").await?;
println!("ストリーミングURL: {}", stream_url);
```

## アーキテクチャ

- **RadikoAuthManager**: radiko APIの2段階認証を処理
- **RadikoClient**: 認証済みHTTPクライアント
- **RadikoEndpoint**: APIエンドポイントのURL生成
- **RadikoStream**: HLSストリーミング機能

## 開発

```bash
# プロジェクトをビルド
cargo build

# テストを実行
cargo test

# コードの整形
cargo fmt

# リンターを実行
cargo clippy
```

## 依存関係

- `reqwest`: HTTP client
- `tokio`: 非同期ランタイム
- `serde`: シリアライゼーション
- `chrono`: 日時処理
- `anyhow`: エラーハンドリング

## 注意事項

このプロジェクトは非公式のSDKであり、radiko公式のサポートは受けられません。利用は自己責任で行ってください。

## ライセンス

MIT License

## 免責事項

このソフトウェアは「現状のまま」で提供され、明示的または暗示的を問わず、いかなる保証もありません。作者は、このソフトウェアの使用により生じたいかなる損害についても責任を負いません。
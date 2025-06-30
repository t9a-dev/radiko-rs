# radiko-rs

日本のインターネットラジオサービス「radiko」へのアクセスを提供する非公式Rustライブラリです。

## 特徴

- 2段階認証フローによるradiko APIの認証
- 認証トークンを用いたHLSストリーミングURLへのアクセス
- 番組情報と番組表の取得
- 番組検索機能
- タイムゾーン対応の日時処理

## インストール

`Cargo.toml`に以下を追加してください：

```toml
[dependencies]
radiko-rs = "0.1.0"
```

## 使用方法

```rust
use radiko_rs::{RadikoClient, RadikoAuthManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 認証済みクライアントを作成
    let auth_manager = RadikoAuthManager::new().await?;
    let client = RadikoClient::new(auth_manager).await?;
    
    // radiko APIにアクセス
    // ...
    
    Ok(())
}
```

## 認証について

このライブラリはradikoの2段階認証プロセスを自動的に処理します：

1. `auth1` APIからauthトークンとキー情報を取得
2. `playerCommon.js`から認証キーを抽出
3. `auth2` APIに部分キーを送信して認証を完了
4. `X-Radiko-Authtoken`ヘッダー付きのHTTPクライアントを作成

## ライセンス

このプロジェクトは以下のいずれかのライセンスで提供されます：

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) または http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) または http://opensource.org/licenses/MIT)

お好みの方をお選びください。

## 免責事項

このライブラリは教育目的および個人利用のみを目的としています。radikoの利用規約を尊重し、責任を持ってこのライブラリをご利用ください。
use std::sync::Arc;

use anyhow::Result;

use base64::{Engine, engine::general_purpose};
use regex::Regex;
use reqwest::{
    Client, Url,
    cookie::{CookieStore, Jar},
    header::HeaderMap,
};

use crate::api::endpoint::{LOGIN_CHECK_URL, RadikoEndpoint};

pub const USER_AGENT_VALUE: &str =
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:140.0) Gecko/20100101 Firefox/140.0";

#[derive(Debug, Clone)]
pub struct RadikoAuthManager {
    inner: Arc<RadikoAuthManagerRef>,
}

#[derive(Debug, Clone)]
struct RadikoAuthManagerRef {
    area_id: String,
    http_client: Client,
    auth_token: String,
    stream_lsid: String,
    cookie: String,
}

impl RadikoAuthManager {
    pub async fn new() -> Self {
        Self::init().await.unwrap()
    }

    pub fn get_area_id(&self) -> String {
        self.inner.area_id.clone()
    }

    pub fn get_http_client(&self) -> Client {
        self.inner.http_client.clone()
    }

    pub fn get_auth_token(&self) -> String {
        self.inner.auth_token.clone()
    }

    pub fn get_lsid(&self) -> String {
        self.inner.stream_lsid.clone()
    }

    pub fn get_cookie(&self) -> String {
        self.inner.cookie.clone()
    }

    pub fn get_headers_string_value(&self) -> String {
        let auth_token = self.get_auth_token();
        let area_id = self.get_area_id();
        let lsid = self.get_lsid();
        let radiko_session = self.get_cookie();
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "Accept: */*\r\n",
            "Accept-Encoding: gzip, deflate, br, zstd\r\n",
            "Accept-Language: ja,en-US;q=0.7,en;q=0.3\r\n",
            "DNT: 1\r\n",
            "Priority: u=0, i\r\n",
            "X-Radiko-Authtoken: ",
            auth_token,
            "\r\n",
            "X-Radiko-Areaid: ",
            area_id,
            "\r\n",
            // "User-Agent: ",
            // USER_AGENT,
            // "\r\n",
            "Origin: https://radiko.jp\r\n",
            "Referer: https://radiko.jp/\r\n",
            "Sec-Fetch-Dest: empty\r\n",
            "Sec-Fetch-Mode: cors\r\n",
            "Sec-Fetch-Site: cross-site\r\n",
            "Sec-Fetch-GPC: 1\r\n",
            "Upgrade-Insecure-Requests: 1\r\n",
            "Cookie: a_exp:",
            lsid,
            ";",
            "radiko-policy-accept:",
            "2024-10-04T00%3A00%3A00%2B09%3A00",
            ";",
            radiko_session,
            ";",
            "rdk_profile_data:{\"gender_code\":\"0\",\"birth_month\":\"202501\"};",
            "store_version:3;"
        )
    }

    pub async fn refresh_auth(&mut self) -> Result<Self> {
        Self::init().await
    }

    async fn init() -> Result<Self> {
        let auth1_url = RadikoEndpoint::get_auth1_endpoint();
        let auth2_url = RadikoEndpoint::get_auth2_endpoint();
        let auth_key = Self::get_public_auth_key().await;
        let cookie_jar = Arc::new(Jar::default());
        let client = Client::builder()
            .cookie_provider(cookie_jar.clone())
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .build()?;

        // get area_id
        let response_body = client
            .get(RadikoEndpoint::get_area_id_endpoint())
            .send()
            .await?
            .text()
            .await?;

        let area_id_pattern = Regex::new(r"[A-Z]{2}[0-9]{2}")?;
        let Some(area_id_caps) = area_id_pattern.captures(&response_body) else {
            panic!("not found pattern area_id");
        };
        let area_id = &area_id_caps[0];

        // set-cookie radiko_session
        let _ = client.get(LOGIN_CHECK_URL).send().await?;

        // auth1
        let mut headers = HeaderMap::new();
        headers.insert("X-Radiko-App", "pc_html5".parse()?);
        headers.insert("X-Radiko-App-Version", "0.0.1".parse()?);
        headers.insert("X-Radiko-User", "dummy_user".parse()?);
        headers.insert("X-Radiko-Device", "pc".parse()?);

        let res_auth1 = client.get(auth1_url).headers(headers).send().await?;

        // auth2
        let auth_token = res_auth1
            .headers()
            .get("X-Radiko-Authtoken")
            .unwrap()
            .to_str()?;
        let offset = res_auth1
            .headers()
            .get("X-Radiko-KeyOffset")
            .unwrap()
            .to_str()?
            .parse::<usize>()?;
        let length = res_auth1
            .headers()
            .get("X-Radiko-KeyLength")
            .unwrap()
            .to_str()?
            .parse::<usize>()?;
        let partial_key = general_purpose::STANDARD.encode(&auth_key[offset..offset + length]);

        let mut headers = HeaderMap::new();
        headers.insert("X-Radiko-Authtoken", auth_token.parse()?);
        headers.insert("X-Radiko-Partialkey", partial_key.parse()?);
        headers.insert("X-Radiko-User", "dummy_user".parse()?);
        headers.insert("X-Radiko-Device", "pc".parse()?);

        let _res_auth2 = client
            .get(&auth2_url)
            .headers(headers.clone())
            .send()
            .await?;

        let radiko_session_from_cookie = cookie_jar
            .cookies(&Url::parse(&auth2_url)?)
            .unwrap()
            .to_str()?
            .to_string();

        // cookieに設定されるa_expはmd5ハッシュ現在日時から適当に生成しているだけ
        // 適当なMD5ハッシュをlsidにしてブラウザと同じエンドポイントでストリーム開けるか試す
        // https://radiko.jp/apps/js/common.js?_=20250306
        let lsid = crate::utils::generate_md5_hash();

        let authed_client = Client::builder()
            .default_headers(headers.clone())
            .cookie_provider(cookie_jar.clone())
            .build()?;

        Ok(Self {
            inner: Arc::new(RadikoAuthManagerRef {
                area_id: area_id.to_string(),
                http_client: authed_client,
                auth_token: auth_token.to_string(),
                stream_lsid: lsid,
                cookie: radiko_session_from_cookie.to_string(),
            }),
        })
    }

    async fn get_public_auth_key() -> String {
        // https://github.com/miyagawa/ripdiko/blob/e9080f99c4c45b112256d822802f3dd56ab908f1/bin/ripdiko#L66
        let url = "https://radiko.jp/apps/js/playerCommon.js";
        let response_body = reqwest::get(url).await.unwrap().text().await.unwrap();
        let auth_key_pattern =
            regex::Regex::new(r"new RadikoJSPlayer\(.*?,.*?,.'(?P<auth_key>\w+)'").unwrap();
        let Some(auth_key_caps) = auth_key_pattern.captures(&response_body) else {
            panic!("failed get auth_key ")
        };

        auth_key_caps["auth_key"].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn init_radiko_auth_manager_test() -> Result<()> {
        let radiko_auth_manager = RadikoAuthManager::new().await;

        println!("radiko_auth_manager: {:#?}", radiko_auth_manager);
        println!(
            "get_headers_string_value: {:#?}",
            radiko_auth_manager.get_headers_string_value()
        );

        Ok(())
    }
}

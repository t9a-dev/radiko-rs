use std::{borrow::Cow, collections::HashMap, str::FromStr, sync::Arc};

use anyhow::{Result, anyhow};

use base64::{Engine, engine::general_purpose};
use regex::Regex;
use reqwest::{
    Client, Url,
    cookie::{self, Jar},
    header::HeaderMap,
};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

use crate::api::endpoint::RadikoEndpoint;

pub const USER_AGENT_VALUE: &str =
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:140.0) Gecko/20100101 Firefox/140.0";

#[derive(Debug, Clone)]
pub struct RadikoAuthManager {
    inner: Arc<RadikoAuthManagerRef>,
}

#[derive(Debug, Clone)]
struct RadikoAuthManagerRef {
    area_id: String,
    area_free: bool,
    http_client: Client,
    auth_token: String,
    stream_lsid: String,
    mail: Option<SecretString>,
    pass: Option<SecretString>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginResponse {
    twitter_name: Option<String>,
    status: String,
    unpaid: String,
    radiko_session: String,
    areafree: String,
    member_ukey: String,
    facebook_name: Option<String>,
    privileges: Vec<String>,
    paid_member: String,
}

impl RadikoAuthManager {
    pub async fn new() -> Self {
        Self::init(None, None).await.unwrap()
    }

    pub async fn new_area_free(mail: &str, pass: &str) -> Self {
        Self::init(
            Some(SecretString::new(mail.into())),
            Some(SecretString::new(pass.into())),
        )
        .await
        .unwrap()
    }

    pub fn area_id(&self) -> Cow<str> {
        Cow::Borrowed(&self.inner.area_id)
    }

    pub fn area_free(&self) -> bool {
        self.inner.area_free
    }

    pub fn http_client(&self) -> Client {
        self.inner.http_client.clone()
    }

    pub fn auth_token(&self) -> Cow<str> {
        Cow::Borrowed(&self.inner.auth_token)
    }

    pub fn lsid(&self) -> Cow<str> {
        Cow::Borrowed(&self.inner.stream_lsid)
    }

    pub async fn refresh_auth(&mut self) -> Result<Self> {
        Self::init(self.inner.mail.clone(), self.inner.pass.clone()).await
    }

    async fn init(mail: Option<SecretString>, pass: Option<SecretString>) -> Result<Self> {
        let is_area_free = mail.is_some() && pass.is_some();
        let auth1_url = RadikoEndpoint::auth1_endpoint();
        let auth2_url = RadikoEndpoint::auth2_endpoint();
        let auth_key = Self::get_public_auth_key().await;

        // get area_id
        let response_body = Client::new()
            .get(RadikoEndpoint::area_id_endpoint())
            .send()
            .await?
            .text()
            .await?;

        let area_id_pattern = Regex::new(r"[A-Z]{2}[0-9]{2}")?;
        let Some(area_id_caps) = area_id_pattern.captures(&response_body) else {
            panic!("failed get area_id. not found pattern area_id");
        };
        let default_area_id = area_id_caps[0].to_string();

        // login
        let cookie: Arc<cookie::Jar> = if is_area_free {
            RadikoAuthManager::login(
                &mail.clone().unwrap().expose_secret(),
                &pass.clone().unwrap().expose_secret(),
            )
            .await?
        } else {
            Arc::new(Jar::default())
        };
        let logined_client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .cookie_provider(cookie.clone())
            .build()?;

        // auth1
        let mut headers = HeaderMap::new();
        headers.insert("X-Radiko-App", "pc_html5".parse()?);
        headers.insert("X-Radiko-App-Version", "0.0.1".parse()?);
        headers.insert("X-Radiko-User", "dummy_user".parse()?);
        headers.insert("X-Radiko-Device", "pc".parse()?);

        let res_auth1 = logined_client
            .get(auth1_url)
            .headers(headers)
            .send()
            .await?;

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

        let res_auth2 = logined_client
            .get(&auth2_url)
            .headers(headers.clone())
            .send()
            .await?;
        if !res_auth2.status().is_success() {
            return Err(anyhow!("error auth2 request: {}", res_auth2.text().await?));
        }

        let authed_client = Client::builder()
            .cookie_provider(cookie)
            .default_headers(headers)
            .build()?;

        // cookieに設定されるa_expはmd5ハッシュ現在日時から適当に生成しているだけ
        // 適当なMD5ハッシュをlsidにしてブラウザと同じエンドポイントでストリーム開けるか試す
        // https://radiko.jp/apps/js/common.js?_=20250306
        let lsid = crate::utils::generate_md5_hash();

        Ok(Self {
            inner: Arc::new(RadikoAuthManagerRef {
                area_id: default_area_id.to_string(),
                area_free: is_area_free,
                http_client: authed_client,
                auth_token: auth_token.to_string(),
                stream_lsid: lsid,
                mail: mail,
                pass: pass,
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

    async fn login(mail: &str, pass: &str) -> Result<Arc<cookie::Jar>> {
        let mut login_info = HashMap::new();
        login_info.insert("mail", mail);
        login_info.insert("pass", pass);
        let login_res: LoginResponse = Client::new()
            .post(RadikoEndpoint::login_endpoint())
            .form(&login_info)
            .send()
            .await?
            .json()
            .await?;
        let cookie = format!("radiko_session={}", login_res.radiko_session);
        let jar = Arc::new(Jar::default());
        jar.add_cookie_str(&cookie, &Url::from_str(RadikoEndpoint::RADIKO_HOST)?);

        let login_check_res = Client::builder()
            .cookie_provider(jar.clone())
            .build()?
            .get(RadikoEndpoint::LOGIN_CHECK_URL)
            .send()
            .await?;

        if !login_check_res.status().is_success() {
            return Err(anyhow!(
                "login check failed: {}",
                login_check_res.text().await?
            ));
        }

        Ok(jar)
    }
}

#[cfg(test)]
mod tests {

    use crate::utils;

    use super::*;
    use std::env;

    #[tokio::test]
    async fn login_process_test() -> Result<()> {
        utils::load_env();
        let mail = env::var("mail").expect("failed mail from dotenv");
        let pass = env::var("pass").expect("failed pass from dotenv");
        let _ = RadikoAuthManager::login(&mail, &pass).await?;

        Ok(())
    }

    #[tokio::test]
    async fn init_radiko_auth_manager_test() -> Result<()> {
        let radiko_auth_manager = RadikoAuthManager::new().await;

        println!("radiko_auth_manager: {:#?}", radiko_auth_manager);

        Ok(())
    }
}

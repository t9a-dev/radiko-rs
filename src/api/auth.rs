use anyhow::Result;

use base64::{Engine, engine::general_purpose};
use reqwest::{Client, header::HeaderMap};

use crate::api::endpoint::RadikoEndpoint;

pub struct RadikoAuthManager {
    http_client: Client,
    auth_token: Option<String>,
}

impl RadikoAuthManager {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
            auth_token: None,
        }
    }

    pub async fn get_auth_token(&self) -> Result<String> {
        match &self.auth_token {
            Some(token) => Ok(token.clone()),
            None => self.generate_auth_token().await,
        }
    }

    pub async fn get_http_client_with_auth_token(&self) -> Result<Client> {
        let mut headers = HeaderMap::new();

        match &self.auth_token {
            Some(token) => {
                headers.insert("X-Radiko-Authtoken", token.parse()?);
            }
            None => {
                let token = self.generate_auth_token().await?;
                headers.insert("X-Radiko-Authtoken", token.parse()?);
            }
        }

        Ok(Client::builder().default_headers(headers).build()?)
    }

    pub async fn refresh_auth_token(&mut self) -> Result<()>{
        self.auth_token = Some(self.generate_auth_token().await?);
        Ok(())
    }

    async fn generate_auth_token(&self) -> Result<String> {
        let client = self.http_client.clone();
        let auth1_url = RadikoEndpoint::get_auth1_endpoint();
        let auth2_url = RadikoEndpoint::get_auth2_endpoint();
        let auth_key = Self::get_auth_key().await;

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
            .get(auth2_url)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        Ok(auth_token.to_string())
    }

    async fn get_auth_key() -> String {
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
mod tests{
    use super::*;

    #[tokio::test]
    async fn get_auth_token_test() -> Result<()>{
        let radiko_auth_manager = RadikoAuthManager::new();
        let token = radiko_auth_manager.get_auth_token().await?;
        println!("{}",&token);
        assert_ne!("",&token);
        Ok(())
    }
}
use anyhow::Result;

use crate::client::RadikoClient;

use super::endpoint::RadikoEndpoint;

pub struct RadikoStream {
    radiko_client: RadikoClient,
    stream_url: String,
}

impl RadikoStream {
    pub fn new(station_id: &str, radiko_client: RadikoClient) -> Self {
        Self {
            radiko_client,
            stream_url: RadikoEndpoint::get_playlist_create_url_endpoint(station_id),
        }
    }

    pub fn get_stream_url(&self) -> &str {
        &self.stream_url
    }

    pub async fn validate_stream_url(&self) -> Result<bool> {
        let res = self
            .radiko_client
            .http_client
            .get(self.stream_url.clone())
            .send()
            .await?;
        Ok(res.status().is_success())
    }
}

#[cfg(test)]
mod tests {
    use crate::api::auth::RadikoAuthManager;
    use crate::api::stream::RadikoStream;
    use crate::client::RadikoClient;
    use anyhow::Result;

    #[tokio::test]
    async fn valid_stream_url_test() -> Result<()> {
        let station_id = "TBS";
        let radiko_auth_manager = RadikoAuthManager::new().await;
        let radiko_client = RadikoClient::new(radiko_auth_manager).await;
        let radiko_stream = RadikoStream::new(station_id, radiko_client);

        assert!(radiko_stream.validate_stream_url().await?);
        Ok(())
    }
}

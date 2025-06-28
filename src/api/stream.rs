use anyhow::Result;
use reqwest;

use crate::client::RadikoClient;

pub struct RadikoStream {
    client: RadikoClient,
    stream_url: String,
}

impl RadikoStream {
    pub fn new(radiko_client: RadikoClient, stream_url: String) -> Self {
        Self {
            client: radiko_client,
            stream_url: stream_url,
        }
    }

    pub async fn validate_stream_url(&self) -> Result<bool> {
        let res = self
            .client
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
    use crate::api::endpoint::RadikoEndpoint;
    use crate::api::stream::RadikoStream;
    use crate::client::RadikoClient;
    use anyhow::Result;

    #[tokio::test]
    async fn valid_stream_url_test() -> Result<()> {
        let station_id = "TBS";
        let stream_url = RadikoEndpoint::get_playlist_create_url_endpoint(station_id);
        let radiko_auth_manager = RadikoAuthManager::new();
        let radiko_client = RadikoClient::new(radiko_auth_manager).await;
        let radiko_stream = RadikoStream::new(radiko_client, stream_url.to_string());

        assert_eq!(true, radiko_stream.validate_stream_url().await?);
        Ok(())
    }
}

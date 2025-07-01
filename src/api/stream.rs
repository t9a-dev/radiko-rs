use std::{rc::Rc, sync::Arc};

use anyhow::Result;
use reqwest::Client;

use crate::client::RadikoClient;

use super::endpoint::RadikoEndpoint;

pub struct RadikoStream {
    inner: Arc<RadikoStreamRef>,
}

struct RadikoStreamRef {
    station_id: String,
    radiko_client: RadikoClient,
    stream_url: String,
}

impl RadikoStream {
    pub fn new(station_id: &str, radiko_client: RadikoClient) -> Self {
        Self {
            inner: Arc::new(RadikoStreamRef {
                station_id: station_id.to_string(),
                radiko_client: radiko_client.clone(),
                stream_url: RadikoEndpoint::get_playlist_create_url_endpoint(
                    station_id,
                    &radiko_client.get_auth_manager().get_lsid(),
                ),
            }),
        }
    }

    pub fn get_station_id(&self) -> String {
        self.inner.station_id.clone()
    }

    pub fn get_http_client(&self) -> Client {
        self.inner.radiko_client.get_http_client()
    }

    pub async fn get_hls_master_playlist(&self) -> Result<String> {
        Ok(self
            .get_http_client()
            .get(&self.inner.stream_url)
            .send()
            .await?
            .text()
            .await?)
    }

    pub fn get_stream_url(&self) -> &str {
        &self.inner.stream_url
    }
}

#[cfg(test)]
mod tests {
    use crate::api::auth::RadikoAuthManager;
    use crate::api::stream::RadikoStream;
    use crate::client::RadikoClient;
    use anyhow::Result;
    use reqwest::Client;

    #[tokio::test]
    async fn valid_stream_url_test() -> Result<()> {
        let station_id = "TBS";
        let radiko_auth_manager = RadikoAuthManager::new().await;
        let radiko_client = RadikoClient::new(radiko_auth_manager.clone()).await;
        let radiko_stream = RadikoStream::new(station_id, radiko_client.clone());

        println!("radiko_auth_manager: {:#?}", radiko_auth_manager);
        println!("area_id: {}", radiko_client.get_area_id());
        println!("station_id: {}", station_id);

        Ok(())
    }
}

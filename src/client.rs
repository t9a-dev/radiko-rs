use std::sync::Arc;

use reqwest::Client;

use crate::api::auth::RadikoAuthManager;

#[derive(Debug, Clone)]
pub struct RadikoClient {
    inner: Arc<RadikoClientRef>,
}

#[derive(Debug, Clone)]
struct RadikoClientRef {
    auth_manager: RadikoAuthManager,
    http_client: Client,
    area_id: String,
}

impl RadikoClient {
    pub async fn new(radiko_auth_manager: RadikoAuthManager) -> Self {
        Self {
            inner: Arc::new(RadikoClientRef {
                auth_manager: radiko_auth_manager.clone(),
                http_client: radiko_auth_manager.get_http_client(),
                area_id: radiko_auth_manager.get_area_id().await.unwrap(),
            }),
        }
    }

    pub fn get_auth_manager(&self) -> RadikoAuthManager {
        self.inner.auth_manager.clone()
    }

    pub fn get_area_id(&self) -> String {
        self.inner.area_id.clone()
    }

    pub fn get_http_client(&self) -> Client {
        self.inner.http_client.clone()
    }
}

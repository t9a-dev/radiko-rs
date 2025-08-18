use std::sync::Arc;

use reqwest::Client;

use crate::api::auth::RadikoAuthManager;

#[derive(Debug, Clone)]
pub struct RadikoClient {
    inner: Arc<RadikoClientRef>,
}

#[derive(Debug, Clone)]
struct RadikoClientRef {
    auth_manager: Arc<RadikoAuthManager>,
    http_client: Client,
    area_id: String,
}

impl RadikoClient {
    pub async fn new(radiko_auth_manager: Arc<RadikoAuthManager>) -> Self {
        Self {
            inner: Arc::new(RadikoClientRef {
                auth_manager: Arc::clone(&radiko_auth_manager),
                http_client: radiko_auth_manager.http_client(),
                area_id: radiko_auth_manager.area_id().to_string(),
            }),
        }
    }

    pub fn auth_manager(&self) -> Arc<RadikoAuthManager> {
        Arc::clone(&self.inner.auth_manager)
    }

    pub fn area_id(&self) -> String {
        self.inner.area_id.clone()
    }

    pub fn http_client(&self) -> Client {
        self.inner.http_client.clone()
    }
}

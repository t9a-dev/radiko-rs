use reqwest::Client;
use std::time::Duration;

use crate::api::auth::RadikoAuthManager;

pub struct RadikoClient {
    pub http_client: Client,
    pub area_id: String,
}

impl RadikoClient {
    pub async fn new(radiko_auth_manager: RadikoAuthManager) -> Self {
        Self {
            http_client: radiko_auth_manager.get_http_client_with_auth_token().await.unwrap(),
            area_id: "".to_string(),
        }
    }
}

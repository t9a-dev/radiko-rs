use reqwest::Client;

use crate::api::auth::RadikoAuthManager;

#[derive(Debug,Clone)]
pub struct RadikoClient {
    pub auth_manager: RadikoAuthManager,
    http_client: Client,
    area_id: String,
}

impl RadikoClient {
    pub async fn new(mut radiko_auth_manager: RadikoAuthManager) -> Self {
        Self {
            auth_manager: radiko_auth_manager.clone(),
            http_client: radiko_auth_manager
                .get_http_client_with_auth_token()
                .await
                .unwrap(),
            area_id: radiko_auth_manager.get_area_id().await.unwrap(),
        }
    }

    pub fn get_area_id(&self) -> String{
        self.area_id.clone()
    }

    pub fn get_http_client(&self) -> Client {
        self.http_client.clone()
    }
}

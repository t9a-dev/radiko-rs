use std::sync::Arc;

use crate::api::{
    auth::RadikoAuthManager, client::RadikoClient, program::RadikoProgram, station::RadikoStation,
    stream::RadikoStream,
};

pub struct Radiko {
    auth_manager: Arc<RadikoAuthManager>,
    client: Arc<RadikoClient>,
    station: RadikoStation,
    stream: RadikoStream,
    program: RadikoProgram,
}

impl Radiko {
    pub async fn new() -> Self {
        let shared_auth_manager = Arc::new(RadikoAuthManager::new().await);
        let shared_client = Arc::new(RadikoClient::new(Arc::clone(&shared_auth_manager)).await);

        Self {
            auth_manager: Arc::clone(&shared_auth_manager),
            client: Arc::clone(&shared_client),
            station: RadikoStation::new(Arc::clone(&shared_client)),
            stream: RadikoStream::new(Arc::clone(&shared_client)),
            program: RadikoProgram::new(Arc::clone(&shared_client)),
        }
    }

    pub async fn new_area_free(email: &str, password: &str) -> Self {
        let shared_auth_manager = Arc::new(RadikoAuthManager::new_area_free(email, password).await);
        let shared_client = Arc::new(RadikoClient::new(Arc::clone(&shared_auth_manager)).await);

        Self {
            auth_manager: Arc::clone(&shared_auth_manager),
            client: Arc::clone(&shared_client),
            station: RadikoStation::new(Arc::clone(&shared_client)),
            stream: RadikoStream::new(Arc::clone(&shared_client)),
            program: RadikoProgram::new(Arc::clone(&shared_client)),
        }
    }

    pub fn auth_manager(&self) -> Arc<RadikoAuthManager> {
        Arc::clone(&self.auth_manager)
    }
    pub fn client(&self) -> Arc<RadikoClient> {
        Arc::clone(&self.client)
    }
    pub fn station(&self) -> &RadikoStation {
        &self.station
    }
    pub fn stream(&self) -> &RadikoStream {
        &self.stream
    }
    pub fn program(&self) -> &RadikoProgram {
        &self.program
    }
}

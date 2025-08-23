use std::sync::{Arc, RwLock};

use crate::{
    api::{
        auth::RadikoAuthManager, program::RadikoProgram, station::RadikoStation,
        stream::RadikoStream,
    },
    models::{
        program::Programs, region::RegionStations, search::SearchCondition, station::Stations,
    },
};
use anyhow::Result;
use secrecy::{ExposeSecret, SecretString};

pub struct Radiko {
    inner: Arc<RwLock<RadikoRef>>,
}

struct RadikoRef {
    auth_manager: Arc<RadikoAuthManager>,
    stream: RadikoStream,
    station: RadikoStation,
    program: RadikoProgram,
    email: Option<SecretString>,
    password: Option<SecretString>,
}

impl Radiko {
    pub async fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(Self::init_inner(None, None).await)),
        }
    }

    pub async fn new_area_free(email: &str, password: &str) -> Self {
        Self {
            inner: Arc::new(RwLock::new(
                Self::init_inner(
                    Some(SecretString::new(email.into())),
                    Some(SecretString::new(password.into())),
                )
                .await,
            )),
        }
    }

    async fn init_inner(email: Option<SecretString>, password: Option<SecretString>) -> RadikoRef {
        let is_area_free = email.is_some() && password.is_some();
        let shared_auth_manager = if is_area_free {
            Arc::new(
                RadikoAuthManager::new_area_free(
                    email.clone().unwrap().expose_secret(),
                    password.clone().unwrap().expose_secret(),
                )
                .await,
            )
        } else {
            Arc::new(RadikoAuthManager::new().await)
        };
        RadikoRef {
            auth_manager: Arc::clone(&shared_auth_manager),
            stream: RadikoStream::new(Arc::clone(&shared_auth_manager)),
            station: RadikoStation::new(),
            program: RadikoProgram::new(),
            email: email,
            password: password,
        }
    }

    pub async fn refresh_auth(&self) -> Result<()> {
        let email = self.inner.read().unwrap().email.clone();
        let password = self.inner.read().unwrap().password.clone();
        let mut inner = self.inner.write().unwrap();
        *inner = Self::init_inner(email, password).await;
        drop(inner);
        Ok(())
    }

    pub fn area_id(&self) -> String {
        self.inner
            .read()
            .unwrap()
            .auth_manager
            .area_id()
            .to_string()
    }
    pub fn auth_token(&self) -> String {
        self.inner
            .read()
            .unwrap()
            .auth_manager
            .auth_token()
            .to_string()
    }

    pub fn stream_url(&self, station_id: &str) -> String {
        self.inner
            .read()
            .unwrap()
            .stream
            .stream_url(station_id)
            .to_string()
    }

    pub async fn stations_all(&self) -> Result<Vec<RegionStations>> {
        self.inner.read().unwrap().station.stations_all().await
    }

    pub async fn stations_from_area_id(&self, area_id: &str) -> Result<Stations> {
        self.inner
            .read()
            .unwrap()
            .station
            .stations_from_area_id(area_id)
            .await
    }

    pub async fn now_on_air_programs(&self, area_id: &str) -> Result<Programs> {
        self.inner
            .read()
            .unwrap()
            .program
            .now_on_air_programs(area_id)
            .await
    }

    pub async fn weekly_programs_from_station(&self, station_id: &str) -> Result<Programs> {
        self.inner
            .read()
            .unwrap()
            .program
            .weekly_programs_from_station(station_id)
            .await
    }

    pub async fn find_program(&self, search_condition: &SearchCondition) -> Result<Programs> {
        self.inner
            .read()
            .unwrap()
            .program
            .find_program(search_condition)
            .await
    }
}

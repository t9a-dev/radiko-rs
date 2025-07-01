use std::sync::Arc;

use crate::{client::RadikoClient, dto::station_xml::RadikoStationXml, models::station::Stations};
use anyhow::Result;

use super::endpoint::RadikoEndpoint;

#[derive(Debug, Clone)]
pub struct RadikoStation {
    inner: Arc<RadikoStationRef>,
}

#[derive(Debug, Clone)]
struct RadikoStationRef {
    client: RadikoClient,
}

impl RadikoStation {
    pub fn new(radiko_client: RadikoClient) -> Self {
        Self {
            inner: Arc::new(RadikoStationRef {
                client: radiko_client,
            }),
        }
    }

    pub async fn get_stations(&self, area_id: &str) -> Result<Stations> {
        let res = self
            .inner
            .client
            .get_http_client()
            .get(RadikoEndpoint::get_station_list_endpoint(area_id))
            .send()
            .await?
            .text()
            .await?;

        let radiko_station: RadikoStationXml = quick_xml::de::from_str(&res)?;

        Ok(Stations::from(radiko_station))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::{auth::RadikoAuthManager, station::RadikoStation},
        client::RadikoClient,
    };
    use anyhow::{Ok, Result};

    #[tokio::test]
    async fn get_stations_test() -> Result<()> {
        let area_id = "JP13";
        let radiko_auth_manager = RadikoAuthManager::new().await;
        let radiko_client = RadikoClient::new(radiko_auth_manager).await;
        let radiko_station = RadikoStation::new(radiko_client);
        let stations = radiko_station.get_stations(area_id).await?;

        println!("{}_stations: {:#?}", area_id, stations);

        assert!(stations.data.len() > 0);
        assert_eq!(stations.area_id, area_id);
        Ok(())
    }
}

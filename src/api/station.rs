use std::sync::Arc;

use crate::{
    client::RadikoClient,
    dto::{region_xml::RegionXml, station_xml::RadikoStationXml},
    models::{
        region::{Region, RegionStations},
        station::Stations,
    },
};
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

    pub async fn get_stations_from_area_id(&self, area_id: &str) -> Result<Stations> {
        let res = self
            .inner
            .client
            .http_client()
            .get(RadikoEndpoint::station_list_from_area_id_endpoint(area_id))
            .send()
            .await?
            .text()
            .await?;

        let radiko_station: RadikoStationXml = quick_xml::de::from_str(&res)?;

        Ok(Stations::from(radiko_station))
    }

    pub async fn get_station_list_all(&self) -> Result<Vec<RegionStations>> {
        let res = self
            .inner
            .client
            .http_client()
            .get(RadikoEndpoint::station_list_all_endpoint())
            .send()
            .await?
            .text()
            .await?;

        let region: RegionXml = quick_xml::de::from_str(&res)?;

        Ok(Region::from(region).stations_groups)
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
        let stations = radiko_station.get_stations_from_area_id(area_id).await?;

        println!("{}_stations: {:#?}", area_id, stations);

        assert!(!stations.data.is_empty());
        assert_eq!(stations.area_id, area_id);
        Ok(())
    }

    #[tokio::test]
    async fn get_station_list_all_test() -> Result<()> {
        let radiko_auth_manager = RadikoAuthManager::new().await;
        let radiko_client = RadikoClient::new(radiko_auth_manager).await;
        let radiko_station = RadikoStation::new(radiko_client);
        let all_station_list = radiko_station.get_station_list_all().await?;

        for region in all_station_list.iter() {
            println!("{}", region.region_name);
            for station in region.stations.iter() {
                println!("{}:{}:{}", station.area_id, station.id, station.name);
            }
            println!("{}", "-".repeat(40));
        }

        assert!(!all_station_list.is_empty());
        Ok(())
    }
}

use crate::{dto::station_xml::RadikoStationXml, models::station::Stations};
use anyhow::Result;
use reqwest::Client;

use super::endpoint::RadikoEndpoint;

pub struct RadikoStation {
    http_client: Client,
}

impl RadikoStation {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
        }
    }

    pub async fn get_stations(&self, area_id: &str) -> Result<Stations> {
        let res = self
            .http_client
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
    use crate::api::station::RadikoStation;
    use anyhow::{Ok, Result};

    #[tokio::test]
    async fn get_stations_test() -> Result<()> {
        let area_id = "JP13";
        let radiko_station = RadikoStation::new();
        let stations = radiko_station.get_stations(area_id).await?;

        println!("{}_stations: {:#?}", area_id, stations);

        assert!(stations.data.len() > 0);
        assert_eq!(stations.area_id, area_id);
        Ok(())
    }
}
use std::sync::Arc;

use crate::client::RadikoClient;
use crate::models::search::SearchCondition;
use crate::{dto::program_xml::RadikoProgramXml, models::program::Programs};
use anyhow::{Result, anyhow};

use super::endpoint::RadikoEndpoint;

pub struct RadikoProgram {
    inner: Arc<RadikoProgramRef>,
}

struct RadikoProgramRef {
    radiko_client: RadikoClient,
}

impl RadikoProgram {
    pub fn new(radiko_client: RadikoClient) -> Self {
        Self {
            inner: Arc::new(RadikoProgramRef {
                radiko_client: radiko_client,
            }),
        }
    }

    pub async fn get_now_on_air_programs(&self, area_id: &str) -> Result<Programs> {
        let res = self
            .inner
            .radiko_client
            .get_http_client()
            .get(RadikoEndpoint::get_now_on_air_programs(area_id))
            .send()
            .await?
            .text()
            .await?;

        let radiko_program: RadikoProgramXml = quick_xml::de::from_str(&res)?;

        Ok(Programs::from(radiko_program))
    }

    pub async fn find_program_from_condition(
        &self,
        condition: &SearchCondition,
    ) -> Result<Programs> {
        if condition.key.is_empty() {
            return Err(anyhow!("condition key required."));
        }

        let res = self
            .inner
            .radiko_client
            .get_http_client()
            .get(RadikoEndpoint::get_search_endpoint())
            .query(&condition.to_query_params())
            .send()
            .await?
            .text()
            .await?;

        Ok(serde_json::from_str(&res)?)
    }

    pub async fn find_weekly_programs_from_station(&self, station_id: &str) -> Result<Programs> {
        let res = self
            .inner
            .radiko_client
            .get_http_client()
            .get(RadikoEndpoint::get_weekly_programs_endpoint(station_id))
            .send()
            .await?
            .text()
            .await?;

        let radiko_program: RadikoProgramXml = quick_xml::de::from_str(&res)?;

        Ok(Programs::from(radiko_program))
    }
}

#[cfg(test)]
mod tests {
    use crate::api::{auth::RadikoAuthManager, program::RadikoProgram};
    use crate::client::RadikoClient;
    use crate::models::search::SearchCondition;
    use anyhow::{Ok, Result};

    #[tokio::test]
    async fn get_now_on_air_programs_test() -> Result<()> {
        let area_id = "JP13";
        let radiko_auth_manager = RadikoAuthManager::new().await;
        let radiko_client = RadikoClient::new(radiko_auth_manager).await;
        let radiko_program = RadikoProgram::new(radiko_client);
        let programs = radiko_program.get_now_on_air_programs(area_id).await?;

        println!("{}_now_on_air_programs: {:#?}", area_id, programs);

        assert!(programs.data.len() > 0);

        Ok(())
    }

    #[tokio::test]
    async fn find_program_from_condition_test() -> Result<()> {
        let search_condition = SearchCondition {
            key: vec!["オールナイトニッポン".to_string(), "".to_string()],
            station_id: Some(vec!["LFR".to_string()]),
            ..Default::default()
        };
        let radiko_auth_manager = RadikoAuthManager::new().await;
        let radiko_client = RadikoClient::new(radiko_auth_manager).await;
        let radiko_program = RadikoProgram::new(radiko_client);
        let result = radiko_program
            .find_program_from_condition(&search_condition)
            .await?;

        println!("{:#?}", result);

        assert!(result.data.len() > 0);
        Ok(())
    }

    #[tokio::test]
    async fn find_weekly_programs_from_station_test() -> Result<()> {
        let station_id = "LFR";
        let radiko_auth_manager = RadikoAuthManager::new().await;
        let radiko_client = RadikoClient::new(radiko_auth_manager).await;
        let radiko_program = RadikoProgram::new(radiko_client);
        let programs = radiko_program
            .find_weekly_programs_from_station(station_id)
            .await?;

        println!("{}_weekly_programs: {:#?}", station_id, programs);

        assert!(programs.data.len() > 0);

        Ok(())
    }
}

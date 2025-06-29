use crate::models::search::SearchCondition;
use crate::{dto::program_xml::RadikoProgramXml, models::program::Programs};
use anyhow::{Result, anyhow};
use reqwest::Client;

use super::endpoint::RadikoEndpoint;

pub struct RadikoProgram {
    http_client: Client,
}

impl RadikoProgram {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
        }
    }

    pub async fn find_program_from_condition(
        &self,
        condition: &SearchCondition,
    ) -> Result<Programs> {
        if condition.key.is_empty() {
            return Err(anyhow!("condition key required."));
        }

        let res = self
            .http_client
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
            .http_client
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
    use crate::api::program::RadikoProgram;
    use crate::models::search::SearchCondition;
    use anyhow::{Ok, Result};

    #[tokio::test]
    async fn find_program_from_condition_test() -> Result<()> {
        let search_condition = SearchCondition {
            key: vec!["オールナイトニッポン".to_string(), "".to_string()],
            station_id: Some(vec!["LFR".to_string()]),
            ..Default::default()
        };
        let radiko_program = RadikoProgram::new();
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
        let radiko_program = RadikoProgram::new();
        let programs = radiko_program
            .find_weekly_programs_from_station(station_id)
            .await?;

        println!("{}_weekly_programs: {:#?}", station_id, programs);

        assert!(programs.data.len() > 0);

        Ok(())
    }
}

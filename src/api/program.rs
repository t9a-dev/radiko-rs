use std::sync::Arc;

use crate::models::search::SearchCondition;
use crate::{dto::program_xml::RadikoProgramXml, models::program::Programs};
use anyhow::{Result, anyhow};
use reqwest::Client;

use super::endpoint::RadikoEndpoint;

pub struct RadikoProgram {
    inner: Arc<RadikoProgramRef>,
}

struct RadikoProgramRef {
    client: Client,
}

impl RadikoProgram {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RadikoProgramRef {
                client: Client::new(),
            }),
        }
    }

    pub async fn now_on_air_programs(&self, area_id: &str) -> Result<Programs> {
        let res = self
            .inner
            .client
            .get(RadikoEndpoint::now_on_air_programs(area_id))
            .send()
            .await?
            .text()
            .await?;

        let radiko_program: RadikoProgramXml = quick_xml::de::from_str(&res)?;

        Ok(Programs::from(radiko_program))
    }

    pub async fn find_program(&self, condition: &SearchCondition) -> Result<Programs> {
        if condition.key.is_empty() {
            return Err(anyhow!("condition key required."));
        }

        let res = self
            .inner
            .client
            .get(RadikoEndpoint::search_endpoint())
            .query(&condition.to_query_params())
            .send()
            .await?
            .text()
            .await?;

        Ok(serde_json::from_str(&res)?)
    }

    pub async fn weekly_programs_from_station(&self, station_id: &str) -> Result<Programs> {
        let res = self
            .inner
            .client
            .get(RadikoEndpoint::weekly_programs_endpoint(station_id))
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
    use super::*;
    use crate::radiko::Radiko;

    #[tokio::test]
    async fn get_now_on_air_programs_test() -> Result<()> {
        let area_id = "JP13";
        let radiko = Radiko::new().await;
        let programs = radiko.now_on_air_programs(area_id).await?;

        println!("{}_now_on_air_programs: {:#?}", area_id, programs);

        assert!(!programs.data.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn find_program_from_condition_test() -> Result<()> {
        let search_condition = SearchCondition {
            key: vec!["オールナイトニッポン".to_string(), "".to_string()],
            station_id: Some(vec!["LFR".to_string()]),
            ..Default::default()
        };
        let radiko = Radiko::new().await;
        let result = radiko.find_program(&search_condition).await?;

        println!("{:#?}", result);

        assert!(!result.data.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn find_weekly_programs_from_station_test() -> Result<()> {
        let station_id = "LFR";
        let radiko = Radiko::new().await;
        let programs = radiko.weekly_programs_from_station(station_id).await?;

        println!("{}_weekly_programs: {:#?}", station_id, programs);

        assert!(!programs.data.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn program_duration_methods_test() -> Result<()> {
        let station_id = "LFR";
        let radiko = Radiko::new().await;
        let programs = radiko.weekly_programs_from_station(station_id).await?;
        let program_len = programs.data.len();
        let target_program = programs.data[program_len - 150].clone();

        println!("program: {:#?}", target_program);

        println!(
            "now2start_time num_secs: {:#?}",
            target_program.now_to_start_duration(None)
        );

        println!(
            "start2end secs: {:#?}",
            target_program.start_to_end_duration()
        );

        assert!(!programs.data.is_empty());

        Ok(())
    }
}

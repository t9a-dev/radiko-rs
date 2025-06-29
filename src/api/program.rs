use crate::models::program::{SearchCondition, SearchResults};
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
    ) -> Result<SearchResults> {
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

    pub async fn find_program_from_station(&self, _station_id: &str) -> Result<SearchCondition> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::program::RadikoProgram;
    use crate::models::program::SearchCondition;
    use anyhow::Result;

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

        result
            .data
            .iter()
            .for_each(|program| println!("{}", program.title));

        assert!(result.data.len() > 0);
        Ok(())
    }
}

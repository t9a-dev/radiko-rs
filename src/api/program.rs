use crate::models::program::{SearchCondition, SearchResult, SearchResults};
use anyhow::{anyhow, Result};
use reqwest::Client;

use super::endpoint::RadikoEndpoint;

pub struct RadikoProgram{
    http_client: Client
}

impl RadikoProgram{
    pub fn new() -> Self{
        Self { http_client: Client::new() }
    }
    pub async fn find_program(&self,condition: &SearchCondition) -> Result<SearchResults>{
        if condition.key.is_empty(){
            return Err(anyhow!("condition key required."));
        }
        
        let res = self.http_client
        .get(RadikoEndpoint::get_search_endpoint())
        .query(&condition.to_query_params())
        .send()
        .await?
        .text()
        .await?;

        Ok(serde_json::from_str(&res)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::program::RadikoProgram;
    use crate::models::program::SearchCondition;
    use anyhow::Result;

    #[tokio::test]
    async fn find_program_test() -> Result<()> {
        let search_condition = SearchCondition{
            key: vec!["オールナイトニッポン".to_string()],
            ..Default::default()
        };
        let radiko_program = RadikoProgram::new(); 
        let result = radiko_program.find_program(&search_condition).await?;

        println!("{:#?}",&result.data);

        assert!(result.data.len() > 0);
        Ok(())
    }
}

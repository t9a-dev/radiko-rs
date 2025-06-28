use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Urls {
    #[serde(rename = "url")]
    url: Vec<Url>,
}

#[derive(Debug, Deserialize)]
pub struct Url {
    #[serde(rename = "@areafree")]
    areafree: bool,
    playlist_create_url: String,
}

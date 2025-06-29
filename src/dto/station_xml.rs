use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename = "stations")]
pub struct RadikoStationXml {
    #[serde(rename = "@area_id")]
    pub area_id: String,
    #[serde(rename = "@area_name")]
    pub area_name: String,
    #[serde(rename = "station")]
    pub stations: Vec<StationXml>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StationXml {
    pub id: String,
    pub name: String,
    pub ascii_name: String,
    pub ruby: String,
    pub areafree: u8,
    pub timefree: u8,
    #[serde(rename = "logo")]
    pub logos: Vec<LogoXml>,
    pub banner: String,
    pub href: String,
    pub simul_max_delay: u32,
    pub tf_max_delay: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogoXml {
    #[serde(rename = "@width")]
    pub width: u32,
    #[serde(rename = "@height")]
    pub height: u32,
    #[serde(rename = "@align")]
    pub align: String,
    #[serde(rename = "$text")]
    pub url: String,
}
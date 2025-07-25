use serde_derive::{Deserialize, Serialize};

use crate::dto::station_xml::{LogoXml, RadikoStationXml, StationXml as StationXmlDto};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    pub id: String,
    pub name: String,
    pub ascii_name: String,
    pub ruby: String,
    pub areafree: bool,
    pub timefree: bool,
    pub logos: Vec<Logo>,
    pub banner: String,
    pub href: String,
    pub simul_max_delay: u32,
    pub tf_max_delay: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logo {
    pub width: u32,
    pub height: u32,
    pub align: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stations {
    pub area_id: String,
    pub area_name: String,
    pub data: Vec<Station>,
}

impl From<LogoXml> for Logo {
    fn from(value: LogoXml) -> Self {
        Logo {
            width: value.width,
            height: value.height,
            align: value.align,
            url: value.url,
        }
    }
}

impl From<StationXmlDto> for Station {
    fn from(value: StationXmlDto) -> Self {
        Station {
            id: value.id,
            name: value.name,
            ascii_name: value.ascii_name,
            ruby: value.ruby,
            areafree: value.areafree == 1,
            timefree: value.timefree == 1,
            logos: value.logos.into_iter().map(Logo::from).collect(),
            banner: value.banner,
            href: value.href,
            simul_max_delay: value.simul_max_delay,
            tf_max_delay: value.tf_max_delay,
        }
    }
}

impl From<RadikoStationXml> for Stations {
    fn from(value: RadikoStationXml) -> Self {
        Stations {
            area_id: value.area_id,
            area_name: value.area_name,
            data: value.stations.into_iter().map(Station::from).collect(),
        }
    }
}

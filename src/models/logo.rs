use serde_derive::{Deserialize, Serialize};

use crate::dto::logo_xml::LogoXml;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logo {
    pub width: u32,
    pub height: u32,
    pub align: String,
    pub url: String,
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

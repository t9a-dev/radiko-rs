use serde_derive::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::{AsRefStr, Display};

#[derive(Debug, Clone, Copy, Display, AsRefStr, Serialize, Deserialize)]
pub enum Filter {
    #[strum(to_string = "future")]
    Live,
    #[strum(to_string = "")]
    All,
    #[strum(to_string = "past")]
    TimeFree,
}

// ex: https://radiko.jp/v3/api/program/search?key=トム・ブラウン
// ```json
// "meta": {
//   "key": [
//     "トム・ブラウン"
//   ],
//   "station_id": [],
//   "area_id": [],
//   "cur_area_id": "",
//   "region_id": "",
//   "start_day": "",
//   "end_day": "",
//   "filter": "",
//   "result_count": 8,
//   "page_idx": 0,
//   "row_limit": 0,
//   "kakuchou": [],
//   "suisengo": "",
//   "genre_id": []
// },
// ```
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCondition {
    pub key: Vec<String>,
    pub filter: Option<Filter>,
    pub start_day: Option<String>,
    pub end_day: Option<String>,
    pub row_limit: Option<i32>,
    pub area_id: Option<Vec<String>>,
    pub station_id: Option<Vec<String>>,
    pub cur_area_id: Option<String>,
}

impl Default for SearchCondition {
    fn default() -> Self {
        Self {
            filter: Some(Filter::Live),
            row_limit: Some(50),
            key: Default::default(),
            start_day: Default::default(),
            end_day: Default::default(),
            area_id: Default::default(),
            station_id: Default::default(),
            cur_area_id: Default::default(),
        }
    }
}

impl SearchCondition {
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        for key in &self.key {
            params.push(("key".to_string(), key.clone()));
        }

        if let Some(station_ids) = &self.station_id {
            for station_id in station_ids {
                params.push(("station_id".to_string(), station_id.clone()));
            }
        }

        if let Some(area_ids) = &self.area_id {
            for area_id in area_ids {
                params.push(("area_id".to_string(), area_id.clone()));
            }
        }

        if let Some(cur_area_id) = &self.cur_area_id {
            params.push(("cur_area_id".to_string(), cur_area_id.clone()));
        }

        if let Some(start_day) = &self.start_day {
            params.push(("start_day".to_string(), start_day.clone()));
        }

        if let Some(end_day) = &self.end_day {
            params.push(("end_day".to_string(), end_day.clone()));
        }

        if let Some(filter) = &self.filter {
            params.push(("filter".to_string(), filter.to_string().clone()));
        }

        if let Some(row_limit) = &self.row_limit {
            params.push(("row_limit".to_string(), row_limit.to_string()));
        }

        params
    }
}

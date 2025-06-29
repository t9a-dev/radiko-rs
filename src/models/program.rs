
use serde_derive::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum_macros::{AsRefStr, Display};

#[derive(Debug,Clone, Copy,Display,AsRefStr,Serialize,Deserialize)]
pub enum Filter {
    #[strum(to_string= "future")]
    Live,
    #[strum(to_string= "")]
    All,
    #[strum(to_string= "past")]
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

impl Default for SearchCondition{
    fn default() -> Self {
        Self {
             filter: Some(Filter::Live),
             row_limit: Some(50),
             key: Default::default(),
             start_day: Default::default(),
             end_day: Default::default(),
             area_id: Default::default(),
             station_id: Default::default(),
             cur_area_id: Default::default() 
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

// ```json
// "data": [
//   {
//     "start_time": "2025-06-29 00:00:00",
//     "end_time": "2025-06-29 01:30:00",
//     "start_time_s": "2400",
//     "end_time_s": "2530",
//     "program_date": "20250628",
//     "program_url": "https://www.mbs1179.com/yaru/",
//     "station_id": "MBS",
//     "performer": "極楽とんぼ、河合郁人、さらば青春の光（週替わり）、トム・ブラウン（週替わり）、小沢一敬（週替わり）（スピードワゴン）、大谷映美里（=LOVE）、池田裕子",
//     "title": "アッパレやってまーす！～土曜日です～",
//     "info": "",
//     "description": "メールアドレス：\u003ca href=mailto:yarudo@mbs1179.com target=_blank\u003eyarudo@mbs1179.com\u003c/a\u003e\u003cbr /\u003e\u003cbr /\u003e\u003cbr /\u003e◆アッパレやってまーす！&#65374;土曜日です&#65374;番組サイト◆\u003cbr /\u003e☆番組ホームページ：\u003ca href='https://www.mbs1179.com/yaru/' target=_blank\u003eこちらをクリック\u003c/a\u003e\u003cbr /\u003e☆X（旧Twitter）：\u003ca href='https://twitter.com/mbs_yarudo/' target=_blank\u003e@mbs_yarudo\u003c/a\u003e",
//     "status": "past",
//     "img": "https://program-static.cf.radiko.jp/6ff0b838-2453-4734-ad79-0be88a84b425.jpeg",
//     "genre": {
//       "personality": {
//         "id": "C010",
//         "name": "タレント"
//       },
//       "program": {
//         "id": "P006",
//         "name": "バラエティ"
//       }
//     },
//     "ts_in_ng": 0,
//     "ts_out_ng": 0,
//     "tsplus_in_ng": 0,
//     "tsplus_out_ng": 0,
//     "metas": [
//       {
//         "name": "twitter",
//         "value": "#radiko"
//       }
//     ]
//   },
//   {
//     "start_time": "2025-07-06 00:00:00",
//     "end_time": "2025-07-06 01:30:00",
//     "start_time_s": "2400",
//     "end_time_s": "2530",
//     "program_date": "20250705",
//     "program_url": "https://www.mbs1179.com/yaru/",
//     "station_id": "MBS",
//     "performer": "極楽とんぼ、河合郁人、さらば青春の光（週替わり）、トム・ブラウン（週替わり）、小沢一敬（週替わり）（スピードワゴン）、大谷映美里（=LOVE）、池田裕子",
//     "title": "アッパレやってまーす！～土曜日です～",
//     "info": "",
//     "description": "メールアドレス：\u003ca href=mailto:yarudo@mbs1179.com target=_blank\u003eyarudo@mbs1179.com\u003c/a\u003e\u003cbr /\u003e\u003cbr /\u003e\u003cbr /\u003e◆アッパレやってまーす！&#65374;土曜日です&#65374;番組サイト◆\u003cbr /\u003e☆番組ホームページ：\u003ca href='https://www.mbs1179.com/yaru/' target=_blank\u003eこちらをクリック\u003c/a\u003e\u003cbr /\u003e☆X（旧Twitter）：\u003ca href='https://twitter.com/mbs_yarudo/' target=_blank\u003e@mbs_yarudo\u003c/a\u003e",
//     "status": "future",
//     "img": "https://program-static.cf.radiko.jp/6ff0b838-2453-4734-ad79-0be88a84b425.jpeg",
//     "genre": {
//       "personality": {
//         "id": "C010",
//         "name": "タレント"
//       },
//       "program": {
//         "id": "P006",
//         "name": "バラエティ"
//       }
//     },
//     "ts_in_ng": 0,
//     "ts_out_ng": 0,
//     "tsplus_in_ng": 0,
//     "tsplus_out_ng": 0,
//     "metas": [
//       {
//         "name": "twitter",
//         "value": "#radiko"
//       }
//     ]
//   },
// ]
// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityGenre {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramGenre {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genre {
    pub personality: Option<PersonalityGenre>,
    pub program: Option<ProgramGenre>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub start_time: String,
    pub end_time: String,
    pub start_time_s: String,
    pub end_time_s: String,
    pub program_date: String,
    pub program_url: String,
    pub station_id: String,
    pub performer: String,
    pub title: String,
    pub info: String,
    pub description: String,
    pub status: String,
    pub img: String,
    pub genre: Option<Genre>,
    pub ts_in_ng: i32,
    pub ts_out_ng: i32,
    pub tsplus_in_ng: i32,
    pub tsplus_out_ng: i32,
    pub metas: Vec<Meta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub data: Vec<SearchResult>,
}
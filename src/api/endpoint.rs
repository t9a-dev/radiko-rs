const V2_URL: &str = "https://radiko.jp/v2/";
const V3_URL: &str = "https://radiko.jp/v3/";
const V4_URL: &str = "https://radiko.jp/v4/";
const API_URL: &str = "https://api.radiko.jp/";
const AREA_URL: &str = "https://radiko.jp/area/";

pub struct RadikoEndpoint {}

impl RadikoEndpoint {
    pub const RADIKO_HOST: &str = "https://radiko.jp";
    /// radiko_session取得に利用
    pub const LOGIN_CHECK_URL: &str = "https://radiko.jp/ap/member/webapi/v2/member/login/check";

    pub fn area_id_endpoint() -> String {
        AREA_URL.to_string()
    }

    pub fn login_endpoint() -> String {
        format!("{}api/member/login", V4_URL)
    }

    pub fn auth1_endpoint() -> String {
        format!("{}api/auth1", V2_URL)
    }

    pub fn auth2_endpoint() -> String {
        format!("{}api/auth2", V2_URL)
    }

    pub fn search_endpoint() -> String {
        format!("{}api/program/search", V3_URL)
    }

    pub fn station_list_endpoint(area_id: &str) -> String {
        format!("{}station/list/{}.xml", V3_URL, area_id)
    }

    // https://api.radiko.jp/program/v3/now/JP13.xml
    pub fn now_on_air_programs(area_id: &str) -> String {
        format!("{}program/v3/now/{}.xml", API_URL, area_id)
    }
    pub fn weekly_programs_endpoint(station_id: &str) -> String {
        format!("{}program/v3/weekly/{}.xml", API_URL, station_id)
    }

    pub fn stream_url_list_endpoint(station_id: &str) -> String {
        format!("{}station/stream/pc_html5/{}.xml", V3_URL, station_id)
    }

    pub fn stream_url(station_id: &str, stream_url: &str) -> String {
        format!("{}?station_id={}&l=15&lsid=&type=b", stream_url, station_id)
    }

    /// HLSストリーミングのMasterPlaylist.m3u8を返すエンドポイントを取得
    /// radikoによる仕様変更時にはエンドポイント自体が変更されたり、クエリパラメータが変更される模様
    pub fn playlist_create_url_endpoint(station_id: &str, lsid: &str) -> String {
        format!(
            "https://si-f-radiko.smartstream.ne.jp/so/playlist.m3u8?station_id={}&l=15&lsid={}&type=b",
            station_id, lsid
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::api::endpoint::RadikoEndpoint;

    #[test]
    fn area_id_endpoint_test() {
        let get_area_id_endpoint = RadikoEndpoint::area_id_endpoint();
        assert_eq!(get_area_id_endpoint, "https://radiko.jp/area/");
    }

    #[test]
    fn auth1_endpoint_test() {
        let get_auth1_endpoint = RadikoEndpoint::auth1_endpoint();
        assert_eq!(get_auth1_endpoint, "https://radiko.jp/v2/api/auth1");
    }

    #[test]
    fn auth2_endpoint_test() {
        let get_auth2_endpoint = RadikoEndpoint::auth2_endpoint();
        assert_eq!(get_auth2_endpoint, "https://radiko.jp/v2/api/auth2");
    }

    #[test]
    fn stream_url_list_endpoint_test() {
        let get_stream_url_list_endpoint = RadikoEndpoint::stream_url_list_endpoint("TBS");
        assert_eq!(
            get_stream_url_list_endpoint,
            "https://radiko.jp/v3/station/stream/pc_html5/TBS.xml"
        );
    }

    #[test]
    fn search_endpoint_test() {
        assert_eq!(
            "https://radiko.jp/v3/api/program/search",
            RadikoEndpoint::search_endpoint()
        );
    }

    #[test]
    fn stations_endpoint() {
        let area_id = "JP13";
        assert_eq!(
            format!("https://radiko.jp/v3/station/list/{}.xml", area_id),
            RadikoEndpoint::station_list_endpoint(area_id)
        );
    }

    #[test]
    fn now_on_air_programs() {
        let area_id = "JP13";
        assert_eq!(
            format!("https://api.radiko.jp/program/v3/now/{}.xml", area_id),
            RadikoEndpoint::now_on_air_programs(area_id)
        );
    }

    #[test]
    fn weekly_programs_endpoint() {
        let station_id = "TBS";
        assert_eq!(
            format!("https://api.radiko.jp/program/v3/weekly/{}.xml", station_id),
            RadikoEndpoint::weekly_programs_endpoint(station_id)
        );
    }

    #[test]
    fn playlist_create_url_endpoint_test() {
        let station_id = "TBS";
        let lsid = crate::utils::generate_md5_hash();
        let playlist_crate_url = RadikoEndpoint::playlist_create_url_endpoint(station_id, &lsid);
        assert_eq!(
            playlist_crate_url,
            format!(
                "https://si-f-radiko.smartstream.ne.jp/so/playlist.m3u8?station_id={}&l=15&lsid={}&type=b",
                station_id, lsid
            )
        )
    }
}

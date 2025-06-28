const V2_URL: &str = "https://radiko.jp/v2/";
const V3_URL: &str = "https://radiko.jp/v3/";
const AREA_URL: &str = "https://radiko.jp/area/";

pub struct RadikoEndpoint {}

impl RadikoEndpoint {
    pub fn get_area_id_endpoint() -> String {
        "https://radiko.jp/area/".to_string()
    }

    pub fn get_auth1_endpoint() -> String {
        format!("{}api/auth1", V2_URL)
    }

    pub fn get_auth2_endpoint() -> String {
        format!("{}api/auth2", V2_URL)
    }

    pub fn get_stream_url_list_endpoint(station_id: &str) -> String {
        // https://radiko.jp/v3/station/stream/pc_html5/TBS.xml
        format!("{}station/stream/pc_html5/{}.xml", V3_URL, station_id)
    }
    
    /// HLSストリーミングのMasterPlaylist.m3u8を返すエンドポイントを取得
    /// TODO: 固定のエンドポイントを返すが、radikoの仕様変更で定期的に変わるのでget_stream_url_ilst_endpointから
    /// 自動で取得する方法が望ましい。
    pub fn get_playlist_create_url_endpoint(station_id: &str) -> String{
        // https://si-f-radiko.smartstream.ne.jp/so/playlist.m3u8?station_id=TBS&l=15&lsid=&type=b
        format!("https://si-f-radiko.smartstream.ne.jp/so/playlist.m3u8?station_id={}&l=15&lsid=&type=b",station_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::endpoint::RadikoEndpoint;

    #[test]
    fn get_area_id_endpoint_test() {
        let get_area_id_endpoint = RadikoEndpoint::get_area_id_endpoint();
        assert_eq!(get_area_id_endpoint, "https://radiko.jp/area/");
    }

    #[test]
    fn get_auth1_endpoint_test() {
        let get_auth1_endpoint = RadikoEndpoint::get_auth1_endpoint();
        assert_eq!(get_auth1_endpoint, "https://radiko.jp/v2/api/auth1");
    }

    #[test]
    fn get_auth2_endpoint_test() {
        let get_auth2_endpoint = RadikoEndpoint::get_auth2_endpoint();
        assert_eq!(get_auth2_endpoint, "https://radiko.jp/v2/api/auth2");
    }

    #[test]
    fn get_stream_url_list_endpoint_test() {
        let get_stream_url_list_endpoint = RadikoEndpoint::get_stream_url_list_endpoint("TBS");
        assert_eq!(
            get_stream_url_list_endpoint,
            "https://radiko.jp/v3/station/stream/pc_html5/TBS.xml"
        );
    }

    #[test]
    fn get_playlist_create_url_endpoint_test(){
        let station_id = "TBS";
        let playlist_crate_url = RadikoEndpoint::get_playlist_create_url_endpoint(station_id);
        assert_eq!(
            playlist_crate_url,
            format!("https://si-f-radiko.smartstream.ne.jp/so/playlist.m3u8?station_id={}&l=15&lsid=&type=b",station_id)
        )
    }
}

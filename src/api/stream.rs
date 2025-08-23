use std::{borrow::Cow, convert::TryFrom, io::Write, sync::Arc};

use anyhow::{Result, anyhow};
use hls_m3u8::MasterPlaylist;
use tempfile::NamedTempFile;

use super::{auth::RadikoAuthManager, endpoint::RadikoEndpoint};

pub struct RadikoStream {
    inner: Arc<RadikoStreamRef>,
}

struct RadikoStreamRef {
    auth_manager: Arc<RadikoAuthManager>,
}

impl RadikoStream {
    pub fn new(radiko_auth_manager: Arc<RadikoAuthManager>) -> Self {
        Self {
            inner: Arc::new(RadikoStreamRef {
                auth_manager: radiko_auth_manager.clone(),
            }),
        }
    }

    pub fn stream_url(&self, station_id: &str) -> String {
        let lsid = &self.inner.auth_manager.lsid().to_string();
        if self.inner.auth_manager.area_free() {
            RadikoEndpoint::area_free_playlist_create_url_endpoint(station_id, &lsid)
        } else {
            RadikoEndpoint::playlist_create_url_endpoint(station_id, &lsid)
        }
    }

    #[allow(dead_code)]
    pub async fn get_hls_master_playlist_content(&self, station_id: &str) -> Result<Cow<str>> {
        let master_playlist_res = self
            .inner
            .auth_manager
            .http_client()
            .get(&self.stream_url(station_id))
            .send()
            .await?;

        if !master_playlist_res.status().is_success() {
            return Err(anyhow!(
                "get hls master playlist error: {:#?}, client_info: {:#?}",
                master_playlist_res.text().await?,
                self.inner.auth_manager.http_client()
            ));
        }

        Ok(master_playlist_res.text().await?.into())
    }

    #[allow(dead_code)]
    pub fn extract_medialist_url(&self, master_playlist_content: &str) -> Result<Cow<str>> {
        let master_playlist = match MasterPlaylist::try_from(master_playlist_content) {
            Ok(master_playlist) => master_playlist,
            Err(err) => panic!(
                "extract_medialist_url error: {:#?},master_playlist_content: {:#?}",
                err, master_playlist_content
            ),
        };
        Ok(master_playlist
            .variant_streams
            .iter()
            .next()
            .ok_or_else(|| anyhow!("No stream found in master playlist"))
            .and_then(|stream| match stream {
                hls_m3u8::tags::VariantStream::ExtXStreamInf { uri, .. } => Ok(uri.to_string()),
                _ => Err(anyhow!("Invalid stream type")),
            })?
            .into())
    }

    #[allow(dead_code)]
    pub async fn download_playlist_to_tempfile(&self, station_id: &str) -> Result<NamedTempFile> {
        let playlist_content = self
            .inner
            .auth_manager
            .http_client()
            .get(&self.stream_url(station_id))
            .send()
            .await?
            .bytes()
            .await?;

        let mut temp_file = NamedTempFile::with_suffix(".m3u8")?;
        temp_file.write_all(&playlist_content)?;
        temp_file.flush()?;

        Ok(temp_file)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::auth::RadikoAuthManager;
    use crate::utils::load_env;
    use crate::{api::stream::RadikoStream, radiko::Radiko};
    use std::{env, process::Stdio};

    use anyhow::Result;

    use tokio::{
        io::{AsyncBufReadExt, BufReader},
        process::Command,
    };

    #[tokio::test]
    async fn hls_m3u8_playground() -> Result<()> {
        let station_id = "TBS";
        let radiko_stream = RadikoStream::new(RadikoAuthManager::new().await.into());

        let master_playlist_content = radiko_stream
            .get_hls_master_playlist_content(station_id)
            .await?;
        let segment_uri = radiko_stream.extract_medialist_url(&master_playlist_content)?;

        println!("parsed_uri: {}", segment_uri);

        Ok(())
    }

    #[tokio::test]
    async fn stream_url_test() -> Result<()> {
        let radiko = Radiko::new().await;
        let available_stations = radiko
            .stations_from_area_id(&radiko.area_id().await)
            .await?;
        let station_id = available_stations.data.get(0).unwrap().id.clone();

        run_ffmpeg_command_stream(radiko, &station_id).await?;

        Ok(())
    }

    #[tokio::test]
    async fn area_free_stream_url_test() -> Result<()> {
        load_env();
        let mail = env::var("mail").expect("failed mail from dotenv");
        let pass = env::var("pass").expect("failed pass from dotenv");
        let station_id = "MBS";
        let radiko = Radiko::new_area_free(&mail, &pass).await;

        println!("station_id: {}", station_id);

        run_ffmpeg_command_stream(radiko, station_id).await?;

        Ok(())
    }

    async fn run_ffmpeg_command_stream(radiko: Radiko, station_id: &str) -> Result<()> {
        let strem_url = radiko.stream_url(station_id).await;
        let token = radiko.auth_token().await.to_string();

        let cmd = Command::new("ffmpeg")
            .args([
                "-loglevel",
                "debug",
                "-protocol_whitelist",
                "file,http,https,tcp,tls,crypto",
                "-headers",
                &format!("X-Radiko-Authtoken: {}\r\n", token),
                "-allowed_extensions",
                "ALL",
                "-seekable",
                "0",
                "-http_seekable",
                "0",
                "-f",
                "hls",
                "-i",
                &strem_url,
                "-reconnect",
                "3",
                "-reconnect_at_eof",
                "1",
                "-reconnect_streamed",
                "1",
                "-reconnect_delay_max",
                "5",
                "-timeout",
                "10000000",
                "-http_persistent",
                "1",
                "-acodec",
                "copy",
                "-vn",
                "-bsf:a",
                "aac_adtstoasc",
                "-t",
                "10",
                "-y",
                "output.aac",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn();

        match cmd {
            Ok(mut child) => {
                let process_id = child.id().unwrap_or(0);
                let stream_id = process_id;

                println!(
                    "FFmpeg process started for stream {} with PID {}",
                    stream_id, process_id
                );

                // stderrをログ出力用に監視
                if let Some(stderr) = child.stderr.take() {
                    let reader = BufReader::new(stderr);
                    let mut lines = reader.lines();
                    let log_stream_id = stream_id.clone();

                    tokio::spawn(async move {
                        while let Ok(Some(line)) = lines.next_line().await {
                            if line.contains("error") || line.contains("Error") {
                                println!("FFmpeg {}: {}", log_stream_id, line);
                            } else if line.contains("frame=") || line.contains("time=") {
                                // プログレス情報は詳細レベルで
                                println!("FFmpeg {}: {}", log_stream_id, line);
                            } else {
                                println!("FFmpeg {}: {}", log_stream_id, line);
                            }
                        }
                    });
                }

                // プロセス終了を待機
                match child.wait().await {
                    Ok(status) => {
                        if status.success() {
                            println!("FFmpeg process for exited successfully");
                            Ok(())
                        } else {
                            Err(anyhow::anyhow!(
                                "FFmpeg process for exited with error: {}",
                                status
                            ))
                        }
                    }
                    Err(e) => Err(anyhow::anyhow!(
                        "Error waiting for FFmpeg process {}: {}",
                        stream_id,
                        e
                    )),
                }
            }
            Err(e) => Err(anyhow::anyhow!("Failed to start ffmpeg for stream : {}", e)),
        }
    }
}

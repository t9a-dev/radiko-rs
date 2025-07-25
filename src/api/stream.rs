use std::{convert::TryFrom, io::Write, sync::Arc};

use anyhow::{Result, anyhow};
use hls_m3u8::MasterPlaylist;
use reqwest::Client;
use tempfile::NamedTempFile;

use crate::client::RadikoClient;

use super::endpoint::RadikoEndpoint;

pub struct RadikoStream {
    inner: Arc<RadikoStreamRef>,
}

struct RadikoStreamRef {
    station_id: String,
    radiko_client: RadikoClient,
    stream_url: String,
}

impl RadikoStream {
    pub fn new(station_id: &str, radiko_client: RadikoClient) -> Self {
        Self {
            inner: Arc::new(RadikoStreamRef {
                station_id: station_id.to_string(),
                radiko_client: radiko_client.clone(),
                stream_url: RadikoEndpoint::get_playlist_create_url_endpoint(
                    station_id,
                    &radiko_client.get_auth_manager().get_lsid(),
                ),
            }),
        }
    }

    pub fn get_station_id(&self) -> String {
        self.inner.station_id.clone()
    }

    pub fn get_http_client(&self) -> Client {
        self.inner.radiko_client.get_http_client()
    }

    pub async fn get_hls_master_playlist(&self) -> Result<String> {
        Ok(self
            .get_http_client()
            .get(&self.inner.stream_url)
            .send()
            .await?
            .text()
            .await?)
    }

    pub fn get_stream_url(&self) -> &str {
        &self.inner.stream_url
    }

    pub fn extract_medialist_url(&self, master_playlist_content: &str) -> Result<String> {
        let master_playlist = MasterPlaylist::try_from(master_playlist_content)?;
        Ok(master_playlist
            .variant_streams
            .iter()
            .next()
            .ok_or_else(|| anyhow!("No stream found in master playlist"))
            .and_then(|stream| match stream {
                hls_m3u8::tags::VariantStream::ExtXStreamInf { uri, .. } => Ok(uri.to_string()),
                _ => Err(anyhow!("Invalid stream type")),
            })?)
    }

    pub async fn download_playlist_to_tempfile(&self) -> Result<NamedTempFile> {
        let playlist_content = self
            .get_http_client()
            .get(self.get_stream_url())
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
    use std::process::Stdio;

    use crate::api::auth::RadikoAuthManager;
    use crate::api::stream::RadikoStream;
    use crate::client::RadikoClient;
    use anyhow::Result;

    use tokio::{
        io::{AsyncBufReadExt, BufReader},
        process::Command,
    };

    #[tokio::test]
    async fn hls_m3u8_playground() -> Result<()> {
        let station_id = "TBS";
        let radiko_auth_manager = RadikoAuthManager::new().await;
        let radiko_client = RadikoClient::new(radiko_auth_manager.clone()).await;
        let radiko_stream = RadikoStream::new(station_id, radiko_client.clone());

        let master_playlist_content = radiko_stream.get_hls_master_playlist().await?;
        let segment_uri = radiko_stream.extract_medialist_url(&master_playlist_content)?;

        println!("parsed_uri: {}", segment_uri);

        Ok(())
    }

    #[tokio::test]
    async fn stream_url_test() -> Result<()> {
        let station_id = "TBS";
        let radiko_auth_manager = RadikoAuthManager::new().await;
        let radiko_client = RadikoClient::new(radiko_auth_manager.clone()).await;
        let radiko_stream = RadikoStream::new(station_id, radiko_client.clone());

        println!("radiko_auth_manager: {:#?}", radiko_auth_manager);
        println!("area_id: {}", radiko_client.get_area_id());
        println!("station_id: {}", station_id);

        run_ffmpeg_command_stream(radiko_stream, &radiko_auth_manager.get_auth_token()).await?;

        Ok(())
    }

    async fn run_ffmpeg_command_stream(
        radiko_stream: RadikoStream,
        radiko_auth_token: &str,
    ) -> Result<()> {
        let strem_url = radiko_stream.get_stream_url();

        let cmd = Command::new("ffmpeg")
            .args([
                "-loglevel",
                "debug",
                "-protocol_whitelist",
                "file,http,https,tcp,tls,crypto",
                "-headers",
                &format!("X-Radiko-Authtoken: {}\r\n", radiko_auth_token),
                "-allowed_extensions",
                "ALL",
                "-seekable",
                "0",
                "-http_seekable",
                "0",
                "-f",
                "hls",
                "-i",
                strem_url,
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

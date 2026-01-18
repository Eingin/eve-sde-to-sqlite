use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::io::{Read, Write};
use std::path::Path;

use crate::ui::Ui;

const LATEST_URL: &str = "https://developers.eveonline.com/static-data/tranquility/latest.jsonl";
const ZIP_URL: &str =
    "https://developers.eveonline.com/static-data/eve-online-static-data-latest-jsonl.zip";

#[derive(Debug, Deserialize)]
pub struct SdeInfo {
    #[serde(rename = "_key")]
    pub key: String,
    #[serde(rename = "buildNumber")]
    pub build_number: u64,
    #[serde(rename = "releaseDate")]
    pub release_date: String,
}

pub struct SdeClient {
    client: Client,
}

impl SdeClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent("eve-sde-to-sqlite")
            .build()
            .context("Failed to create HTTP client")?;
        Ok(Self { client })
    }

    /// Fetch the latest SDE build info
    pub fn fetch_latest_info(&self) -> Result<SdeInfo> {
        let response = self
            .client
            .get(LATEST_URL)
            .send()
            .context("Failed to fetch latest SDE info")?;

        let text = response.text().context("Failed to read response")?;
        let info: SdeInfo = serde_json::from_str(&text).context("Failed to parse SDE info")?;

        Ok(info)
    }

    /// Download the SDE zip file to the given path
    pub fn download_zip(&self, dest: &Path, ui: &mut impl Ui) -> Result<()> {
        let response = self
            .client
            .get(ZIP_URL)
            .send()
            .context("Failed to start download")?;

        let total_size = response.content_length().unwrap_or(0);

        let mut file = std::fs::File::create(dest).context("Failed to create destination file")?;

        let mut downloaded: u64 = 0;
        let mut buffer = [0u8; 8192];
        let mut reader = response;

        loop {
            let bytes_read = reader
                .read(&mut buffer)
                .context("Failed to read from response")?;

            if bytes_read == 0 {
                break;
            }

            file.write_all(&buffer[..bytes_read])
                .context("Failed to write to file")?;

            downloaded += bytes_read as u64;
            ui.set_progress(downloaded, total_size, format_bytes(downloaded, total_size));
        }

        ui.log("Download complete");
        Ok(())
    }
}

/// Format bytes as human-readable string
fn format_bytes(current: u64, total: u64) -> String {
    fn fmt(bytes: u64) -> String {
        if bytes >= 1_000_000_000 {
            format!("{:.1} GB", bytes as f64 / 1_000_000_000.0)
        } else if bytes >= 1_000_000 {
            format!("{:.1} MB", bytes as f64 / 1_000_000.0)
        } else if bytes >= 1_000 {
            format!("{:.1} KB", bytes as f64 / 1_000.0)
        } else {
            format!("{} B", bytes)
        }
    }
    format!("{} / {}", fmt(current), fmt(total))
}

impl Default for SdeClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500, 999), "500 B / 999 B");
        assert_eq!(format_bytes(1500, 3000), "1.5 KB / 3.0 KB");
        assert_eq!(format_bytes(1_500_000, 3_000_000), "1.5 MB / 3.0 MB");
    }
}

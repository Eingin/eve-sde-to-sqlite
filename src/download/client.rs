use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::io::{Read, Write};
use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};

const LATEST_URL: &str = "https://developers.eveonline.com/static-data/tranquility/latest.jsonl";
const ZIP_URL: &str = "https://developers.eveonline.com/static-data/eve-online-static-data-latest-jsonl.zip";

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
        let response = self.client
            .get(LATEST_URL)
            .send()
            .context("Failed to fetch latest SDE info")?;

        let text = response.text().context("Failed to read response")?;
        let info: SdeInfo = serde_json::from_str(&text)
            .context("Failed to parse SDE info")?;
        
        Ok(info)
    }

    /// Download the SDE zip file to the given path
    pub fn download_zip(&self, dest: &Path) -> Result<()> {
        let response = self.client
            .get(ZIP_URL)
            .send()
            .context("Failed to start download")?;

        let total_size = response.content_length().unwrap_or(0);
        
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("=>-"));
        pb.set_message("Downloading SDE");

        let mut file = std::fs::File::create(dest)
            .context("Failed to create destination file")?;

        let mut downloaded: u64 = 0;
        let mut buffer = [0u8; 8192];
        let mut reader = response;

        loop {
            let bytes_read = reader.read(&mut buffer)
                .context("Failed to read from response")?;
            
            if bytes_read == 0 {
                break;
            }

            file.write_all(&buffer[..bytes_read])
                .context("Failed to write to file")?;
            
            downloaded += bytes_read as u64;
            pb.set_position(downloaded);
        }

        pb.finish_with_message("Download complete");
        Ok(())
    }
}

impl Default for SdeClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}

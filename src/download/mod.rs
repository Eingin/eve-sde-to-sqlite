pub mod cache;
pub mod client;
pub mod extract;

pub use cache::*;
pub use client::*;
pub use extract::*;

use anyhow::Result;
use std::path::PathBuf;

use crate::ui::{Phase, Ui};

/// Download the SDE if not cached, return path to extracted directory
pub fn ensure_sde_downloaded(
    cache_dir: Option<PathBuf>,
    force: bool,
    ui: &mut impl Ui,
) -> Result<(PathBuf, u64)> {
    let cache = CacheManager::new(cache_dir)?;
    let client = SdeClient::new()?;

    // Get latest build info
    ui.set_phase(Phase::Checking);
    ui.log("Checking for latest SDE version...");
    let info = client.fetch_latest_info()?;
    ui.set_info(format!(
        "Build {} ({})",
        info.build_number, info.release_date
    ));
    ui.log(format!(
        "Latest build: {} ({})",
        info.build_number, info.release_date
    ));

    let build_dir = cache.build_dir(info.build_number);

    // Check if already cached
    if !force && cache.is_cached(info.build_number) {
        ui.log(format!("Using cached SDE from {:?}", build_dir));
        return Ok((build_dir, info.build_number));
    }

    // Download zip
    ui.set_phase(Phase::Downloading);
    let zip_path = cache.zip_path(info.build_number);
    ui.log(format!("Downloading SDE build {}...", info.build_number));
    client.download_zip(&zip_path, ui)?;

    // Extract zip
    ui.set_phase(Phase::Extracting);
    ui.log(format!("Extracting to {:?}...", build_dir));
    extract_zip(&zip_path, &build_dir, ui)?;

    // Clean up zip file
    std::fs::remove_file(&zip_path).ok();

    // Clean up old builds
    cache.cleanup_old_builds(info.build_number).ok();

    Ok((build_dir, info.build_number))
}

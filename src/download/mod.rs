pub mod cache;
pub mod client;
pub mod extract;

pub use cache::*;
pub use client::*;
pub use extract::*;

use anyhow::Result;
use std::path::PathBuf;

/// Download the SDE if not cached, return path to extracted directory
pub fn ensure_sde_downloaded(
    cache_dir: Option<PathBuf>,
    force: bool,
) -> Result<(PathBuf, u64)> {
    let cache = CacheManager::new(cache_dir)?;
    let client = SdeClient::new()?;

    // Get latest build info
    println!("Checking for latest SDE version...");
    let info = client.fetch_latest_info()?;
    println!("Latest build: {} ({})", info.build_number, info.release_date);

    let build_dir = cache.build_dir(info.build_number);

    // Check if already cached
    if !force && cache.is_cached(info.build_number) {
        println!("Using cached SDE from {:?}", build_dir);
        return Ok((build_dir, info.build_number));
    }

    // Download zip
    let zip_path = cache.zip_path(info.build_number);
    println!("Downloading SDE build {}...", info.build_number);
    client.download_zip(&zip_path)?;

    // Extract zip
    println!("Extracting to {:?}...", build_dir);
    extract_zip(&zip_path, &build_dir)?;

    // Clean up zip file
    std::fs::remove_file(&zip_path).ok();

    // Clean up old builds
    cache.cleanup_old_builds(info.build_number).ok();

    Ok((build_dir, info.build_number))
}

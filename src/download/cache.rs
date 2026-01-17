use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::path::{Path, PathBuf};
use std::fs;

pub struct CacheManager {
    cache_dir: PathBuf,
}

impl CacheManager {
    pub fn new(custom_dir: Option<PathBuf>) -> Result<Self> {
        let cache_dir = match custom_dir {
            Some(dir) => dir,
            None => {
                let proj_dirs = ProjectDirs::from("", "", "eve-sde-to-sqlite")
                    .context("Could not determine cache directory")?;
                proj_dirs.cache_dir().to_path_buf()
            }
        };

        fs::create_dir_all(&cache_dir)
            .context("Failed to create cache directory")?;

        Ok(Self { cache_dir })
    }

    /// Get the cache directory path
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    /// Get path to build-specific directory
    pub fn build_dir(&self, build_number: u64) -> PathBuf {
        self.cache_dir.join(build_number.to_string())
    }

    /// Check if a build is already cached
    pub fn is_cached(&self, build_number: u64) -> bool {
        let build_dir = self.build_dir(build_number);
        build_dir.exists() && build_dir.join("types.jsonl").exists()
    }

    /// Get path to zip file for a build
    pub fn zip_path(&self, build_number: u64) -> PathBuf {
        self.cache_dir.join(format!("{}.zip", build_number))
    }

    /// Clean up old cached builds, keeping only the specified one
    pub fn cleanup_old_builds(&self, keep_build: u64) -> Result<()> {
        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if let Ok(build) = name.parse::<u64>() {
                        if build != keep_build {
                            fs::remove_dir_all(&path).ok();
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

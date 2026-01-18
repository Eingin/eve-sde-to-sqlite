use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::Path;
use zip::ZipArchive;

/// Extract a zip file to the destination directory
pub fn extract_zip(zip_path: &Path, dest_dir: &Path) -> Result<()> {
    let file = File::open(zip_path).context("Failed to open zip file")?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader).context("Failed to read zip archive")?;

    fs::create_dir_all(dest_dir).context("Failed to create destination directory")?;

    let total_files = archive.len();
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} files")
            .unwrap()
            .progress_chars("=>-"),
    );
    pb.set_message("Extracting");

    for i in 0..total_files {
        let mut file = archive
            .by_index(i)
            .context("Failed to read file from archive")?;

        // Get the file name, stripping any directory prefix
        let name = file.name();
        let file_name = Path::new(name)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(name)
            .to_string();

        // Only extract .jsonl files
        if !file_name.ends_with(".jsonl") {
            pb.inc(1);
            continue;
        }

        let dest_path = dest_dir.join(&file_name);
        let mut dest_file = File::create(&dest_path)
            .with_context(|| format!("Failed to create file: {:?}", dest_path))?;

        io::copy(&mut file, &mut dest_file)
            .with_context(|| format!("Failed to extract: {}", file_name))?;

        pb.inc(1);
    }

    pb.finish_with_message("Extraction complete");
    Ok(())
}

use std::{fs::File, io::Write, path::Path};

use anyhow::Result;
use indicatif::ProgressBar;

pub async fn download_file(url: &str, path: &Path) -> Result<()> {
    let client = reqwest::Client::new();
    let mut response = client.get(url).send().await?;

    let total_size = response.content_length().unwrap_or(0);
    let pb = ProgressBar::new(total_size);

    let mut file = File::create(path)?;
    let mut downloaded: u64 = 0;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete");
    Ok(())
}

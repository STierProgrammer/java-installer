use std::{
    env,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::{Parser, Subcommand};
use serde_json::Value;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ListVersions { version: Option<u8> },
    ListAvaliableReleases,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ListVersions { version } => list_versions(version).await,
        Commands::ListAvaliableReleases => available_releases().await,
    }
}

async fn available_releases() -> Result<(), anyhow::Error> {
    let response = reqwest::get("https://api.adoptium.net/v3/info/available_releases")
        .await?
        .json::<Value>()
        .await?;
    let versions = response.as_object().context("Invalid json!")?;

    let lts_releases = versions.get("available_lts_releases");
    if let Some(lts_releases) = lts_releases {
        for lts_release in lts_releases.as_array().context("Invalid json!")? {
            let lts_release_data = lts_release.as_number().unwrap();
            println!("[lts-releases]- {}", lts_release_data);
        }
    }

    let releases = versions.get("avaliable_releases");
    if let Some(releases) = releases {
        for release in releases.as_array().context("Invalid json!")? {
            let release_data = release.as_number().unwrap();
            println!("[releaes]- {}", release_data);
        }
    }

    let most_recent_feature_release = versions.get("most_recent_feature_release").unwrap();
    let most_recent_feature_version = versions.get("most_recent_feature_version").unwrap();
    let most_recent_lts = versions.get("most_recent_lts").unwrap();
    let tip_version = versions.get("tip_version").unwrap();

    println!(
        "[most-recent-feature-release]- {}",
        most_recent_feature_release
    );
    println!(
        "[most_recent_feature_version]- {}",
        most_recent_feature_version
    );
    println!("[most_recent_lts]- {}", most_recent_lts);
    println!("[tip_version]- {}", tip_version);

    Ok(())
}

async fn list_versions(feature_versions: Option<u8>) -> Result<(), anyhow::Error> {
    let url = match feature_versions {
        Some(v) => format!("https://api.adoptium.net/v3/assets/feature_releases/{v}/ga"),
        None => "https://api.adoptium.net/v3/assets/feature_releases".to_string(),
    };

    let response = reqwest::get(&url).await?.json::<Value>().await?;
    let versions = response.as_array().context("Invalid API response")?;

    println!("Available versions:");
    for version in versions {
        let version_data = version["version_data"]["semver"].as_str().unwrap_or("");
        println!("- {}", version_data);
    }

    Ok(())
}

/*
    This will be used later!
*/

#[warn(dead_code)]
fn get_os() -> String {
    match env::consts::OS {
        "linux" => "linux",
        "macos" => "mac",
        "windows" => "windows",
        _ => panic!("Unsupported OS!"),
    }
    .to_string()
}

#[warn(dead_code)]
fn get_arch() -> String {
    match env::consts::ARCH {
        "x86_64" => "x64",
        "aarch64" => "aarch64",
        _ => panic!("Unsupported architecture!"),
    }
    .to_string()
}

#[warn(dead_code)]
fn default_install_path() -> PathBuf {
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap();
    Path::new(&home_dir).join(".java").join("jdk")
}

#[warn(dead_code)]
fn extract_package(package_path: &Path, install_path: &Path) -> Result<(), anyhow::Error> {
    let file = File::open(package_path)?;
    let extension = package_path.extension().and_then(|s| s.to_str());

    match extension {
        Some("gz") => {
            let tar_gz = BufReader::new(file);
            let tar = flate2::read::GzDecoder::new(tar_gz);
            let mut archive = tar::Archive::new(tar);
            archive.unpack(install_path)?;
        }
        Some("zip") => {
            let mut archive = zip::ZipArchive::new(file)?;
            archive.extract(install_path)?;
        }
        _ => return Err(anyhow::anyhow!("Unsupported file format!")),
    }

    Ok(())
}

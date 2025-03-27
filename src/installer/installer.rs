use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use serde_json::Value;
use tempfile::TempDir;

use crate::installer::{download::download_file, env::set_environment_variables, extracter::extract_package, utils::{default_install_path, get_arch, get_os}};

pub async fn install_version(
    version: u8,
    path: Option<PathBuf>,
    package_type: String,
    force: bool,
) -> Result<()> {
    let os = get_os();
    let arch = get_arch();
    let package_type = package_type.to_lowercase();

    if !["jdk", "jre"].contains(&package_type.as_str()) {
        return Err(anyhow::anyhow!("Invalid package type. Must be 'jdk' or 'jre'"));
    }

    let url = format!(
        "https://api.adoptium.net/v3/assets/feature_releases/{version}/ga?\
        architecture={arch}&image_type={package_type}&os={os}&vendor=eclipse"
    );

    let response = reqwest::get(&url).await?.json::<Value>().await?;
    let assets = response.as_array().context("No assets found")?;
    let asset = assets.first().context("No matching version found")?;
    
    let version_data = &asset["version_data"];
    let semver = version_data["semver"]
        .as_str()
        .context("Invalid version data")?;

    let package = asset["binaries"][0]["package"]
        .as_object()
        .context("No package found")?;

    let download_url = package["link"].as_str().context("No download link")?;
    let package_name = package["name"].as_str().context("No package name")?;

    let install_path = path.unwrap_or_else(|| default_install_path(&package_type));
    let java_home = install_path.join(format!("{}-{}", package_type, semver));

    if java_home.exists() && !force {
        return Err(anyhow::anyhow!(
            "Java already exists at {}. Use --force to overwrite.",
            java_home.display()
        ));
    }

    fs::create_dir_all(&java_home)?;

    println!("Downloading {}...", package_name);
    let temp_dir = TempDir::new()?;
    let download_path = temp_dir.path().join(package_name);
    download_file(download_url, &download_path).await?;

    println!("Extracting package...");
    extract_package(&download_path, &java_home)?;

    println!("Setting environment variables...");
    set_environment_variables(&java_home)?;

    println!(
        "\nSuccessfully installed Java {} {} at {}",
        version,
        package_type.to_uppercase(),
        java_home.display()
    );

    Ok(())
}

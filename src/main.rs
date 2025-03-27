mod cli;
mod installer;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use installer::installer::install_version;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install {
            version,
            path,
            package_type,
            force,
        } => install_version(version, path, package_type, force).await,
    }
}
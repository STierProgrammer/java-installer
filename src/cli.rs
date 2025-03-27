use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Install {
        version: u8,

        #[arg(short, long)]
        path: Option<PathBuf>,

        #[arg(long, default_value_t = String::from("jdk"))]
        package_type: String,

        #[arg(short, long)]
        force: bool,
    },
}

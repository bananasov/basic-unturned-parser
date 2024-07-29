use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to the bundles directory
    #[arg(short, long, value_name = "PATH")]
    pub path: PathBuf,
}

use anyhow::{anyhow, Context};
use clap::Parser;
use masterbundle_collector::MasterBundle;
use std::path::{Path, PathBuf};

use torture_parser::get_file_stem;
use torture_parser::parser::{
    assets::{gun::ItemGunAsset, BaseAsset},
    Parser as _,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to the bundles directory
    #[arg(short, long, value_name = "PATH")]
    pub path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let bundle = MasterBundle::new(args.path.clone())?;
    let mut paths: Vec<&Path> = bundle
        .assets
        .iter()
        .map(|x| x.parent().unwrap())
        .filter(|p| !p.as_os_str().is_empty() && *p != Path::new(".") && *p != Path::new(".."))
        .collect();

    paths.sort_by_key(|path| get_file_stem(path).unwrap_or(path.to_str().unwrap().to_string()));
    paths.dedup_by(|a, b| a == b);

    for path in paths {
        let mut data_path: PathBuf = args.path.join(path);

        let stem = path
            .file_stem()
            .ok_or_else(|| anyhow!("Failed to get file stem for: {:#?}", path))?;
        let stem = stem
            .to_str()
            .ok_or_else(|| anyhow!("Failed to convert into a &str"))?;
        data_path.push(format!("{}.dat", stem));

        if let Ok(content) = std::fs::read_to_string(&data_path) {
            let directory = data_path
                .parent()
                .context("Failed to get the parent of data file")?;

            if let Ok(asset) = BaseAsset::parse(directory, &content) {
                if let torture_parser::parser::assets::Type::Gun = asset.r#type {
                    let gun = ItemGunAsset::parse(directory, &content)?;
                    println!("{:#?}", gun);
                }
            }
        }
    }

    Ok(())
}

use anyhow::{anyhow, Context};
use clap::Parser;
use masterbundle_collector::MasterBundle;
use std::path::{Path, PathBuf};
use torture_parser::parser::assets::gear::ItemGearAsset;
use torture_parser::parser::assets::glasses::ItemGlassesAsset;
use torture_parser::parser::assets::mask::ItemMaskAsset;
use torture_parser::parser::assets::parachute::ItemParachuteAsset;

use torture_parser::parser::assets::bag::ItemBagAsset;
use torture_parser::parser::assets::barrel::ItemBarrelAsset;
use torture_parser::parser::assets::barricade::ItemBarricadeAsset;
use torture_parser::parser::assets::consumable::ItemConsumableAsset;
use torture_parser::parser::assets::grip::ItemGripAsset;
use torture_parser::parser::assets::magazine::ItemMagazineAsset;
use torture_parser::parser::assets::optic::ItemOpticAsset;
use torture_parser::parser::assets::sentry::ItemSentryAsset;
use torture_parser::parser::assets::shirt::ItemShirtAsset;
use torture_parser::parser::assets::sight::ItemSightAsset;
use torture_parser::parser::assets::structure::ItemStructureAsset;
use torture_parser::parser::assets::tactical::ItemTacticalAsset;
use torture_parser::parser::assets::Type;

use torture_parser::get_file_stem;
use torture_parser::parser::{
    assets::{gun::ItemGunAsset, BaseAsset},
    Parser as TortureParser,
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
                match asset.r#type {
                    Type::Backpack | Type::Pants | Type::Vest => {
                        let bwa = ItemBagAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Gun => {
                        let bwa = ItemGunAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Sight => {
                        let bwa = ItemSightAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Tactical => {
                        let bwa = ItemTacticalAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Grip => {
                        let bwa = ItemGripAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Barrel => {
                        let bwa = ItemBarrelAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Magazine => {
                        let bwa = ItemMagazineAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Optic => {
                        let bwa = ItemOpticAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Medical => {
                        let bwa = ItemConsumableAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Barricade => {
                        let bwa = ItemBarricadeAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Structure => {
                        let bwa = ItemStructureAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Storage => {
                        let bwa = ItemStructureAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Sentry => {
                        let bwa = ItemSentryAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Shirt => {
                        let bwa = ItemShirtAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Cloud => {
                        let bwa = ItemParachuteAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Glasses => {
                        let bwa = ItemGlassesAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Mask => {
                        let bwa = ItemMaskAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Hat => {
                        let bwa = ItemGearAsset::parse(directory, &content)?;
                        println!("{:#?}", bwa);
                    }
                    Type::Unknown => {}
                    _ => println!("{:#?}", asset),
                }
            }
        }
    }

    Ok(())
}

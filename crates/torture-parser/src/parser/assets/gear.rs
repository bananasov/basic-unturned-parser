use anyhow::Context;

use super::clothing::ItemClothingAsset;
use super::Parser;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemGearAsset {
    #[serde(rename = "base")]
    pub item_clothing_asset: ItemClothingAsset,

    /// Specified if hair shows up when wearing.
    pub hair: bool,

    /// Specified if beard shows up when wearing.
    pub beard: bool,
}

impl Parser<ItemGearAsset> for ItemGearAsset {
    fn parse<P: AsRef<std::path::Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemGearAsset> {
        let item_clothing_asset = ItemClothingAsset::parse(directory, content)?;
        let mut item = ItemGearAsset {
            item_clothing_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");

            match field {
                "Hair" => item.hair = true,
                "Beard" => item.beard = true,
                _ => {}
            }
        }

        Ok(item)
    }
}

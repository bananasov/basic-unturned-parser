use std::path::Path;

use anyhow::Context;

use crate::parser::Parser;

use super::clothing::ItemClothingAsset;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemBagAsset {
    #[serde(rename = "base")]
    pub item_clothing_asset: ItemClothingAsset,

    /// Number of rows (vertical storage space).
    pub height: u8,

    /// Number of columns (horizontal storage space).
    pub width: u8,
}

impl Parser<ItemBagAsset> for ItemBagAsset {
    fn parse<P: AsRef<Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemBagAsset> {
        let item_clothing_asset = ItemClothingAsset::parse(directory, content)?;
        let mut item = ItemBagAsset {
            item_clothing_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Height" => {
                    item.height = value.parse().context("Failed to parse Height to a u8")?
                }
                "Width" => item.width = value.parse().context("Failed to parse Width to a u8")?,
                _ => {}
            }
        }

        Ok(item)
    }
}

use anyhow::Context;

use super::barricade::ItemBarricadeAsset;
use super::Parser;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemStorageAsset {
    #[serde(rename = "base")]
    pub item_barricade_asset: ItemBarricadeAsset,

    /// Number of rows (vertical storage space).
    pub height: u8,

    /// Number of columns (horizontal storage space).
    pub width: u8,

    /// If specified, the first item in the storage will be visibly displayed.
    pub display: bool,
}

impl Parser<ItemStorageAsset> for ItemStorageAsset {
    fn parse<P: AsRef<std::path::Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemStorageAsset> {
        let item_barricade_asset = ItemBarricadeAsset::parse(directory, content)?;
        let mut item = ItemStorageAsset {
            item_barricade_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Storage_Y" => {
                    item.height = value.parse().context("Failed to parse Storage_Y to a u8")?
                }
                "Storage_X" => {
                    item.width = value.parse().context("Failed to parse Storage_X to a u8")?
                }
                "Display" => item.display = true,
                _ => {}
            }
        }

        Ok(item)
    }
}

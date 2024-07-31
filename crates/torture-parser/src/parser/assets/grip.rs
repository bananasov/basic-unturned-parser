use std::path::Path;

use super::caliber::ItemCaliberAsset;
use super::Parser;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemGripAsset {
    #[serde(flatten)]
    pub item_caliber_asset: ItemCaliberAsset,

    pub is_bipod: bool,
}

impl Parser<ItemGripAsset> for ItemGripAsset {
    fn parse<P: AsRef<Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemGripAsset> {
        let item_caliber_asset = ItemCaliberAsset::parse(directory, content)?;
        let mut item = ItemGripAsset {
            item_caliber_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");

            if field == "Bipod" {
                item.is_bipod = true
            }
        }

        Ok(item)
    }
}

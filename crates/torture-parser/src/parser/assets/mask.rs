use super::gear::ItemGearAsset;
use super::Parser;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemMaskAsset {
    #[serde(rename = "base")]
    pub item_gear_asset: ItemGearAsset,

    pub is_earpiece: bool,
}

impl Parser<ItemMaskAsset> for ItemMaskAsset {
    fn parse<P: AsRef<std::path::Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemMaskAsset> {
        let item_gear_asset = ItemGearAsset::parse(directory, content)?;
        let mut item = ItemMaskAsset {
            item_gear_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");

            match field {
                "Earpiece" => item.is_earpiece = true,
                _ => {}
            }
        }

        Ok(item)
    }
}

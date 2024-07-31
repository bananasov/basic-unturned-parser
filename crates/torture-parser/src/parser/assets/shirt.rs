use super::bag::ItemBagAsset;
use super::Parser;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemShirtAsset {
    #[serde(rename = "base")]
    pub item_bag_asset: ItemBagAsset,

    pub ignore_hand: bool,
}

impl Parser<ItemShirtAsset> for ItemShirtAsset {
    fn parse<P: AsRef<std::path::Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemShirtAsset> {
        let item_bag_asset = ItemBagAsset::parse(directory, content)?;
        let mut item = ItemShirtAsset {
            item_bag_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");

            match field {
                "Ignore_Hand" => item.ignore_hand = true,
                _ => {}
            }
        }

        Ok(item)
    }
}

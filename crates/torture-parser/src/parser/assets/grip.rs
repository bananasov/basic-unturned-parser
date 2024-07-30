use std::path::Path;

use super::caliber::ItemCaliberAsset;
use super::Parser;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemGripAsset {
    #[serde(flatten)]
    base: ItemCaliberAsset,

    pub is_bipod: bool,
}

impl Parser<ItemGripAsset> for ItemGripAsset {
    fn parse<P: AsRef<Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemGripAsset> {
        let base = ItemCaliberAsset::parse(directory, content)?;
        let mut item = ItemGripAsset {
            base,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");

            match field {
                "Bipod" => item.is_bipod = true,
                _ => {}
            }
        }

        Ok(item)
    }
}

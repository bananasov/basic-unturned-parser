use std::path::Path;

use anyhow::Context;

use super::{BaseAsset, Parser};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemOpticAsset {
    #[serde(rename = "base")]
    pub base_asset: BaseAsset,

    ///  Multiplicative amount of zoom.
    pub zoom: f32,
}

impl Parser<ItemOpticAsset> for ItemOpticAsset {
    fn parse<P: AsRef<Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemOpticAsset> {
        let base_asset = BaseAsset::parse(directory, content)?;
        let mut item = ItemOpticAsset {
            base_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            if field == "Zoom" {
                item.zoom = value.parse().context("Failed to parse Zoom as f32")?
            }
        }

        Ok(item)
    }
}

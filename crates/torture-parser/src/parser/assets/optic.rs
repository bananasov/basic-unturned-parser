use std::path::Path;

use anyhow::Context;

use super::{BaseAsset, Parser};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemOpticAsset {
    #[serde(flatten)]
    base: BaseAsset,

    ///  Multiplicative amount of zoom.
    pub zoom: f32,
}

impl Parser<ItemOpticAsset> for ItemOpticAsset {
    fn parse<P: AsRef<Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemOpticAsset> {
        let base = BaseAsset::parse(directory, content)?;
        let mut item = ItemOpticAsset {
            base,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Zoom" => item.zoom = value.parse().context("Failed to parse Zoom as f32")?,
                _ => {}
            }
        }

        Ok(item)
    }
}

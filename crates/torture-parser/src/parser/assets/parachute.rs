use super::{BaseAsset, Parser};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemParachuteAsset {
    #[serde(rename = "base")]
    pub base_asset: BaseAsset,

    /// Decimal multiplier on the influence of gravity.
    pub gravity: f32,
}

impl Parser<ItemParachuteAsset> for ItemParachuteAsset {
    fn parse<P: AsRef<std::path::Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemParachuteAsset> {
        let base_asset = BaseAsset::parse(directory, content)?;
        let mut item = ItemParachuteAsset {
            base_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Gravity" => item.gravity = value.parse().unwrap_or_default(),
                _ => {}
            }
        }

        Ok(item)
    }
}

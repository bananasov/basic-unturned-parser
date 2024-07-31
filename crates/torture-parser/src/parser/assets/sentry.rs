use super::storage::ItemStorageAsset;
use super::Parser;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemSentryAsset {
    #[serde(rename = "base")]
    pub item_storage_asset: ItemStorageAsset,

    pub requires_power: bool,
    pub infinite_ammo: bool,
    pub infinite_quality: bool,
    pub detection_radius: f32,
    pub mode: Mode,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum Mode {
    #[default]
    Neutral,
    Friendly,
    Hostile,
}

impl Parser<ItemSentryAsset> for ItemSentryAsset {
    fn parse<P: AsRef<std::path::Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemSentryAsset> {
        let item_storage_asset = ItemStorageAsset::parse(directory, content)?;
        let mut item = ItemSentryAsset {
            item_storage_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Requires_Power" => item.requires_power = true,
                "Infinite_Ammo" => item.infinite_ammo = true,
                "Infinite_Quality" => item.infinite_quality = true,
                "Detection_Radius" => item.detection_radius = value.parse().unwrap_or(48.0),
                "Mode" => item.mode = value.into(),
                _ => {}
            }
        }

        Ok(item)
    }
}

impl From<&str> for Mode {
    fn from(value: &str) -> Self {
        match value {
            "Neutral" => Self::Neutral,
            "Friendly" => Self::Friendly,
            "Hostile" => Self::Hostile,
            _ => Self::Neutral,
        }
    }
}

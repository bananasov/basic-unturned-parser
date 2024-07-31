use super::gear::ItemGearAsset;
use super::Parser;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemGlassesAsset {
    #[serde(rename = "base")]
    pub item_gear_asset: ItemGearAsset,

    pub vision: Vision,
    pub is_blindfold: bool,
    pub nightvision_fog_intensity: f32,
}

impl Parser<ItemGlassesAsset> for ItemGlassesAsset {
    fn parse<P: AsRef<std::path::Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemGlassesAsset> {
        let item_gear_asset = ItemGearAsset::parse(directory, content)?;
        let mut item = ItemGlassesAsset {
            item_gear_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Vision" => item.vision = value.into(),
                "Blindfold" => item.is_blindfold = true,
                "Nightvision_Fog_Intensity" => {
                    item.nightvision_fog_intensity = value.parse().unwrap_or_default()
                }
                _ => {}
            }
        }

        Ok(item)
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum Vision {
    #[default]
    None,
    Military,
    Civilian,
    Headlamp,
}

impl From<&str> for Vision {
    fn from(value: &str) -> Self {
        match value {
            "Military" => Self::Military,
            "Civilian" => Self::Civilian,
            "Headlamp" => Self::Headlamp,
            _ => Self::None,
        }
    }
}

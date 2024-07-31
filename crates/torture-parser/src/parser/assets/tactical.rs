use std::path::Path;

use super::caliber::ItemCaliberAsset;
use super::Parser;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemTacticalAsset {
    #[serde(rename = "base")]
    item_caliber_asset: ItemCaliberAsset,

    /// If it provides a toggleable laser.
    pub laser: bool,

    /// If it provides a toggleable flashlight, and allows for using [PlayerSpotLightConfig](https://docs.smartlydressedgames.com/en/stable/data/struct/playerspotlightconfig.html#doc-data-playerspotlightconfig) properties.
    pub light: bool,

    // TODO: Add light config
    /// If it provides a toggleable rangefinder.
    pub rangefinder: bool,

    /// If it provides the ability to perform a melee attack.
    ///
    /// This attack does 40 damage, and is not configurable.
    pub melee: bool,
    //pub laser_color: Color
}

impl Parser<ItemTacticalAsset> for ItemTacticalAsset {
    fn parse<P: AsRef<Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemTacticalAsset> {
        let item_caliber_asset = ItemCaliberAsset::parse(directory, content)?;
        let mut item = ItemTacticalAsset {
            item_caliber_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");

            match field {
                "Laser" => item.laser = true,
                "Light" => item.light = true,
                "Rangefinder" => item.rangefinder = true,
                "Melee" => item.melee = true,
                _ => {}
            }
        }

        Ok(item)
    }
}

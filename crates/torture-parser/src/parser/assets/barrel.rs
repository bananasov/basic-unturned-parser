use std::path::Path;

use anyhow::Context;

use super::caliber::ItemCaliberAsset;
use super::Parser;

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ItemBarrelAsset {
    #[serde(rename = "base")]
    pub item_caliber_asset: ItemCaliberAsset,

    /// Gravity acceleration multiplier for bullets in flight.
    pub ballistic_drop: f32,

    /// Whether or not the muzzle flash should be hidden.
    pub braked: bool,

    /// Amount of quality lost after each firing of the ranged weapon.
    ///
    /// When this value is greater than 0, the item always has a visible item quality shown.
    pub durability: u8,

    /// Multiplier on gunshot rolloff distance.
    ///
    /// Defaults to 0.5 if Silenced, otherwise to 1.
    pub gunshot_rolloff_distance_multiplier: f32, // default = is_silenced ? 0.5f : 1f

    /// Alerts should not be generated when firing.
    pub silenced: bool,

    /// Multiplier on gunfire sound volume.
    ///
    /// This is often used alongside with `silenced`, but doing so is not required.
    pub volume: f32,
}

impl Parser<ItemBarrelAsset> for ItemBarrelAsset {
    fn parse<P: AsRef<Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemBarrelAsset> {
        let item_caliber_asset = ItemCaliberAsset::parse(directory, content)?;
        let mut item = ItemBarrelAsset {
            item_caliber_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Ballistic_Drop" => {
                    item.ballistic_drop = value
                        .parse()
                        .context("Failed to parse Ballistic_Drop as a f32")?
                }
                "Braked" => item.braked = true,
                "Durability" => {
                    item.durability = value
                        .parse()
                        .context("Failed to parse Durability as a u8")?
                }
                "Gunshot_Rolloff_Distance_Multiplier" => {
                    item.gunshot_rolloff_distance_multiplier = value
                        .parse()
                        .context("Failed to parse Gunshot_Rolloff_Distance_Multiplier as a f32")?
                }
                "Silenced" => item.silenced = true,
                "Volume" => {
                    item.volume = value.parse().context("Failed to parse Volume as a f32")?
                }
                _ => {}
            }
        }

        Ok(item)
    }
}

use anyhow::Context;

use super::{BaseAsset, Parser};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemClothingAsset {
    #[serde(rename = "base")]
    pub base_asset: BaseAsset,

    pub armor: f32,
    pub armor_explosion: f32,

    pub water_proof: bool,
    pub fire_proof: bool,
    pub radiation_proof: bool,
    pub movement_speed_multiplier: f32,
    pub visible_on_ragdoll: bool,
    pub hair_visible: bool,
    pub beard_visible: bool,
}

impl Parser<ItemClothingAsset> for ItemClothingAsset {
    fn parse<P: AsRef<std::path::Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemClothingAsset> {
        let base_asset = BaseAsset::parse(directory, content)?;
        let mut item = ItemClothingAsset {
            base_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Armor" => item.armor = value.parse().context("Failed to parse Armor as f32")?,
                "Armor_Explosion" => {
                    item.armor_explosion = value
                        .parse()
                        .context("Failed to parse Armor_Explosion as f32")?
                }
                "Proof_Water" => item.water_proof = true,
                "Proof_Fire" => item.fire_proof = true,
                "Proof_Radiation" => item.radiation_proof = true,
                "Movement_Speed_Multiplier" => {
                    item.movement_speed_multiplier = value.parse().unwrap_or(1.0)
                }
                "Visible_On_Ragdoll" => item.visible_on_ragdoll = value.parse().unwrap_or(true),
                "Hair_Visible" => item.hair_visible = value.parse().unwrap_or(true),
                "Beard_Visible" => item.beard_visible = value.parse().unwrap_or(true),
                _ => {}
            }
        }

        Ok(item)
    }
}

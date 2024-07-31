use std::path::Path;

use anyhow::Context;

use super::weapon::ItemWeaponAsset;
use super::Parser;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemConsumableAsset {
    #[serde(rename = "base")]
    item_weapon_asset: ItemWeaponAsset,

    /// Specified if the item can be used on other players, via the “Secondary” action.
    pub is_aid: bool,

    /// Determines the effect the consumable has in relation to the “Bleeding” status effect.
    pub bleeding_modifier: BleedingModifier,

    /// Determines the effect the consumable has in relation to the “Broken Bones” status effect.
    pub bones_modifier: BonesModifier,

    /// Amount of immunity restored.
    pub disinfectant: u8,

    /// Amount of stamina restored.
    pub energy: u8,

    /// Amount of experience added or removed.
    pub experience: i32,

    /// Amount of food restored.
    ///
    /// If the amount of food to restore is larger than the amount of water to restore, then food constrains water.
    pub food: u8,

    /// Amount of health restored.
    pub health: u8,

    /// Amount of oxygen restored or depleted.
    pub oxygen: i8,

    /// Amount of immunity depleted.
    pub virus: u8,

    /// Length of hallucinations, in seconds.
    ///
    /// The length does not stack when consuming multiple hallucinogenics.
    /// Instead, the timer is reset to the longer value.
    pub vision: u32,

    /// Amount of warmth added.
    pub warmth: u32,

    ///  Amount of water restored.
    ///
    /// If the amount of water to restore is less than the amount of food to restore, then water is constrained by food.
    pub water: u8,
}

impl Parser<ItemConsumableAsset> for ItemConsumableAsset {
    fn parse<P: AsRef<Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemConsumableAsset> {
        let item_weapon_asset = ItemWeaponAsset::parse(directory, content)?;
        let mut item = ItemConsumableAsset {
            item_weapon_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Aid" => item.is_aid = true,
                "Bleeding_Modifier" => item.bleeding_modifier = value.into(),
                "Bones_Modifier" => item.bones_modifier = value.into(),
                "Disinfectant" => {
                    item.disinfectant = value
                        .parse()
                        .context("Failed to parse Disinfectant as u8")?
                }
                "Experience" => {
                    item.experience = value.parse().context("Failed to parse Experience as i32")?
                }
                "Energy" => item.energy = value.parse().context("Failed to parse Energy as u8")?,
                "Food" => item.food = value.parse().context("Failed to parse Food as u8")?,
                "Health" => item.health = value.parse().context("Failed to parse Health as u8")?,
                "Virus" => item.virus = value.parse().context("Failed to parse Virus as u8")?,
                "Water" => item.water = value.parse().context("Failed to parse Water as u8")?,
                "Oxygen" => item.oxygen = value.parse().context("Failed to parse Oxygen as i8")?,
                "Vision" => item.vision = value.parse().context("Failed to parse Vision as u32")?,
                "Warmth" => item.warmth = value.parse().context("Failed to parse Warmth as u32")?,
                _ => {}
            }
        }

        Ok(item)
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum BleedingModifier {
    #[default]
    None,
    Cut,
    Heal,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum BonesModifier {
    #[default]
    None,
    Break,
    Heal,
}

impl From<&str> for BleedingModifier {
    fn from(value: &str) -> Self {
        match value {
            "Cut" => Self::Cut,
            "Heal" => Self::Heal,
            _ => Self::None,
        }
    }
}

impl From<&str> for BonesModifier {
    fn from(value: &str) -> Self {
        match value {
            "Break" => Self::Break,
            "Heal" => Self::Heal,
            _ => Self::None,
        }
    }
}

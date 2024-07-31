use anyhow::Context;

use super::BaseAsset;
use super::Parser;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemStructureAsset {
    #[serde(rename = "base")]
    pub base_asset: BaseAsset,

    pub health: u16,
    pub range: f32,

    pub can_be_damaged: bool,
    pub requires_pillars: bool,

    pub is_vulnerable: bool,
    pub is_unrepairable: bool,
    pub is_explosion_proof: bool,
    pub is_unpickupable: bool,
    pub is_unsalvageable: bool,

    pub armor_tier: ArmorTier,
    pub construct: Construct,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum ArmorTier {
    #[default]
    Low,
    High,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum Construct {
    #[default]
    Floor,
    Wall,
    Rampart,
    Roof,
    Pillar,
    Post,
    FloorPoly,
    RoofPoly,
}

impl Parser<ItemStructureAsset> for ItemStructureAsset {
    fn parse<P: AsRef<std::path::Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemStructureAsset> {
        let base_asset = BaseAsset::parse(directory, content)?;
        let mut item = ItemStructureAsset {
            base_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Health" => item.health = value.parse().context("Failed to parse Health as u16")?,
                "Range" => item.range = value.parse().context("Failed to parse Range as f32")?,
                "Can_Be_Damaged" => item.can_be_damaged = value.parse().unwrap_or(true),
                "Requires_Pillars" => item.requires_pillars = value.parse().unwrap_or(true),
                "Vulnerable" => item.is_vulnerable = true,
                "Unrepairable" => item.is_unrepairable = true,
                "Proof_Explosion" => item.is_explosion_proof = true,
                "Unpickupable" => item.is_unpickupable = true,
                "Unsalvageable" => item.is_unsalvageable = true,
                "Armor_Tier" => item.armor_tier = value.into(),
                "Construct" => item.construct = value.into(),
                _ => {}
            }
        }

        Ok(item)
    }
}

impl From<&str> for ArmorTier {
    fn from(value: &str) -> Self {
        match value {
            "High" => Self::High,
            _ => Self::Low,
        }
    }
}

impl From<&str> for Construct {
    fn from(value: &str) -> Self {
        match value {
            "Floor" => Self::Floor,
            "Wall" => Self::Wall,
            "Rampart" => Self::Rampart,
            "Roof" => Self::Roof,
            "Pillar" => Self::Pillar,
            "Post" => Self::Post,
            "Floor_Poly" => Self::FloorPoly,
            "Roof_Poly" => Self::RoofPoly,
            _ => Self::Floor,
        }
    }
}

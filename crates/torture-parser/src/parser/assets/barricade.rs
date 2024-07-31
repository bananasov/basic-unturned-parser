use anyhow::Context;

use super::BaseAsset;
use super::Parser;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemBarricadeAsset {
    #[serde(rename = "base")]
    pub base_asset: BaseAsset,

    pub health: u16,
    pub is_locked: bool,
    pub is_vulnerable: bool,
    pub can_bypass_claims: bool,
    pub range: f32,
    pub radius: f32,
    pub armor_tier: ArmorTier,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum ArmorTier {
    #[default]
    Low,
    High,
}

impl Parser<ItemBarricadeAsset> for ItemBarricadeAsset {
    fn parse<P: AsRef<std::path::Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemBarricadeAsset> {
        let base_asset = BaseAsset::parse(directory, content)?;
        let mut item = ItemBarricadeAsset {
            base_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Health" => item.health = value.parse().context("Failed to parse Health as u16")?,
                "Locked" => item.is_locked = true,
                "Vulnerable" => item.is_vulnerable = true,
                "Bypass_Claim" => item.can_bypass_claims = true,
                "Range" => item.range = value.parse().context("Failed to parse Range as f32")?,
                "Radius" => item.radius = value.parse().context("Failed to parse Radius as f32")?,
                "Armor_Tier" => item.armor_tier = value.into(),
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

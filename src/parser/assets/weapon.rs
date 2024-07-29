use anyhow::Context;

use super::{BaseAsset, Parser};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemWeaponAsset {
    #[serde(flatten)]
    pub base: BaseAsset,

    pub range: f32,
    pub player_damage: PlayerDamage,
    // pub barricade_damage: f32,
    // pub structure_damage: f32,
    // pub vehicle_damage: f32,
    // pub resource_damage: f32,
    // pub object_damage: f32,
    pub durability: f32,
    pub wear: u8,
    pub invulnerable: bool,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PlayerDamage {
    pub food: f32,
    pub water: f32,
    pub virus: f32,
    pub hallucination: f32,
}

impl Parser<ItemWeaponAsset> for ItemWeaponAsset {
    fn parse<P: AsRef<std::path::Path>>(
        directory: P,
        content: String,
    ) -> anyhow::Result<ItemWeaponAsset> {
        let base = BaseAsset::parse(directory.as_ref(), content.clone())?;
        let player_damage = PlayerDamage::parse(directory.as_ref(), content.clone())?;

        let mut item = ItemWeaponAsset {
            base,
            player_damage,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Range" => item.range = value.parse().context("Failed to parse Range as a f32")?,
                "Durability" => {
                    item.durability = value
                        .parse()
                        .context("Failed to parse Durability as a f32")?
                }
                "Wear" => item.wear = value.parse().context("Failed to parse Wear as a u8")?,
                "Invulnerable" => item.invulnerable = true,
                _ => {}
            }
        }

        Ok(item)
    }
}

impl Parser<PlayerDamage> for PlayerDamage {
    fn parse<P: AsRef<std::path::Path>>(
        _directory: P,
        content: String,
    ) -> anyhow::Result<PlayerDamage> {
        let mut damage = PlayerDamage::default();

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Player_Damage_Food" => {
                    damage.food = value
                        .parse()
                        .context("Failed to parse Player_Damage_Food as a f32")?
                }
                "Player_Damage_Water" => {
                    damage.water = value
                        .parse()
                        .context("Failed to parse Player_Damage_Water as a f32")?
                }
                "Player_Damage_Virus" => {
                    damage.virus = value
                        .parse()
                        .context("Failed to parse Player_Damage_Virus as a f32")?
                }
                "Player_Damage_Hallucination" => {
                    damage.hallucination = value
                        .parse()
                        .context("Failed to parse Player_Damage_Hallucination as a f32")?
                }
                _ => {}
            }
        }

        Ok(damage)
    }
}

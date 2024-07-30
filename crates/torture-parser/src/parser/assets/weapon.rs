use std::path::Path;

use anyhow::Context;

use super::{BaseAsset, Parser};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemWeaponAsset {
    #[serde(flatten)]
    pub base: BaseAsset,

    pub range: f32,

    pub player_damage: PlayerDamage,
    pub zombie_damage: ZombieDamage,
    pub animal_damage: AnimalDamage,

    pub barricade_damage: f32,
    pub structure_damage: f32,
    pub vehicle_damage: f32,
    pub resource_damage: f32,
    pub object_damage: f32,

    pub durability: f32,
    pub wear: u8,
    pub invulnerable: bool,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PlayerDamage {
    pub damage: f32,
    pub leg_multiplier: f32,
    pub arm_multiplier: f32,
    pub spine_multiplier: f32,
    pub skull_multiplier: f32,

    pub food: f32,
    pub water: f32,
    pub virus: f32,
    pub hallucination: f32,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ZombieDamage {
    pub damage: f32,
    pub leg_multiplier: f32,
    pub arm_multiplier: f32,
    pub spine_multiplier: f32,
    pub skull_multiplier: f32,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AnimalDamage {
    pub damage: f32,
    pub leg_multiplier: f32,
    pub spine_multiplier: f32,
    pub skull_multiplier: f32,
}

impl Parser<ItemWeaponAsset> for ItemWeaponAsset {
    fn parse<P: AsRef<Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemWeaponAsset> {
        let base = BaseAsset::parse(directory, content)?;
        let player_damage = PlayerDamage::parse(directory, content)?;

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
                "Barricade_Damage" => item.barricade_damage = value.parse().context("Failed to parse Barricade_Damage as f32")?,
"Structure_Damage" => item.structure_damage = value.parse().context("Failed to parse Structure_Damage as f32")?,
"Vehicle_Damage" => item.vehicle_damage = value.parse().context("Failed to parse Vehicle_Damage as f32")?,
"Resource_Damage" => item.resource_damage = value.parse().context("Failed to parse Resource_Damage as f32")?,
"Object_Damage" => item.object_damage = value.parse().context("Failed to parse Object_Damage as f32")?,
                _ => {}
            }
        }

        Ok(item)
    }
}

impl Parser<PlayerDamage> for PlayerDamage {
    fn parse<P: AsRef<Path> + ?Sized>(
        _directory: &P,
        content: &str,
    ) -> anyhow::Result<PlayerDamage> {
        let mut damage = PlayerDamage::default();

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Player_Damage" => {
                    damage.damage = value
                        .parse()
                        .context("Failed to parse Player_Damage as a f32")?
                }
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
                "Player_Leg_Multiplier" => {
                    damage.leg_multiplier = value
                        .parse()
                        .context("Failed to parse Player_Leg_Multiplier as f32")?
                }
                "Player_Arm_Multiplier" => {
                    damage.arm_multiplier = value
                        .parse()
                        .context("Failed to parse Player_Arm_Multiplier as f32")?
                }
                "Player_Spine_Multiplier" => {
                    damage.spine_multiplier = value
                        .parse()
                        .context("Failed to parse Player_Spine_Multiplier as f32")?
                }
                "Player_Skull_Multiplier" => {
                    damage.skull_multiplier = value
                        .parse()
                        .context("Failed to parse Player_Skull_Multiplier as f32")?
                }
                _ => {}
            }
        }

        Ok(damage)
    }
}

impl Parser<ZombieDamage> for ZombieDamage {
    fn parse<P: AsRef<Path> + ?Sized>(
        _directory: &P,
        content: &str,
    ) -> anyhow::Result<ZombieDamage> {
        let mut damage = ZombieDamage::default();

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Zombie_Damage" => {
                    damage.damage = value
                        .parse()
                        .context("Failed to parse Zombie_Damage as a f32")?
                }
                "Zombie_Leg_Multiplier" => {
                    damage.leg_multiplier = value
                        .parse()
                        .context("Failed to parse Zombie_Leg_Multiplier as f32")?
                }
                "Zombie_Arm_Multiplier" => {
                    damage.arm_multiplier = value
                        .parse()
                        .context("Failed to parse Zombie_Arm_Multiplier as f32")?
                }
                "Zombie_Spine_Multiplier" => {
                    damage.spine_multiplier = value
                        .parse()
                        .context("Failed to parse Zombie_Spine_Multiplier as f32")?
                }
                "Zombie_Skull_Multiplier" => {
                    damage.skull_multiplier = value
                        .parse()
                        .context("Failed to parse Zombie_Skull_Multiplier as f32")?
                }
                _ => {}
            }
        }

        Ok(damage)
    }
}

impl Parser<AnimalDamage> for AnimalDamage {
    fn parse<P: AsRef<Path> + ?Sized>(
        _directory: &P,
        content: &str,
    ) -> anyhow::Result<AnimalDamage> {
        let mut damage = AnimalDamage::default();

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Animal_Damage" => {
                    damage.damage = value
                        .parse()
                        .context("Failed to parse Animal_Damage as f32")?
                }
                "Animal_Leg_Multiplier" => {
                    damage.leg_multiplier = value
                        .parse()
                        .context("Failed to parse Animal_Leg_Multiplier as f32")?
                }
                "Animal_Spine_Multiplier" => {
                    damage.spine_multiplier = value
                        .parse()
                        .context("Failed to parse Animal_Spine_Multiplier as f32")?
                }
                "Animal_Skull_Multiplier" => {
                    damage.skull_multiplier = value
                        .parse()
                        .context("Failed to parse Animal_Skull_Multiplier as f32")?
                }
                _ => {}
            }
        }

        Ok(damage)
    }
}

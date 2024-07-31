use anyhow::Context;

use super::caliber::ItemCaliberAsset;
use crate::parser::Parser;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemMagazineAsset {
    #[serde(rename = "base")]
    item_caliber_asset: ItemCaliberAsset,

    /// Number of bullet rays shot.
    pub pellets: u8,

    /// The amount of quality that should be lost after the projectile hits something.
    ///
    /// When this value is greater than 0, the item will have a visible quality value.
    ///
    /// This property is typically used with ranged weapons utilizing the `Action String` key-value pair, such as a crossbow.
    pub stuck: u8,

    /// Multiplier on the damage dealt by the explosive projectiles fired from physics projectile weapons.
    pub projectile_damage_multiplier: f32,

    /// Multiplier on the blast radius of the explosive projectiles fired from physics projectile weapons.
    pub projectile_blast_radius_multiplier: f32,

    /// Multiplier on the launch force applied to the explosive projectiles fired from physics projectile weapons.
    pub projectile_launch_force_multiplier: f32,

    /// In meters, the radius of the area-of-effect explosion caused by a projectile when a magazine attachment is using the `Explosive` flag.
    pub range: f32,

    /// Damage dealt to players caught within the area-of-effect explosion of a magazine attachment using the `Explosive` flag.
    pub player_damage: f32,

    /// Damage dealt to zombies caught within the area-of-effect explosion of a magazine attachment using the `Explosive` flag.
    pub zombie_damage: f32,

    /// Damage dealt to animals caught within the area-of-effect explosion of a magazine attachment using the `Explosive` flag.
    pub animal_damage: f32,

    /// Damage dealt to barricades caught within the area-of-effect explosion of a magazine attachment using the `Explosive` flag.
    pub barricade_damage: f32,

    /// Damage dealt to structures caught within the area-of-effect explosion of a magazine attachment using the `Explosive` flag.
    pub structure_damage: f32,

    /// Damage dealt to vehicles caught within the area-of-effect explosion of a magazine attachment using the `Explosive` flag.
    pub vehicle_damage: f32,

    /// Damage dealt to resources caught within the area-of-effect explosion of a magazine attachment using the `Explosive` flag.
    pub resource_damage: f32,

    /// Damage dealt to players caught within the area-of-effect explosion of a magazine attachment using the Explosive flag.
    ///
    /// Defaults to the value of the resource_damage property.
    pub object_damage: f32,

    /// Multiplier on reload speed.
    pub speed: f32,

    /// When this flag is included,
    /// the projectile fired from a ballistics projectile weapon will cause an area-of-effect explosion.
    pub is_explosive: bool,
}

impl Parser<ItemMagazineAsset> for ItemMagazineAsset {
    fn parse<P: AsRef<std::path::Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemMagazineAsset> {
        let item_caliber_asset = ItemCaliberAsset::parse(directory, content)?;
        let mut item = ItemMagazineAsset {
            item_caliber_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Pellets" => {
                    item.pellets = value.parse().context("Failed to parse Pellets as a u8")?
                }
                "Stuck" => item.stuck = value.parse().context("Failed to parse Stuck as a u8")?,
                "Projectile_Damage_Multiplier" => {
                    item.projectile_damage_multiplier = value
                        .parse()
                        .context("Failed to parse Projectile_Damage_Multiplier as a f32")?
                }
                "Projectile_Blast_Radius_Multiplier" => {
                    item.projectile_blast_radius_multiplier = value
                        .parse()
                        .context("Failed to parse Projectile_Blast_Radius_Multiplier as a f32")?
                }
                "Projectile_Launch_Force_Multiplier" => {
                    item.projectile_launch_force_multiplier = value
                        .parse()
                        .context("Failed to parse Projectile_Launch_Force_Multiplier as a f32")?
                }
                "Range" => item.range = value.parse().context("Failed to parse Range as a f32")?,
                "Player_Damage" => {
                    item.player_damage = value
                        .parse()
                        .context("Failed to parse Player_Damage as a f32")?
                }
                "Zombie_Damage" => {
                    item.zombie_damage = value
                        .parse()
                        .context("Failed to parse Zombie_Damage as a f32")?
                }
                "Animal_Damage" => {
                    item.animal_damage = value
                        .parse()
                        .context("Failed to parse Animal_Damage as a f32")?
                }
                "Barricade_Damage" => {
                    item.barricade_damage = value
                        .parse()
                        .context("Failed to parse Barricade_Damage as a f32")?
                }
                "Structure_Damage" => {
                    item.structure_damage = value
                        .parse()
                        .context("Failed to parse Structure_Damage as a f32")?
                }
                "Vehicle_Damage" => {
                    item.vehicle_damage = value
                        .parse()
                        .context("Failed to parse Vehicle_Damage as a f32")?
                }
                "Resource_Damage" => {
                    item.resource_damage = value
                        .parse()
                        .context("Failed to parse Resource_Damage as a f32")?
                }
                "Object_Damage" => {
                    item.object_damage = value
                        .parse()
                        .context("Failed to parse Object_Damage as a f32")?
                }
                "Speed" => item.speed = value.parse().context("Failed to parse speed as a f32")?,
                "Explosive" => item.is_explosive = true,
                _ => {}
            }
        }

        Ok(item)
    }
}

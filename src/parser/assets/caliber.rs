use std::path::Path;

use anyhow::Context;

use super::{BaseAsset, Parser};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemCaliberAsset {
    #[serde(flatten)]
    pub base: BaseAsset,

    /// Multiplier on character movement speed while aiming down sights.
    pub aiming_movement_speed_multiplier: f32,

    /// Multiplier on recoil magnitude while aiming down sights.
    pub aiming_recoil_multiplier: f32,

    /// Multiplier on the value of `aim_in_duration` property available to `ItemGunAsset`.
    pub aim_duration_multiplier: f32,

    /// Multiplier on damage.
    ///
    /// Defaults to the value of the Damage property, or 1 if both properties are unset.
    pub ballistic_damage_multiplier: f32,

    pub calibers: Vec<u16>,

    /// *Deprecated since version 3.27.0.0: Use `ballistic_damage_multiplier` instead.*
    ///
    /// Maintained for backwards compatibility.
    /// If both this property and Ballistic_Damage_Multiplier have been set, the latter’s value is used.
    pub damage: f32,

    /// The value of the attached ranged weapon’s Firerate property is reduced by the value of this property.
    ///
    /// A larger decrease will allow for the ranged weapon to fire more often.
    pub firerate: u8,

    /// If true, gun can damage entities with Invulnerable tag.
    pub invulnerable: bool,

    /// When this flag is included, the attachment should be affected by Steam Economy skins that include support for skinning attachments.
    pub paintable: bool,

    /// Multiplier on horizontal recoil.
    pub recoil_x: f32,

    /// Multiplier on vertical recoil.
    pub recoil_y: f32,

    /// Multiplier on shake.
    pub shake: f32,

    /// Multiplier on bullet spread.
    pub spread: f32,

    /// Multiplier on scope sway.
    pub sway: f32,
}

impl Parser<ItemCaliberAsset> for ItemCaliberAsset {
    fn parse<P: AsRef<Path>>(directory: P, content: String) -> anyhow::Result<ItemCaliberAsset> {
        let base = BaseAsset::parse(directory, content.clone())?;
        let mut item = ItemCaliberAsset {
            base,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Aiming_Movement_Speed_Multiplier" => {
                    item.aiming_movement_speed_multiplier = value
                        .parse()
                        .context("Failed to parse Aiming_Movement_Speed_Multiplier as a f32")?
                }
                "Aiming_Recoil_Multiplier" => {
                    item.aiming_recoil_multiplier = value
                        .parse()
                        .context("Failed to parse Aiming_Recoil_Multiplier as a f32")?
                }
                "Aim_Duration_Multiplier" => {
                    item.aiming_recoil_multiplier = value
                        .parse()
                        .context("Failed to parse Aim_Duration_Multiplier as a f32")?
                }
                "Ballistic_Damage_Multiplier" => {
                    item.aiming_recoil_multiplier = value
                        .parse()
                        .context("Failed to parse Ballistic_Damage_Multiplier as a f32")?
                }
                "Damage" => {
                    item.damage = value.parse().context("Failed to parse Damage as a f32")?
                }
                "Firerate" => {
                    item.firerate = value.parse().context("Failed to parse Damage as a u8")?;
                }
                "Invulnerable" => item.invulnerable = true,
                "Paintable" => item.paintable = true,
                "Recoil_X" => {
                    item.recoil_x = value.parse().context("Failed to parse Recoil_X as a f32")?
                }
                "Recoil_Y" => {
                    item.recoil_y = value.parse().context("Failed to parse Recoil_Y as a f32")?
                }
                "Shake" => item.shake = value.parse().context("Failed to parse Shake as a f32")?,
                "Spread" => {
                    item.spread = value.parse().context("Failed to parse Spread as a f32")?
                }
                "Sway" => item.sway = value.parse().context("Failed to parse Sway as f32")?,
                _ => {}
            }
        }

        Ok(item)
    }
}

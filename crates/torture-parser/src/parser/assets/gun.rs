use std::path::Path;

use anyhow::Context;

use super::weapon::ItemWeaponAsset;
use super::Parser;

// TODO: DOCUMENT THIS :sob:

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemGunAsset {
    #[serde(flatten)]
    pub base: ItemWeaponAsset,

    pub ammo: Ammo,
    pub sight_id: u16,
    pub tactical_id: u16,
    pub grip_id: u16,
    pub barrel_id: u16,
    pub magazine_id: u16,
    // pub magazine_replacements: Vec<MagazineReplacement>,
    pub hooks: Vec<Hook>,
    // pub magazine_calibers: Vec<u16>,
    // pub attachment_calibers: Vec<u16>,
    pub firerate: u8,
    pub action: Action,
    pub firemodes: Vec<Firemode>,
    pub is_turret: bool,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Ammo {
    pub min: u8,
    pub max: u8,
}

// #[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
// pub struct MagazineReplacement {
//     pub map: String,
//     pub id: u16,
// }

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum Hook {
    #[default]
    None, // will literally never be used.
    Sight,
    Tactical,
    Grip,
    Barrel,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum Action {
    #[default]
    None, // will literally never be used.
    Trigger,
    Bolt,
    Pump,
    Rail,
    String,
    Break,
    Rocket,
    Minigun,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum Firemode {
    #[default]
    None, // will literally never be used.
    Safety,
    Semi,
    Auto,
    Burst(i32),
}

impl Parser<ItemGunAsset> for ItemGunAsset {
    fn parse<P: AsRef<Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemGunAsset> {
        let base = ItemWeaponAsset::parse(directory, content)?;
        let ammo = Ammo::parse(directory, content)?;
        let hooks: Vec<Hook> = Vec::parse(directory, content)?;
        let firemodes: Vec<Firemode> = Vec::parse(directory, content)?;

        let mut item = ItemGunAsset {
            base,
            ammo,
            hooks,
            firemodes,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Barrel" => item.barrel_id = value.parse().unwrap_or(0),
                "Grip" => item.grip_id = value.parse().unwrap_or(0),
                "Sight" => item.sight_id = value.parse().unwrap_or(0),
                "Tactical" => item.tactical_id = value.parse().unwrap_or(0),
                "Magazine" => item.magazine_id = value.parse().unwrap_or(0),
                "Firerate" => item.firerate = value.parse().unwrap_or(0),
                "Action" => item.action = value.into(),
                "Turret" => item.is_turret = true,
                _ => {}
            }
        }

        Ok(item)
    }
}

impl Parser<Ammo> for Ammo {
    fn parse<P: AsRef<Path> + ?Sized>(_directory: &P, content: &str) -> anyhow::Result<Ammo> {
        let mut ammo = Ammo::default();

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Ammo_Min" => ammo.min = value.parse().context("Failed to parse Ammo_Min as u8")?,
                "Ammo_Max" => ammo.max = value.parse().context("Failed to parse Ammo_Max as u8")?,
                _ => {}
            }
        }

        Ok(ammo)
    }
}

impl Parser<Vec<Hook>> for Vec<Hook> {
    fn parse<P: AsRef<Path> + ?Sized>(_directory: &P, content: &str) -> anyhow::Result<Vec<Hook>> {
        let mut hooks = Vec::new();

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");

            match field {
                "Hook_Sight" => hooks.push(Hook::Sight),
                "Hook_Barrel" => hooks.push(Hook::Barrel),
                "Hook_Grip" => hooks.push(Hook::Grip),
                "Hook_Tactical" => hooks.push(Hook::Tactical),
                _ => {}
            }
        }

        Ok(hooks)
    }
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        match value {
            "Trigger" => Self::Trigger,
            "Bolt" => Self::Bolt,
            "Pump" => Self::Pump,
            "Rail" => Self::Rail,
            "String" => Self::String,
            "Break" => Self::Break,
            "Rocket" => Self::Rocket,
            "Minigun" => Self::Minigun,
            _ => Self::None,
        }
    }
}

impl Parser<Vec<Firemode>> for Vec<Firemode> {
    fn parse<P: AsRef<Path> + ?Sized>(
        _directory: &P,
        content: &str,
    ) -> anyhow::Result<Vec<Firemode>> {
        let mut firemodes = Vec::new();

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Safety" => firemodes.push(Firemode::Safety),
                "Semi" => firemodes.push(Firemode::Semi),
                "Auto" => firemodes.push(Firemode::Auto),
                "Burst" => {
                    let amount: i32 = value.parse().unwrap_or(0);

                    firemodes.push(Firemode::Burst(amount))
                }
                _ => {}
            }
        }

        Ok(firemodes)
    }
}

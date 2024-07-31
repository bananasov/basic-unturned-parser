pub mod bag;
pub mod barrel;
pub mod barricade;
pub mod caliber;
pub mod clothing;
pub mod consumable;
pub mod gear;
pub mod glasses;
pub mod grip;
pub mod gun;
pub mod magazine;
pub mod mask;
pub mod optic;
pub mod parachute;
pub mod sentry;
pub mod shirt;
pub mod sight;
pub mod storage;
pub mod structure;
pub mod tactical;
pub mod weapon;

use std::path::Path;

use anyhow::Context;

use crate::parser::language::Language;

use super::Parser;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct BaseAsset {
    pub name: String,
    pub description: String,
    pub guid: String,
    pub r#type: Type,
    pub rarity: Rarity,
    pub id: u16,
}

#[derive(Debug, Copy, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Type {
    #[default]
    Unknown,
    Hat,
    Pants,
    Shirt,
    Mask,
    Backpack,
    Vest,
    Glasses,
    Gun,
    Sight,
    Tactical,
    Grip,
    Barrel,
    Magazine,
    Food,
    Water,
    Medical,
    Melee,
    Fuel,
    Tool,
    Barricade,
    Storage,
    Beacon,
    Farm,
    Trap,
    Structure,
    Supply,
    Throwable,
    Grower,
    Optic,
    Refill,
    Fisher,
    Cloud,
    Map,
    Key,
    Box,
    ArrestStart,
    ArrestEnd,
    Tank,
    Generator,
    Detonator,
    Charge,
    Library,
    Filter,
    Sentry,
    VehicleRepairTool,
    Tire,
    Compass,
    OilPump,
    Vehicle,
    Resource,
    Spawn,
    Npc,
    Decal,
    Effect,
    Animal,
}

#[derive(Debug, Default, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub enum Rarity {
    #[default]
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythical,
}

impl Parser<BaseAsset> for BaseAsset {
    fn parse<P: AsRef<Path> + ?Sized>(directory: &P, content: &str) -> anyhow::Result<BaseAsset> {
        let lines = content.lines();
        let mut base_asset = BaseAsset::default();

        for line in lines {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "GUID" => base_asset.guid = value.into(),
                "Type" => base_asset.r#type = value.into(),
                "Rarity" => base_asset.rarity = value.into(),
                "ID" => base_asset.id = value.parse().context("Failed to parse ID to a u16")?,
                _ => {}
            }
        }

        let language_file = directory.as_ref().join("English.dat");

        let language = Language::parse_language(&language_file)?;
        base_asset.name = language.name;
        base_asset.description = language.description;

        Ok(base_asset)
    }
}

impl From<&str> for Rarity {
    fn from(value: &str) -> Self {
        match value {
            "Common" => Self::Common,
            "Uncommon" => Self::Uncommon,
            "Rare" => Self::Rare,
            "Epic" => Self::Epic,
            "Legendary" => Self::Legendary,
            "Mythical" => Self::Mythical,

            "common" => Self::Common,
            "uncommon" => Self::Uncommon,
            "rare" => Self::Rare,
            "epic" => Self::Epic,
            "legendary" => Self::Legendary,
            "mythical" => Self::Mythical,
            _ => panic!("Invalid rarity, got '{}'", value),
        }
    }
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        match value {
            "Hat" => Self::Hat,
            "Pants" => Self::Pants,
            "Shirt" => Self::Shirt,
            "Mask" => Self::Mask,
            "Backpack" => Self::Backpack,
            "Vest" => Self::Vest,
            "Glasses" => Self::Glasses,
            "Gun" => Self::Gun,
            "Sight" => Self::Sight,
            "Tactical" => Self::Tactical,
            "Grip" => Self::Grip,
            "Barrel" => Self::Barrel,
            "Magazine" => Self::Magazine,
            "Food" => Self::Food,
            "Water" => Self::Water,
            "Medical" => Self::Medical,
            "Melee" => Self::Melee,
            "Fuel" => Self::Fuel,
            "Tool" => Self::Tool,
            "Barricade" => Self::Barricade,
            "Storage" => Self::Storage,
            "Beacon" => Self::Beacon,
            "Farm" => Self::Farm,
            "Trap" => Self::Trap,
            "Structure" => Self::Structure,
            "Supply" => Self::Supply,
            "Throwable" => Self::Throwable,
            "Grower" => Self::Grower,
            "Optic" => Self::Optic,
            "Refill" => Self::Refill,
            "Fisher" => Self::Fisher,
            "Cloud" => Self::Cloud,
            "Map" => Self::Map,
            "Key" => Self::Key,
            "Box" => Self::Box,
            "Arrest_Start" => Self::ArrestStart,
            "Arrest_End" => Self::ArrestEnd,
            "Tank" => Self::Tank,
            "Generator" => Self::Generator,
            "Detonator" => Self::Detonator,
            "Charge" => Self::Charge,
            "Library" => Self::Library,
            "Filter" => Self::Filter,
            "Sentry" => Self::Sentry,
            "Vehicle_Repair_Tool" => Self::VehicleRepairTool,
            "Tire" => Self::Tire,
            "Compass" => Self::Compass,
            "Oil_Pump" => Self::OilPump,
            "Resource" => Self::Resource,
            "Vehicle" => Self::Vehicle,
            "Spawn" => Self::Spawn,
            "Npc" => Self::Npc,
            "Decal" => Self::Decal,
            "Effect" => Self::Effect,
            "Animal" => Self::Animal,
            _ => Self::Unknown,
        }
    }
}

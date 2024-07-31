use anyhow::Context;

use crate::parser::Parser;

use super::caliber::ItemCaliberAsset;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ItemSightAsset {
    #[serde(rename = "base")]
    item_caliber_asset: ItemCaliberAsset,

    /// Multiplicative amount of zoom.
    ///
    /// This value must be equal to or greater than 1.
    pub zoom: f32,

    /// This sight should be holographic.
    pub holographic: bool,

    /// Set a unique lighting vision effect to use.
    ///
    /// The value of this property may effect the default values of other properties.
    ///
    /// The Headlamp enumerator is not supported by this property.
    pub vision: Vision,
    // List of distance markers, they are to add visible (and accurate) distance markers to the scope that account for the weapon’s bullet drop.
    // pub distance_markers: Vec<DistanceMarker>,
    // TODO: Nightvision stuffs
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DistanceMarker {
    /// Meters between the player and a hypothethical target.
    pub distance: f32,

    /// Distance between center line and start of horizontal line marker.
    ///
    /// Display-related properties like `line_offset` are a percentage (represented as a decimal value from 0 to 1).
    /// For example, `0.25 would` be 25%.
    pub line_offset: f32,

    /// Length of horizontal line marker.
    ///
    /// Display-related properties like `line_width` are a percentage (represented as a decimal value from 0 to 1).
    /// For example, `0.25` would be 25%.
    pub line_width: f32,

    /// Direction the horizontal line and text expand in.
    pub side: Side,

    /// If true, a label with Distance text is shown next to the horizontal line marker.
    pub has_label: f32,
    // TODO: Add Color32
    //pub color: Color32
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Side {
    /// Marking extends to the left from the center.
    Left,

    /// Marking extends to the right from the center.
    Right,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum Vision {
    /// There is no vision effect, and normal lighting is used.
    #[default]
    None,
    /// “Military” nightvision lighting is used. When supported by the asset, nightvision color is `#507814`, and nightvision fog intensity is `0.25`.
    Military,
    /// “Civilian” nightvision lighting is used. When supported by the asset, nightvision color is `#666666`, and nightvision fog intensity is `0.5`.
    Civilian,
    /// “Headlamp” lighting is used. When supported by the asset, this will enable a toggleable light source and allow for using PlayerSpotLightConfig properties.
    Headlamp,
}

impl From<&str> for Side {
    fn from(value: &str) -> Self {
        match value {
            "Left" => Self::Left,
            "Right" => Self::Right,
            _ => Self::Left,
        }
    }
}

impl From<&str> for Vision {
    fn from(value: &str) -> Self {
        match value {
            "Military" => Self::Military,
            "Civilian" => Self::Civilian,
            "Headlamp" => Self::Headlamp,
            _ => Self::None,
        }
    }
}

impl Parser<ItemSightAsset> for ItemSightAsset {
    fn parse<P: AsRef<std::path::Path> + ?Sized>(
        directory: &P,
        content: &str,
    ) -> anyhow::Result<ItemSightAsset> {
        let item_caliber_asset = ItemCaliberAsset::parse(directory, content)?;
        let mut item = ItemSightAsset {
            item_caliber_asset,
            ..Default::default()
        };

        for line in content.lines() {
            let mut split = line.split_whitespace();

            let field = split.next().unwrap_or("");
            let value = split.next().unwrap_or("");

            match field {
                "Vision" => item.vision = value.into(),
                "Zoom" => item.zoom = value.parse().context("Failed to parse Zoom as f32")?,
                "Holographic" => item.holographic = true,
                _ => {}
            }
        }

        Ok(item)
    }
}

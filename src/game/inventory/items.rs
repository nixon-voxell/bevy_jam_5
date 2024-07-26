use bevy::prelude::*;

#[derive(Component, Default, Debug, Clone)]
pub struct ItemIcon {
    pub icon: Handle<Image>,
    pub color: Color,
}

/// Name of the item
#[derive(Component, Default, Debug, Clone)]
pub struct ItemName(pub String);

/// Long description of the item
#[derive(Component, Default, Debug, Clone)]
pub struct Description(pub String);

/// Item can be used
#[derive(Component, Default, Debug, Clone)]
pub struct UsuableItem;

/// Item can be worn
/// A maximum of one wearable item of each variant can be equipped by a character
#[derive(Component, Debug, Clone)]
pub enum WearableItem {
    Hands,
    Feet,
    Body,
    Head,
    Shield,
}

/// Number of remaining times the item can be used
/// If not present, item is infinite use.
#[derive(Component, Debug, Clone)]
pub struct ConsumbleItem {
    uses: u32,
}

impl Default for ConsumbleItem {
    fn default() -> Self {
        Self { uses: 1 }
    }
}

/// Price of the item in gold coins
#[derive(Component, Default, Debug, Clone)]
pub struct ItemPrice(pub i32);

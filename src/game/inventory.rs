use bevy::prelude::*;

/// Maximum number of the items a character can
#[derive(Component, Debug)]
pub struct MaxInventorySize(pub usize);

impl Default for MaxInventorySize {
    fn default() -> Self {
        Self(4)
    }
}

/// List of an entity's equipped items
#[derive(Component, Debug)]
pub struct Inventory {
    item_slots: Vec<Option<Entity>>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            item_slots: vec![None; 3],
        }
    }
}

impl Inventory {
    pub fn slot_count(&self) -> usize {
        self.item_slots.len()
    }

    pub fn add_slot(&mut self) {
        self.item_slots.push(None);
    }

    pub fn take(&mut self, slot: usize) -> Option<Entity> {
        self.item_slots[slot].take()
    }

    pub fn get(&self, slot: usize) -> Option<Entity> {
        self.item_slots[slot]
    }

    pub fn set(&mut self, slot: usize, entity: Entity) -> Option<Entity> {
        let previous = self.item_slots[slot];
        self.item_slots[slot] = Some(entity);
        previous
    }
}

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

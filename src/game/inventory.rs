pub mod items;

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

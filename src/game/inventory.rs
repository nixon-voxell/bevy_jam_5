use bevy::prelude::*;

use crate::path_finding::tiles::TileDir;

pub const ITEM_TEMPLATES: &[Item] = &[
    Item {
        name: "axe",
        description: "Axe, mid range weapon. (Land only)",
        health_effect: -2,
        item_count: 2,
        directions: &TileDir::EDGES,
        range: 2,
        cost: 10,
        air: false,
    },
    Item {
        name: "dagger",
        description: "Dagger, close range high damage weapon. (Land only)",
        health_effect: -4,
        item_count: 1,
        directions: &TileDir::EDGES,
        range: 1,
        cost: 40,
        air: false,
    },
    Item {
        name: "sword",
        description: "Sword, mid range weapon. (Land only)",
        health_effect: -1,
        item_count: 1,
        directions: &TileDir::ALL,
        range: 2,
        cost: 30,
        air: false,
    },
    Item {
        name: "whip",
        description: "Whip, long range low damage weapon. (Land & Air)",
        health_effect: -1,
        item_count: 1,
        directions: &TileDir::ALL,
        range: 3,
        cost: 20,
        air: true,
    },
    Item {
        name: "bow",
        description: "Bow, long range weapon. (Land & Air)",
        health_effect: -2,
        item_count: 2,
        directions: &TileDir::ALL,
        range: 3,
        cost: 30,
        air: true,
    },
    Item {
        name: "health_potion",
        description: "Healing potion, heals 1 health.",
        health_effect: 1,
        item_count: 1,
        directions: &TileDir::ALL,
        range: 2,
        cost: 20,
        air: false,
    },
];

/// Maximum number of the items a character can
#[derive(Component, Debug)]
pub struct MaxInventorySize(pub u32);

impl Default for MaxInventorySize {
    fn default() -> Self {
        Self(3)
    }
}

/// List of an entity's equipped items
#[derive(Component, Debug)]
pub struct Inventory {
    pub selected_item: Option<usize>,
    item_slots: Vec<Option<Item>>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            selected_item: None,
            item_slots: vec![Some(ITEM_TEMPLATES[2]), None, None],
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

    pub fn take(&mut self, slot: usize) -> Option<Item> {
        self.item_slots[slot].take()
    }

    pub fn get(&self, slot: usize) -> Option<Item> {
        self.item_slots.get(slot).copied().flatten()
    }

    pub fn set(&mut self, slot: usize, item: Item) -> Option<Item> {
        let previous = self.item_slots[slot];
        self.item_slots[slot] = Some(item);
        previous
    }

    pub fn get_empty_slot(&self) -> Option<usize> {
        self.item_slots.iter().position(|i| i.is_none())
    }

    pub fn clear(&mut self) {
        for item in self.item_slots.iter_mut() {
            *item = None;
        }
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

/// Marking that this entity is just a reference item.
#[derive(Debug, Clone, Copy)]
pub struct Item {
    /// Name of the item
    pub name: &'static str,
    /// Long description of the item
    pub description: &'static str,
    /// Positive for healing effect, negative for attack effect.
    pub health_effect: i32,
    /// Number of items you get per purchase.
    pub item_count: u32,
    pub directions: &'static [TileDir],
    pub range: u32,
    /// Cost of the item in gold coins
    pub cost: u32,
    pub air: bool,
}

use bevy::prelude::*;

use super::map::{KING_MOVES, ROOK_MOVES};

pub const ITEM_TEMPLATES: &[ItemTemplate] = &[
    ItemTemplate {
        name: "axe",
        description: "Axe, mid range weapon. (Land only)",
        health_effect: -2,
        item_count: 2,
        directions: &ROOK_MOVES,
        range: 2,
        cost: 10,
        air: false,
    },
    ItemTemplate {
        name: "dagger",
        description: "Dagger, close range high damage weapon. (Land only)",
        health_effect: -2,
        item_count: 2,
        directions: &ROOK_MOVES,
        range: 1,
        cost: 10,
        air: false,
    },
    ItemTemplate {
        name: "sword",
        description: "Sword, mid range high damage weapon. (Land only)",
        health_effect: -4,
        item_count: 2,
        directions: &KING_MOVES,
        range: 3,
        cost: 30,
        air: false,
    },
    ItemTemplate {
        name: "whip",
        description: "Whip, long range low damage weapon. (Land & Air)",
        health_effect: -1,
        item_count: 2,
        directions: &KING_MOVES,
        range: 4,
        cost: 10,
        air: true,
    },
    ItemTemplate {
        name: "bow",
        description: "Bow, long range weapon. (Land & Air)",
        health_effect: -2,
        item_count: 4,
        directions: &KING_MOVES,
        range: 4,
        cost: 10,
        air: true,
    },
    ItemTemplate {
        name: "health_potion",
        description: "Healing potion, heals 1 health.",
        health_effect: 1,
        item_count: 1,
        directions: &KING_MOVES,
        range: 4,
        cost: 10,
        air: false,
    },
];

/// Maximum number of the items a character can
#[derive(Component, Debug)]
pub struct MaxInventorySize(pub u32);

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
pub struct ItemTemplate {
    /// Name of the item
    pub name: &'static str,
    /// Long description of the item
    pub description: &'static str,
    /// Positive for healing effect, negative for attack effect.
    pub health_effect: i32,
    /// Number of items you get per purchase.
    pub item_count: u32,
    pub directions: &'static [IVec2],
    pub range: u32,
    /// Cost of the item in gold coins
    pub cost: u32,
    pub air: bool,
}

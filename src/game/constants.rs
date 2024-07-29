use bevy::ui::Val;
use bevy::ui::ZIndex;

//pub const INITIAL_GOLD: u32 = 50;
pub const INITIAL_GOLD: u32 = 1000;
//pub const INITIAL_POPULATION: u32 = 25;
pub const INITIAL_POPULATION: u32 = 250;

pub const HOUSE_POPULATION_CAPACITY: u32 = 10;

pub const HOUSE_COST: u32 = 25;
pub const TAVERN_COST: u32 = 75;

pub const ICON_SIZE: Val = Val::Px(16.);

pub const BUILDING_COSTS_PANEL_Z_INDEX: ZIndex = ZIndex::Global(10);

pub const TEXT_SIZE: f32 = 16.;
pub const BIG_TEXT_SIZE: f32 = 30.;

pub const UPGRADE_COST: u32 = 60;

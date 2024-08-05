use bevy::color::palettes::css::GREEN_YELLOW;
use bevy::color::palettes::tailwind::YELLOW_300;
use bevy::ui::Val;
use bevy::ui::ZIndex;

//pub const INITIAL_GOLD: u32 = 50;
pub const INITIAL_GOLD: u32 = 1000;
//pub const INITIAL_POPULATION: u32 = 25;
pub const INITIAL_POPULATION: u32 = 125;

pub const HOUSE_POPULATION_CAPACITY: u32 = 10;

pub const HOUSE_COST: u32 = 25;
pub const TAVERN_COST: u32 = 25;

pub const ICON_SIZE: Val = Val::Px(16.);

pub const BUILDING_COSTS_PANEL_Z_INDEX: ZIndex = ZIndex::Global(10);

pub const TEXT_SIZE: f32 = 16.;
pub const BIG_TEXT_SIZE: f32 = 30.;

pub const UPGRADE_COST: u32 = 20;

pub const TAVERN_FONT_SIZE: f32 = 20.;

pub const RECRUIT_COST: u32 = 40;

pub const SLOT_COST: u32 = 0;

pub const UNIT_LIST_ZINDEX: ZIndex = ZIndex::Global(150);

pub const CURSOR_COLOR: bevy::prelude::Srgba = YELLOW_300;

pub const DEPLOYMENT_ZONE_COLOR: bevy::prelude::Srgba = GREEN_YELLOW;

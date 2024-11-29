use bevy::prelude::*;

use super::construction::StructureType;

#[derive(Event, Copy, Clone, PartialEq)]
pub struct SelectStructureTypeEvent(pub StructureType);

#[derive(Event, Copy, Clone, PartialEq)]
pub struct EndDayTurn;

use bevy::prelude::*;

use crate::screen::{
    playing::{SeasonLabel, TurnUntilLabel},
    Screen,
};

/// Number of turns in a day.
pub const TURN_PER_DAY: u32 = 10;
/// Number of days in a season.
pub const DAY_PER_SEASON: u32 = 2; // TODO: Determine a balanced number, set to low for testing.
/// Number of days in a cycle which contains all 3 seasons.
pub const DAY_PER_CYCLE: u32 = DAY_PER_SEASON * 3;

pub struct CyclePlugin;

impl Plugin for CyclePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TimeOfDay>()
            .init_resource::<Season>()
            .init_resource::<DayCycle>()
            .init_resource::<Turn>()
            .add_event::<NextSeason>()
            .add_event::<EndTurn>()
            .add_systems(OnEnter(Screen::Playing), reset_cycle)
            .add_systems(
                OnEnter(Screen::Playing),
                (season_label, turn_until_label).after(reset_cycle),
            )
            .add_systems(
                Update,
                (
                    end_turn,
                    season_label.run_if(resource_changed::<Season>),
                    turn_until_label.run_if(resource_changed::<Turn>),
                )
                    .run_if(in_state(Screen::Playing)),
            );
    }
}

fn reset_cycle(mut season: ResMut<Season>, mut turn: ResMut<Turn>) {
    *season = Season::default();
    turn.0 = 0;
}

fn end_turn(
    mut end_turn_evt: EventReader<EndTurn>,
    mut turn: ResMut<Turn>,
    day_cycle: Res<DayCycle>,
    mut next_tod: ResMut<NextState<TimeOfDay>>,
    mut season: ResMut<Season>,
) {
    if end_turn_evt.is_empty() == false {
        end_turn_evt.clear();
        turn.0 += 1;

        match turn.0 % TURN_PER_DAY >= day_cycle.day {
            true => next_tod.set(TimeOfDay::Night),
            false => next_tod.set(TimeOfDay::Day),
        }

        let day = turn.0 / TURN_PER_DAY;
        season.set_if_neq(match (day % DAY_PER_CYCLE) / DAY_PER_SEASON {
            0 => Season::Summer,
            1 => Season::Autumn,
            2 => Season::Winter,
            num => unreachable!("Season range is [0, 3) but given {num} instead!"),
        });
    }
}

fn season_label(mut q_texts: Query<&mut Text, With<SeasonLabel>>, season: Res<Season>) {
    let Ok(mut text) = q_texts.get_single_mut() else {
        return;
    };

    let section = &mut text.sections[0];
    section.value = season.label().to_string();
}

fn turn_until_label(
    mut q_texts: Query<&mut Text, With<TurnUntilLabel>>,
    turn: Res<Turn>,
    day_cycle: Res<DayCycle>,
) {
    let Ok(mut text) = q_texts.get_single_mut() else {
        return;
    };

    let turn_in_day = turn.0 % TURN_PER_DAY;
    let (turn_left, target_day) = match turn_in_day >= day_cycle.day {
        true => (TURN_PER_DAY - turn_in_day, "day"),
        false => (day_cycle.day - turn_in_day, "night"),
    };

    let section = &mut text.sections[0];
    section.value = format!("{} turn(s) until {}", turn_left, target_day);
}

/// Current turn in the day cycle.
#[derive(Resource, Debug, Copy, Clone, PartialEq, Default)]
pub struct Turn(pub u32);

#[derive(Event, Copy, Clone, PartialEq, Default)]
pub struct EndTurn;

#[derive(Resource, Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum Season {
    #[default]
    Summer,
    Autumn,
    Winter,
}

impl Season {
    pub fn label(&self) -> &'static str {
        match self {
            Season::Summer => "Summer",
            Season::Autumn => "Autumn",
            Season::Winter => "Winter",
        }
    }
}

#[derive(Event, Copy, Clone, PartialEq, Default)]
pub struct NextSeason;

#[derive(States, Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum TimeOfDay {
    #[default]
    Day,
    Night,
}

#[derive(Resource, Debug, Copy, Clone, PartialEq)]
pub struct DayCycle {
    /// Number of daylight turns in a day.
    pub day: u32,
    /// Number of night turns in a day.
    pub night: u32,
}

impl Default for DayCycle {
    fn default() -> Self {
        Self::from(Season::default())
    }
}

impl From<Season> for DayCycle {
    fn from(season: Season) -> Self {
        match season {
            Season::Summer => DayCycle { day: 6, night: 4 },
            Season::Autumn => DayCycle { day: 5, night: 5 },
            Season::Winter => DayCycle { day: 4, night: 6 },
        }
    }
}

#[derive(Resource, Debug, Copy, Clone, PartialEq)]
pub struct DaysUntilFullMoon(pub u32);

impl Default for DaysUntilFullMoon {
    fn default() -> Self {
        Self(3)
    }
}

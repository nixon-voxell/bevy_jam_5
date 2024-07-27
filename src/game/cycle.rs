use bevy::prelude::*;

use crate::screen::{playing::GameState, Screen};

use super::{
    map::VillageMap, unit_list::PlayerUnitList, update_resource_label,
    update_resource_label_system, WatchRes,
};

/// Number of turns in a day.
pub const TURN_PER_DAY: u32 = 10;
/// Number of days in a season.
pub const DAY_PER_SEASON: u32 = 2; // TODO: Determine a balanced number, set to low for testing.
/// Number of days in a cycle which contains all 3 seasons.
pub const DAY_PER_CYCLE: u32 = DAY_PER_SEASON * 3;

const M: f32 = 0.4;
pub const CLEAR_BACKGROUND: Color = Color::srgb(0.7 * M, 0.75 * M, 0.8 * M);
pub const SUMMER_BACKGROUND: Color = Color::srgb(0.16 * M, 0.67 * M, 0.29 * M);
pub const AUTUMN_BACKGROUND: Color = Color::srgb(0.98 * M, 0.69 * M, 0.23 * M);
pub const WINTER_BACKGROUND: Color = Color::srgb(0.65 * M, 0.82 * M, 0.95 * M);

pub struct CyclePlugin;

impl Plugin for CyclePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TimeOfDay>()
            .init_resource::<Season>()
            .init_resource::<DayCycle>()
            .init_resource::<Turn>()
            .add_event::<NextSeason>()
            .add_event::<EndTurn>()
            .add_event::<EndDeployment>()
            .add_systems(OnEnter(Screen::Playing), (reset_cycle, update_background))
            .add_systems(OnExit(Screen::Playing), reset_background)
            .add_systems(
                Update,
                (
                    end_turn,
                    end_deployment,
                    update_cycle
                        .run_if(resource_changed::<Turn>)
                        .after(end_turn),
                    (
                        update_resource_label::<Season>(),
                        update_resource_label_system::<Turn>(turn_until_label.into_configs()),
                    )
                        .after(update_cycle),
                    update_background.run_if(state_changed::<TimeOfDay>),
                    update_day.run_if(resource_changed::<Turn>),
                )
                    .run_if(in_state(Screen::Playing)),
            );
    }
}

fn update_day(turn: Res<Turn>, mut next_game_state: ResMut<NextState<GameState>>) {
    if turn.0 != 0 && turn.0 % TURN_PER_DAY == 0 {
        next_game_state.set(GameState::Merchant);
    }
}

fn update_background(
    time_of_day: Res<State<TimeOfDay>>,
    season: Res<Season>,
    mut clear_color: ResMut<ClearColor>,
) {
    let color = match *season {
        Season::Summer => SUMMER_BACKGROUND,
        Season::Autumn => AUTUMN_BACKGROUND,
        Season::Winter => WINTER_BACKGROUND,
    };
    clear_color.0 = match time_of_day.get() {
        TimeOfDay::Day => color,
        TimeOfDay::Night => color.mix(&Color::BLACK, 0.5),
    };
}

fn reset_background(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = CLEAR_BACKGROUND;
}

fn reset_cycle(mut season: ResMut<Season>, mut turn: ResMut<Turn>) {
    *season = Season::default();
    turn.0 = 0;
}

fn end_turn(mut end_turn_evt: EventReader<EndTurn>, mut turn: ResMut<Turn>) {
    if end_turn_evt.is_empty() == false {
        end_turn_evt.clear();
        turn.0 += 1;
    }
}

fn end_deployment(
    mut end_deployment_evt: EventReader<EndDeployment>,
    mut gamestate: ResMut<NextState<GameState>>,
    player_unit_list: Res<PlayerUnitList>,
    village_map: Res<VillageMap>,
) {
    if !end_deployment_evt.is_empty() {
        end_deployment_evt.clear();
        for entity in player_unit_list.0.iter() {
            if village_map.object.locate(*entity).is_none() {
                println!("Undeployed still");
                return;
            }
        }
        gamestate.set(GameState::BattleTurn);
    }
}

fn update_cycle(
    turn: Res<Turn>,
    mut day_cycle: ResMut<DayCycle>,
    mut next_tod: ResMut<NextState<TimeOfDay>>,
    mut season: ResMut<Season>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    // Season
    let day = turn.0 / TURN_PER_DAY;
    season.set_if_neq(match (day % DAY_PER_CYCLE) / DAY_PER_SEASON {
        0 => Season::Summer,
        1 => Season::Autumn,
        2 => Season::Winter,
        num => unreachable!("Season range is [0, 3) but given {num} instead!"),
    });

    // Day cycle
    *day_cycle = DayCycle::from(*season);

    // Time of day
    match turn.0 % TURN_PER_DAY >= day_cycle.day {
        true => {
            next_tod.set(TimeOfDay::Night);
        }
        false => next_tod.set(TimeOfDay::Day),
    }

    if turn.0 % TURN_PER_DAY == day_cycle.day {
        game_state.set(GameState::Deployment);
    }
}

fn turn_until_label(
    mut q_texts: Query<&mut Text, With<WatchRes<Turn>>>,
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

#[derive(Event, Copy, Clone, PartialEq, Default)]
pub struct EndDeployment;

#[derive(Resource, Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum Season {
    #[default]
    Summer,
    Autumn,
    Winter,
}

impl std::fmt::Display for Season {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Season::Summer => "Summer".fmt(f),
            Season::Autumn => "Autumn".fmt(f),
            Season::Winter => "Winter".fmt(f),
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

impl DayCycle {
    fn len(&self) -> u32 {
        self.day + self.night
    }
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

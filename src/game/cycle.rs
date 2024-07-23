use bevy::prelude::*;

pub const TURN_PER_DAY: u32 = 10;
pub const DAY_PER_SEASON: u32 = 10;

pub struct CyclePlugin;

impl Plugin for CyclePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Season>()
            .init_state::<TimeOfDay>()
            .init_resource::<DayCycle>()
            .init_resource::<Turn>()
            .add_event::<NextSeason>()
            .add_event::<NextTurn>()
            .add_systems(Update, (turn_update_event, season_update_event));
    }
}

fn turn_update_event(
    mut next_turn_evt: EventReader<NextTurn>,
    mut turn: ResMut<Turn>,
    day_cycle: Res<DayCycle>,
    mut time_of_day: ResMut<NextState<TimeOfDay>>,
) {
    if next_turn_evt.is_empty() == false {
        next_turn_evt.clear();
        turn.0 = (turn.0 + 1) % TURN_PER_DAY;

        if turn.0 >= day_cycle.day {
            time_of_day.set(TimeOfDay::Night);
        } else {
            time_of_day.set(TimeOfDay::Day);
        }
    }
}

fn season_update_event(
    mut next_season_evt: EventReader<NextSeason>,
    season: Res<State<Season>>,
    mut next_season: ResMut<NextState<Season>>,
    mut day_cycle: ResMut<DayCycle>,
) {
    if next_season_evt.is_empty() == false {
        next_season_evt.clear();

        let season = match season.get() {
            Season::Summer => Season::Autumn,
            Season::Autumn => Season::Winter,
            Season::Winter => Season::Summer,
        };

        next_season.set(season);
        *day_cycle = DayCycle::from(season);
    }
}

/// Current turn in the day cycle.
#[derive(Resource, Debug, Copy, Clone, PartialEq, Default)]
pub struct Turn(pub u32);

#[derive(Event, Copy, Clone, PartialEq, Default)]
pub struct NextTurn;

#[derive(States, Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
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

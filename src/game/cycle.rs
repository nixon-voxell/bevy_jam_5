use bevy::prelude::*;

pub struct CyclePlugin;

impl Plugin for CyclePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Season>()
            .init_state::<TimeOfDay>()
            .init_resource::<DayCycle>()
            .init_resource::<Turn>();
    }
}

/// Current turn in the day cycle.
#[derive(Resource, Debug, Copy, Clone, PartialEq, Default)]
pub struct Turn(pub u32);

#[derive(States, Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum Season {
    #[default]
    Summer,
    Autumn,
    Winter,
}

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

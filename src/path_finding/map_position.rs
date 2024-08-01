use bevy::prelude::*;

#[derive(Component, Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MapStep {
    #[default]
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl MapStep {
    pub const fn index(self) -> usize {
        match self {
            MapStep::North => 0,
            MapStep::NorthEast => 1,
            MapStep::East => 2,
            MapStep::SouthEast => 3,
            MapStep::South => 4,
            MapStep::SouthWest => 5,
            MapStep::West => 6,
            MapStep::NorthWest => 7,
        }
    }

    pub const ALL: [Self; 8] = [
        MapStep::North,
        MapStep::NorthEast,
        MapStep::East,
        MapStep::SouthEast,
        MapStep::South,
        MapStep::SouthWest,
        MapStep::West,
        MapStep::NorthWest,
    ];

    pub const CARDINALS: [Self; 4] = [MapStep::North, MapStep::East, MapStep::South, MapStep::West];

    pub const DIAGONALS: [Self; 4] = [
        MapStep::NorthEast,
        MapStep::SouthEast,
        MapStep::SouthWest,
        MapStep::NorthWest,
    ];

    pub const fn meridean(self) -> i32 {
        match self {
            MapStep::North => -1,
            MapStep::NorthEast => -1,
            MapStep::East => 0,
            MapStep::SouthEast => 1,
            MapStep::South => 1,
            MapStep::SouthWest => 1,
            MapStep::West => 0,
            MapStep::NorthWest => -1,
        }
    }

    pub const fn parallel(self) -> i32 {
        match self {
            MapStep::North => 0,
            MapStep::NorthEast => 1,
            MapStep::East => 1,
            MapStep::SouthEast => 1,
            MapStep::South => 0,
            MapStep::SouthWest => -1,
            MapStep::West => -1,
            MapStep::NorthWest => -1,
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            MapStep::North => MapStep::South,
            MapStep::NorthEast => MapStep::SouthWest,
            MapStep::East => MapStep::West,
            MapStep::SouthEast => MapStep::SouthWest,
            MapStep::South => MapStep::North,
            MapStep::SouthWest => MapStep::NorthEast,
            MapStep::West => MapStep::East,
            MapStep::NorthWest => MapStep::SouthEast,
        }
    }

    pub const fn from_index(index: usize) -> Self {
        MapStep::ALL[index % 8]
    }

    pub const fn turn_left_45(self) -> Self {
        let index = (self.index() + 7) % 8;
        MapStep::from_index(index)
    }

    pub const fn turn_right_45(self) -> Self {
        let index = (self.index() + 1) % 8;
        MapStep::from_index(index)
    }

    pub const fn turn_left_90(self) -> Self {
        let index = (self.index() + 6) % 8;
        MapStep::from_index(index)
    }

    pub const fn turn_right_90(self) -> Self {
        let index = (self.index() + 2) % 8;
        MapStep::from_index(index)
    }
}

#[derive(Component, Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos(i32, i32);

impl Pos {
    pub fn x(self) -> i32 {
        self.0
    }

    pub fn y(self) -> i32 {
        self.0
    }

    pub fn step(self, dir: MapStep) -> Self {
        Self(self.x() + dir.meridean(), self.y() + dir.parallel())
    }
}

#[derive(Component, Default, Clone, PartialEq, Debug, Eq)]
pub struct Path(Vec<MapStep>);

impl Path {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = MapStep> + 'a {
        self.0.iter().copied()
    }

    pub fn follow(&self, mut start: Pos) -> impl Iterator<Item = Pos> + '_ {
        self.iter().map(move |step| {
            start = start.step(step);
            start
        })
    }

    pub fn step(&mut self, step: MapStep) {
        self.0.push(step)
    }

    pub fn reverse(&self) -> Self {
        Self(self.0.iter().rev().map(|step| step.opposite()).collect())
    }
}

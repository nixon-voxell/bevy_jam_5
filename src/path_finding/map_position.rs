use std::ops::{Add, Sub};

use bevy::prelude::*;

#[derive(Component, Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TileStep {
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

impl TileStep {
    pub const fn index(self) -> usize {
        match self {
            TileStep::North => 0,
            TileStep::NorthEast => 1,
            TileStep::East => 2,
            TileStep::SouthEast => 3,
            TileStep::South => 4,
            TileStep::SouthWest => 5,
            TileStep::West => 6,
            TileStep::NorthWest => 7,
        }
    }

    pub const ALL: [Self; 8] = [
        TileStep::North,
        TileStep::NorthEast,
        TileStep::East,
        TileStep::SouthEast,
        TileStep::South,
        TileStep::SouthWest,
        TileStep::West,
        TileStep::NorthWest,
    ];

    pub const ROOK: [Self; 4] = [
        TileStep::North,
        TileStep::East,
        TileStep::South,
        TileStep::West,
    ];

    pub const DIAGONALS: [Self; 4] = [
        TileStep::NorthEast,
        TileStep::SouthEast,
        TileStep::SouthWest,
        TileStep::NorthWest,
    ];

    pub const fn meridean(self) -> i32 {
        match self {
            TileStep::North => -1,
            TileStep::NorthEast => -1,
            TileStep::East => 0,
            TileStep::SouthEast => 1,
            TileStep::South => 1,
            TileStep::SouthWest => 1,
            TileStep::West => 0,
            TileStep::NorthWest => -1,
        }
    }

    pub const fn parallel(self) -> i32 {
        match self {
            TileStep::North => 0,
            TileStep::NorthEast => 1,
            TileStep::East => 1,
            TileStep::SouthEast => 1,
            TileStep::South => 0,
            TileStep::SouthWest => -1,
            TileStep::West => -1,
            TileStep::NorthWest => -1,
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            TileStep::North => TileStep::South,
            TileStep::NorthEast => TileStep::SouthWest,
            TileStep::East => TileStep::West,
            TileStep::SouthEast => TileStep::SouthWest,
            TileStep::South => TileStep::North,
            TileStep::SouthWest => TileStep::NorthEast,
            TileStep::West => TileStep::East,
            TileStep::NorthWest => TileStep::SouthEast,
        }
    }

    pub const fn from_index(index: usize) -> Self {
        TileStep::ALL[index % 8]
    }

    pub const fn turn_left_45(self) -> Self {
        let index = (self.index() + 7) % 8;
        TileStep::from_index(index)
    }

    pub const fn turn_right_45(self) -> Self {
        let index = (self.index() + 1) % 8;
        TileStep::from_index(index)
    }

    pub const fn turn_left_90(self) -> Self {
        let index = (self.index() + 6) % 8;
        TileStep::from_index(index)
    }

    pub const fn turn_right_90(self) -> Self {
        let index = (self.index() + 2) % 8;
        TileStep::from_index(index)
    }

    pub fn repeat(self, steps: u32) -> Path {
        Path(vec![self; steps as usize])
    }
}

#[derive(Component, Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Tile(pub i32, pub i32);

impl Tile {
    pub const ZERO: Self = Self(0, 0);

    pub const fn x(self) -> i32 {
        self.0
    }

    pub const fn y(self) -> i32 {
        self.1
    }

    pub fn step(self, dir: TileStep) -> Self {
        Self(self.x() + dir.parallel(), self.y() + dir.meridean())
    }

    pub const fn splat(value: i32) -> Self {
        Self(value, value)
    }

    const MIN_COORD: i32 = i32::MIN;
    const MAX_COORD: i32 = i32::MAX;

    pub const MIN: Self = Self::splat(Self::MIN_COORD);
    pub const MAX: Self = Self::splat(Self::MAX_COORD);

    pub fn min(self, other: Self) -> Self {
        Self(self.x().min(other.x()), self.y().min(other.y()))
    }

    pub fn max(self, other: Self) -> Self {
        Self(self.x().max(other.x()), self.y().max(other.y()))
    }

    pub const fn to_ivec2(self) -> IVec2 {
        IVec2::new(self.x(), self.y())
    }

    pub fn difference(self, other: Tile) -> TileDim {
        TileDim(other.x() - self.x(), other.y() - self.y())
    }

    pub fn distance_rook(self, other: Tile) -> i32 {
        let d = self.difference(other).abs();
        d.x() + d.y()
    }

    pub fn distance_squared(self, other: Tile) -> i32 {
        let d = self.difference(other);
        d.x().pow(2) + d.y().pow(2)
    }

    pub fn distance_straight(self, other: Tile) -> f32 {
        (self.distance_squared(other) as f32).sqrt()
    }
}

#[derive(Component, Default, Clone, PartialEq, Debug, Eq, Hash)]
pub struct Path(Vec<TileStep>);

impl Path {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = TileStep> + 'a {
        self.0.iter().copied()
    }

    pub fn follow(&self, mut start: Tile) -> impl Iterator<Item = Tile> + '_ {
        self.iter().map(move |step| {
            start = start.step(step);
            start
        })
    }

    pub fn step(&mut self, step: TileStep) {
        self.0.push(step)
    }

    pub fn reverse(&self) -> Self {
        Self(self.0.iter().rev().map(|step| step.opposite()).collect())
    }

    pub fn extend(&mut self, path: &Self) {
        self.0.extend(path.0.iter())
    }

    pub fn join(&self, path: &Self) -> Self {
        Self(self.iter().chain(path.iter()).collect())
    }
}

#[derive(Component, Default, Copy, Clone, PartialEq, Debug, Eq, Hash)]
pub struct TileDim(pub i32, pub i32);

impl TileDim {
    pub const ZERO: Self = Self(0, 0);
    pub const ONE: Self = Self(1, 1);

    pub const fn x(self) -> i32 {
        self.0
    }

    pub const fn y(self) -> i32 {
        self.1
    }

    pub const fn abs(self) -> Self {
        TileDim(self.x().abs(), self.y().abs())
    }

    pub const fn to_ivec2(self) -> IVec2 {
        IVec2::new(self.x(), self.y())
    }

    pub const fn splat(value: i32) -> Self {
        Self(value, value)
    }
}

impl From<IVec2> for Tile {
    fn from(value: IVec2) -> Self {
        Tile(value.x, value.y)
    }
}

impl From<IVec2> for TileDim {
    fn from(value: IVec2) -> Self {
        TileDim(value.x, value.y)
    }
}

impl Add<TileDim> for Tile {
    type Output = Self;

    fn add(self, rhs: TileDim) -> Self::Output {
        (self.to_ivec2() + rhs.to_ivec2()).into()
    }
}

impl Sub<TileDim> for Tile {
    type Output = Self;

    fn sub(self, rhs: TileDim) -> Self::Output {
        (self.to_ivec2() - rhs.to_ivec2()).into()
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub struct TileRect(pub Tile, pub Tile);

impl TileRect {
    pub fn min(self) -> Tile {
        self.0.to_ivec2().min(self.1.to_ivec2()).into()
    }

    pub fn max(self) -> Tile {
        self.0.to_ivec2().max(self.1.to_ivec2()).into()
    }

    pub fn size(self) -> TileDim {
        (self.max().to_ivec2() - self.min().to_ivec2() + IVec2::ONE).into()
    }

    pub fn area(self) -> i32 {
        let size = self.size();
        size.x() * size.y()
    }

    pub fn contains(self, tile: Tile) -> bool {
        (self.min().x()..=self.max().x()).contains(&tile.x())
            && (self.min().y()..=self.max().y()).contains(&tile.y())
    }
}

pub struct TileRectIter {
    current_x: i32,
    current_y: i32,
    min_x: i32,
    max_x: i32,
    max_y: i32,
}

impl Iterator for TileRectIter {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.max_y < self.current_y {
            return None;
        }
        let tile = Tile(self.current_x, self.current_y);
        self.current_x += 1;
        if self.max_x < self.current_x {
            self.current_x = self.min_x;
            self.current_y += 1;
        }
        Some(tile)
    }
}

impl IntoIterator for TileRect {
    type Item = Tile;
    type IntoIter = TileRectIter;

    fn into_iter(self) -> Self::IntoIter {
        let min = self.min();
        let max = self.max();
        TileRectIter {
            current_x: min.x(),
            current_y: min.y(),
            min_x: min.x(),
            max_x: max.x(),
            max_y: max.y(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_step_index() {
        assert_eq!(TileStep::North.index(), 0);
        assert_eq!(TileStep::East.index(), 2);
        assert_eq!(TileStep::SouthWest.index(), 5);
    }

    #[test]
    fn test_tile_step_turns() {
        assert_eq!(TileStep::North.turn_left_45(), TileStep::NorthWest);
        assert_eq!(TileStep::North.turn_right_45(), TileStep::NorthEast);
        assert_eq!(TileStep::North.turn_left_90(), TileStep::West);
        assert_eq!(TileStep::North.turn_right_90(), TileStep::East);
    }

    #[test]
    fn test_tile_step_opposite() {
        assert_eq!(TileStep::North.opposite(), TileStep::South);
        assert_eq!(TileStep::East.opposite(), TileStep::West);
        assert_eq!(TileStep::SouthWest.opposite(), TileStep::NorthEast);
    }

    #[test]
    fn test_tile_step_meridean_parallel() {
        assert_eq!(TileStep::North.meridean(), -1);
        assert_eq!(TileStep::North.parallel(), 0);
        assert_eq!(TileStep::East.meridean(), 0);
        assert_eq!(TileStep::East.parallel(), 1);
    }

    #[test]
    fn test_tile_operations() {
        let tile = Tile(1, 2);
        let step = TileStep::East;
        let new_tile = tile.step(step);
        assert_eq!(new_tile, Tile(2, 2));
    }

    #[test]
    fn test_tile_min_max() {
        let tile1 = Tile(1, 3);
        let tile2 = Tile(2, 2);
        assert_eq!(tile1.min(tile2), Tile(1, 2));
        assert_eq!(tile1.max(tile2), Tile(2, 3));
    }

    #[test]
    fn test_tile_distance() {
        let tile1 = Tile(1, 2);
        let tile2 = Tile(4, 6);
        assert_eq!(tile1.distance_rook(tile2), 7);
        assert_eq!(tile1.distance_squared(tile2), 25);
        assert_eq!(tile1.distance_straight(tile2), 5.0);
    }

    #[test]
    fn test_path_operations() {
        let mut path = Path::default();
        path.step(TileStep::North);
        path.step(TileStep::East);
        assert_eq!(path.len(), 2);

        let start_tile = Tile(0, 0);
        let tiles: Vec<Tile> = path.follow(start_tile).collect();
        assert_eq!(tiles, vec![Tile(0, -1), Tile(1, -1)]);
    }

    #[test]
    fn test_path_reverse() {
        let path = Path(vec![TileStep::North, TileStep::East]);
        let reversed_path = path.reverse();
        assert_eq!(
            reversed_path.iter().collect::<Vec<_>>(),
            vec![TileStep::West, TileStep::South]
        );
    }

    #[test]
    fn test_tile_rect_min_max() {
        let rect = TileRect(Tile(1, 2), Tile(4, 6));
        assert_eq!(rect.min(), Tile(1, 2));
        assert_eq!(rect.max(), Tile(4, 6));
    }

    #[test]
    fn test_tile_rect_area() {
        let rect = TileRect(Tile(1, 2), Tile(4, 6));
        assert_eq!(rect.area(), 20);
    }

    #[test]
    fn test_tile_rect_contains() {
        let rect = TileRect(Tile(1, 2), Tile(4, 6));
        assert!(rect.contains(Tile(2, 3)));
        assert!(!rect.contains(Tile(5, 7)));
    }

    #[test]
    fn test_tile_rect_iterator() {
        let rect = TileRect(Tile(1, 2), Tile(2, 3));
        let tiles: Vec<Tile> = rect.into_iter().collect();
        assert_eq!(tiles, vec![Tile(1, 2), Tile(2, 2), Tile(1, 3), Tile(2, 3)]);
    }
}

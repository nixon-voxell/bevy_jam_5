use std::ops::{Add, Sub};

use bevy::prelude::*;
use bevy::utils::HashSet;

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Edge {
    North,
    East,
    South,
    West,
}

impl Edge {
    pub const ALL: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    pub fn direction(&self) -> Direction {
        match self {
            Edge::North => Direction::North,
            Edge::East => Direction::East,
            Edge::South => Direction::South,
            Edge::West => Direction::West,
        }
    }
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Corner {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

impl Corner {
    pub const ALL: [Self; 4] = [
        Self::NorthEast,
        Self::SouthEast,
        Self::SouthWest,
        Self::NorthWest,
    ];

    pub fn direction(&self) -> Direction {
        match self {
            Corner::NorthEast => Direction::NorthEast,
            Corner::SouthEast => Direction::SouthEast,
            Corner::SouthWest => Direction::SouthWest,
            Corner::NorthWest => Direction::NorthWest,
        }
    }
}

impl Direction {
    pub const fn index(self) -> usize {
        match self {
            Direction::North => 0,
            Direction::NorthEast => 1,
            Direction::East => 2,
            Direction::SouthEast => 3,
            Direction::South => 4,
            Direction::SouthWest => 5,
            Direction::West => 6,
            Direction::NorthWest => 7,
        }
    }

    pub const ALL: [Self; 8] = [
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
    ];

    pub const EDGES: [Self; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    pub const CORNERS: [Self; 4] = [
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::NorthWest,
    ];

    pub const fn meridean(self) -> i32 {
        match self {
            Direction::North => -1,
            Direction::NorthEast => -1,
            Direction::East => 0,
            Direction::SouthEast => 1,
            Direction::South => 1,
            Direction::SouthWest => 1,
            Direction::West => 0,
            Direction::NorthWest => -1,
        }
    }

    pub const fn parallel(self) -> i32 {
        match self {
            Direction::North => 0,
            Direction::NorthEast => -1,
            Direction::East => -1,
            Direction::SouthEast => -1,
            Direction::South => 0,
            Direction::SouthWest => 1,
            Direction::West => 1,
            Direction::NorthWest => 1,
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::NorthEast => Direction::SouthWest,
            Direction::East => Direction::West,
            Direction::SouthEast => Direction::SouthWest,
            Direction::South => Direction::North,
            Direction::SouthWest => Direction::NorthEast,
            Direction::West => Direction::East,
            Direction::NorthWest => Direction::SouthEast,
        }
    }

    pub const fn from_index(index: usize) -> Self {
        Direction::ALL[index % 8]
    }

    pub const fn turn_left_45(self) -> Self {
        let index = (self.index() + 7) % 8;
        Direction::from_index(index)
    }

    pub const fn turn_right_45(self) -> Self {
        let index = (self.index() + 1) % 8;
        Direction::from_index(index)
    }

    pub const fn turn_left_90(self) -> Self {
        let index = (self.index() + 6) % 8;
        Direction::from_index(index)
    }

    pub const fn turn_right_90(self) -> Self {
        let index = (self.index() + 2) % 8;
        Direction::from_index(index)
    }

    pub fn repeat(self, steps: u32) -> Path {
        Path(vec![self; steps as usize])
    }

    pub fn is_edge(self) -> bool {
        matches!(
            self,
            Direction::North | Direction::East | Direction::South | Direction::West
        )
    }

    pub fn is_diagonal(self) -> bool {
        matches!(
            self,
            Direction::NorthEast
                | Direction::SouthEast
                | Direction::SouthWest
                | Direction::NorthWest
        )
    }

    pub fn is_parallel(self) -> bool {
        matches!(self, Direction::East | Direction::West)
    }

    pub fn is_meridean(self) -> bool {
        matches!(self, Direction::North | Direction::South)
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

    pub fn step(self, dir: Direction) -> Self {
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

    pub fn find_direction_edge(self, other: Tile) -> Option<Edge> {
        if self == other {
            return None;
        }

        if self.x() != other.x() && self.y() != other.y() {
            return None;
        }

        Some(if self.x() == other.x() {
            if other.y() < self.y() {
                Edge::North
            } else {
                Edge::South
            }
        } else {
            if other.x() < self.x() {
                Edge::East
            } else {
                Edge::West
            }
        })
    }

    pub fn get_line_between(mut self, other: Tile) -> Option<impl Iterator<Item = Tile>> {
        let Some(edge) = self.find_direction_edge(other) else {
            return None;
        };
        let direction = edge.direction();
        Some(std::iter::from_fn(move || {
            self = self.step(direction);
            Some(self).filter(|cursor| *cursor == other)
        }))
    }

    pub fn get_line_through(mut self, other: Tile) -> Option<impl Iterator<Item = Tile>> {
        let Some(edge) = self.find_direction_edge(other) else {
            return None;
        };
        let direction = edge.direction();
        Some(std::iter::from_fn(move || {
            self = self.step(direction);
            Some(self)
        }))
    }

    pub fn right_angle_path_x(&self, dest: Tile) -> impl Iterator<Item = Tile> {
        let diff = self.difference(dest);

        let dirs = if diff.x() <= 0 && 0 <= diff.y() {
            [Direction::East, Direction::South]
        } else if 0 <= diff.x() && diff.y() <= 0 {
            [Direction::South, Direction::West]
        } else if 0 <= diff.x() && 0 <= diff.y() {
            [Direction::West, Direction::South]
        } else {
            [Direction::South, Direction::East]
        };

        let mut current = *self;
        let mut primary = true;

        // might need to set some variables
        std::iter::from_fn(move || {
            if current == dest {
                return None;
            }

            let next_step = if primary {
                current.step(dirs[0])
            } else {
                current.step(dirs[1])
            };

            if next_step == dest
                || next_step.step(dirs[1]) == dest
                || next_step.step(dirs[0]) == dest
            {
                primary = !primary;
                current = next_step;
                Some(current)
            } else if (primary
                && next_step.distance_straight(dest) < current.distance_straight(dest))
                || (!primary
                    && current.step(dirs[1]).distance_straight(dest)
                        < current.distance_straight(dest))
            {
                current = next_step;
                Some(current)
            } else {
                primary = !primary;
                None
            }
        })
    }

    pub fn right_angle_path(self, dest: Tile, right_turn: bool) -> impl Iterator<Item = Tile> {
        let diff = self.difference(dest);

        let mut dirs = if diff.x() <= 0 && 0 <= diff.y() {
            [Direction::East, Direction::South]
        } else if 0 <= diff.x() && 0 <= diff.y() {
            [Direction::South, Direction::West]
        } else if 0 <= diff.x() && diff.y() <= 0 {
            [Direction::West, Direction::North]
        } else {
            [Direction::North, Direction::East]
        };

        if !right_turn {
            dirs.swap(0, 1);
        }

        println!("dirs = {dirs:?}");
        let mut current = self;

        std::iter::from_fn(move || {
            if current == dest {
                return None;
            }

            let next_primary = current.step(dirs[0]);
            let next_secondary = current.step(dirs[1]);

            if next_primary == dest
                || next_primary.distance_straight(dest) < current.distance_straight(dest)
            {
                current = next_primary;
            } else if next_secondary == dest
                || next_secondary.distance_straight(dest) < current.distance_straight(dest)
            {
                current = next_secondary;
            } else {
                current = next_primary;
            }

            Some(current)
        })
    }

    pub fn cycle(self, other: Tile, clockwise: bool) -> impl Iterator<Item = Tile> {
        self.right_angle_path(other, clockwise)
            .chain(other.right_angle_path(self, !clockwise))
    }

    pub fn edge_adjacent(self) -> [Self; 4] {
        Direction::EDGES.map(|edge| self.step(edge))
    }

    pub fn corner_adjacent(self) -> [Self; 4] {
        Direction::CORNERS.map(|corner| self.step(corner))
    }

    pub fn all_adjacent(self) -> [Self; 8] {
        Direction::ALL.map(|direction| self.step(direction))
    }
}

impl From<Vec2> for Tile {
    fn from(value: Vec2) -> Self {
        Tile(value.x.round() as i32, value.y.round() as i32)
    }
}

#[derive(Component, Default, Clone, PartialEq, Debug, Eq, Hash)]
pub struct Path(Vec<Direction>);

impl Path {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Direction> + 'a {
        self.0.iter().copied()
    }

    pub fn follow(&self, mut start: Tile) -> impl Iterator<Item = Tile> + '_ {
        self.iter().map(move |step| {
            start = start.step(step);
            start
        })
    }

    pub fn step(&mut self, step: Direction) {
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

    pub const fn easterly(self) -> bool {
        self.0 < 0
    }

    pub const fn westerly(self) -> bool {
        0 < self.0
    }

    pub const fn northerly(self) -> bool {
        self.1 < 0
    }

    pub const fn southerly(self) -> bool {
        0 < self.1
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

pub trait Tiled {
    fn contains_tile(&self, tile: Tile) -> bool;
    fn find_perimeter(&self, directions: &[Direction]) -> impl Iterator<Item = Tile>;
}

impl Tiled for TileRect {
    fn contains_tile(&self, tile: Tile) -> bool {
        self.contains(tile)
    }

    fn find_perimeter(&self, directions: &[Direction]) -> impl Iterator<Item = Tile> {
        self.into_iter().filter(move |tile| {
            directions
                .iter()
                .all(|direction| self.contains(tile.step(*direction)))
        })
    }
}

impl Tiled for HashSet<Tile> {
    fn contains_tile(&self, tile: Tile) -> bool {
        self.contains(&tile)
    }

    fn find_perimeter(&self, directions: &[Direction]) -> impl Iterator<Item = Tile> {
        self.into_iter()
            .filter(move |tile| {
                directions
                    .iter()
                    .all(|direction| self.contains(&tile.step(*direction)))
            })
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_step_index() {
        assert_eq!(Direction::North.index(), 0);
        assert_eq!(Direction::East.index(), 2);
        assert_eq!(Direction::SouthWest.index(), 5);
    }

    #[test]
    fn test_tile_step_turns() {
        assert_eq!(Direction::North.turn_left_45(), Direction::NorthWest);
        assert_eq!(Direction::North.turn_right_45(), Direction::NorthEast);
        assert_eq!(Direction::North.turn_left_90(), Direction::West);
        assert_eq!(Direction::North.turn_right_90(), Direction::East);
    }

    #[test]
    fn test_tile_step_opposite() {
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::East.opposite(), Direction::West);
        assert_eq!(Direction::SouthWest.opposite(), Direction::NorthEast);
    }

    #[test]
    fn test_tile_step_meridean_parallel() {
        assert_eq!(Direction::North.meridean(), -1);
        assert_eq!(Direction::North.parallel(), 0);
        assert_eq!(Direction::East.meridean(), 0);
        assert_eq!(Direction::East.parallel(), 1);
    }

    #[test]
    fn test_tile_operations() {
        let tile = Tile(1, 2);
        let step = Direction::East;
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
        path.step(Direction::North);
        path.step(Direction::East);
        assert_eq!(path.len(), 2);

        let start_tile = Tile(0, 0);
        let tiles: Vec<Tile> = path.follow(start_tile).collect();
        assert_eq!(tiles, vec![Tile(0, -1), Tile(1, -1)]);
    }

    #[test]
    fn test_path_reverse() {
        let path = Path(vec![Direction::North, Direction::East]);
        let reversed_path = path.reverse();
        assert_eq!(
            reversed_path.iter().collect::<Vec<_>>(),
            vec![Direction::West, Direction::South]
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

    #[test]
    fn test_right_angle_path_trivial() {
        let t = Tile::ZERO;
        let p: Vec<Tile> = t.right_angle_path(t, true).collect();
        assert!(p.is_empty());
    }

    #[test]
    fn test_right_angle_path_north() {
        let t = Tile::ZERO;
        let d = Tile(0, -3);
        let p: Vec<Tile> = t.right_angle_path(d, true).collect();
        assert_eq!(p, vec![Tile(0, -1), Tile(0, -2), Tile(0, -3)]);
    }

    #[test]
    fn test_right_angle_path_south_west() {
        let t = Tile::ZERO;
        let d = Tile(2, 2);
        let p: Vec<Tile> = t.right_angle_path(d, true).collect();
        assert_eq!(p, vec![Tile(0, 1), Tile(0, 2), Tile(1, 2), Tile(2, 2)]);
    }

    #[test]
    fn test_right_angle_path_south_east() {
        let t = Tile::ZERO;
        let d = Tile(-2, 2);
        let p: Vec<Tile> = t.right_angle_path(d, true).collect();
        assert_eq!(p, vec![Tile(-1, 0), Tile(-2, 0), Tile(-2, 1), Tile(-2, 2)]);
    }

    #[test]
    fn test_right_angle_11_02() {
        let t = Tile(1, 1);
        let d = Tile(0, 2);
        let p: Vec<Tile> = t.right_angle_path(d, true).collect();
        assert_eq!(p, vec![Tile(0, 1), Tile(0, 2)]);
    }

    #[test]
    fn test_right_angle_11_00() {
        let t = Tile(1, 1);
        let d = Tile(0, 0);
        let p: Vec<Tile> = t.right_angle_path(d, true).collect();
        assert_eq!(p, vec![Tile(1, 0), Tile(0, 0)]);
    }

    #[test]
    fn test_right_angle_11_00_left_turn() {
        let t = Tile(1, 1);
        let d = Tile(0, 0);
        let p: Vec<Tile> = t.right_angle_path(d, false).collect();
        assert_eq!(p, vec![Tile(0, 1), Tile(0, 0)]);
    }

    #[test]
    fn test_right_angle_00_11() {
        let t = Tile(0, 0);
        let d = Tile(1, 1);
        let p: Vec<Tile> = t.right_angle_path(d, true).collect();
        assert_eq!(p, vec![Tile(0, 1), Tile(1, 1)]);
    }

    #[test]
    fn test_right_angle_00_11_left_turn() {
        let t = Tile(0, 0);
        let d = Tile(1, 1);
        let p: Vec<Tile> = t.right_angle_path(d, false).collect();
        assert_eq!(p, vec![Tile(1, 0), Tile(1, 1)]);
    }

    #[test]
    fn test_cycle_clockwise() {
        let t = Tile(0, 0);
        let d = Tile(1, 1);
        let p: Vec<Tile> = t.cycle(d, true).collect();
        assert_eq!(p, vec![Tile(0, 1), Tile(1, 1), Tile(0, 1), Tile(0, 0)]);
    }
}

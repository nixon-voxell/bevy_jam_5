pub mod tiles;

use std::cmp::Reverse;

use bevy::utils::HashMap;
use bevy::utils::HashSet;
use priority_queue::PriorityQueue;
use tiles::Tile;

/// Find all tiles that are within a certain distance of a given tile
pub fn find_all_within_distance<N, I>(start: Tile, max_distance: u32, navigator: N) -> HashSet<Tile>
where
    N: Fn(Tile) -> I,
    I: IntoIterator<Item = (Tile, u32)>,
{
    let mut open_set: PriorityQueue<Tile, Reverse<u32>> = PriorityQueue::new();
    open_set.push(start, Reverse(0));
    let mut visited = HashSet::default();
    visited.insert(start);
    while let Some((current, current_weight)) = open_set.pop() {
        for (neighbor, weight) in (navigator)(current) {
            if !visited.contains(&neighbor) {
                let tentative_distance = current_weight.0 + weight;
                if tentative_distance <= max_distance {
                    open_set.push(neighbor, Reverse(tentative_distance));
                    visited.insert(neighbor);
                }
            }
        }
    }
    visited
}

/// Find all tiles that are within a certain distance of a given tile, all moves have the same cost
pub fn find_all_within_distance_unweighted<N, I>(
    start: Tile,
    max_distance: u32,
    navigator: N,
) -> HashSet<Tile>
where
    N: Fn(Tile) -> I,
    I: IntoIterator<Item = Tile>,
{
    find_all_within_distance(start, max_distance, |position| {
        navigator(position).into_iter().map(|target| (target, 1))
    })
}

/// Returns true if there exists at least one path from start to dest.
/// Otherwise false.
pub fn is_any_path<N, I>(start: Tile, dest: Tile, navigator: N) -> bool
where
    N: Fn(Tile) -> I,
    I: IntoIterator<Item = Tile>,
{
    if start == dest {
        return true;
    }
    let mut open = vec![start];
    let mut visited = HashSet::default();

    while let Some(current) = open.pop() {
        visited.insert(current);
        for neighbor in (navigator)(current) {
            if neighbor == dest {
                return true;
            }
            if !visited.contains(&neighbor) {
                open.push(neighbor);
            }
        }
    }
    false
}

pub fn find_all<N, I>(start: Tile, navigator: N) -> HashSet<Tile>
where
    N: Fn(Tile) -> I,
    I: IntoIterator<Item = Tile>,
{
    let mut open = vec![start];
    let mut visited = HashSet::default();

    while let Some(current) = open.pop() {
        visited.insert(current);
        for neighbor in (navigator)(current) {
            if !visited.contains(&neighbor) {
                open.push(neighbor);
            }
        }
    }
    visited
}

pub fn distance_map<N, I>(sources: impl Iterator<Item = Tile>, navigator: N) -> HashMap<Tile, u32>
where
    N: Fn(Tile) -> I,
    I: IntoIterator<Item = Tile>,
{
    let mut open_set: PriorityQueue<Tile, Reverse<u32>> = PriorityQueue::new();
    let mut visited: HashMap<Tile, u32> = sources.map(|tile| (tile, 0)).collect();
    open_set.extend(visited.keys().map(|tile| (*tile, Reverse(0))));

    while let Some((current, current_weight)) = open_set.pop() {
        for neighbor in (navigator)(current) {
            if !visited.contains_key(&neighbor) {
                let distance = current_weight.0 + 1;
                open_set.push(neighbor, Reverse(distance));
                visited.insert(neighbor, distance);
            }
        }
    }
    visited
}

#[cfg(test)]
mod tests {
    use super::distance_map;
    use super::tiles::Tile;

    #[test]
    fn distance_map_trivial() {
        let tiles = [Tile::ZERO, Tile(0, 1)];
        let navigator = |tile: Tile| {
            tile.edge_adjacent()
                .into_iter()
                .filter(|tile| tiles.contains(tile))
        };
        let d = distance_map([Tile::ZERO].into_iter(), navigator);
        assert_eq!(d[&Tile::ZERO], 0);
        assert_eq!(d[&Tile(0, 1)], 1);
    }

    #[test]
    fn distance_map_five_tiles() {
        let tiles = [Tile::ZERO, Tile(0, 1), Tile(1, 0), Tile(1, 1), Tile(1, 2)];
        let navigator = |tile: Tile| {
            tile.edge_adjacent()
                .into_iter()
                .filter(|tile| tiles.contains(tile))
        };
        let d = distance_map([Tile::ZERO].into_iter(), navigator);
        assert_eq!(d[&Tile::ZERO], 0);
        assert_eq!(d[&Tile(0, 1)], 1);
        assert_eq!(d[&Tile(1, 0)], 1);
        assert_eq!(d[&Tile(1, 1)], 2);
        assert_eq!(d[&Tile(1, 2)], 3);
    }

    #[test]
    fn distance_map_two_sources() {
        let tiles = [Tile::ZERO, Tile(0, 1), Tile(1, 0), Tile(1, 1), Tile(1, 2)];
        let navigator = |tile: Tile| {
            tile.edge_adjacent()
                .into_iter()
                .filter(|tile| tiles.contains(tile))
        };
        let d = distance_map([Tile::ZERO, Tile(0, 1)].into_iter(), navigator);
        assert_eq!(d[&Tile::ZERO], 0);
        assert_eq!(d[&Tile(0, 1)], 0);
        assert_eq!(d[&Tile(1, 0)], 1);
        assert_eq!(d[&Tile(1, 1)], 1);
        assert_eq!(d[&Tile(1, 2)], 2);
    }
}

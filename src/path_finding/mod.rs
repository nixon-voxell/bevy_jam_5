pub mod map_position;

use std::cmp::Reverse;

use bevy::utils::HashSet;
use map_position::Tile;
use priority_queue::PriorityQueue;

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

pub mod map_position;

use std::cmp::Reverse;

use bevy::math::IVec2;
use bevy::utils::HashSet;
use priority_queue::PriorityQueue;

/// Find all tiles that are within a certain distance of a given tile
pub fn find_all_within_distance<N, I>(
    start: IVec2,
    max_distance: u32,
    navigator: N,
) -> HashSet<IVec2>
where
    N: Fn(IVec2) -> I,
    I: IntoIterator<Item = (IVec2, u32)>,
{
    let mut open_set: PriorityQueue<IVec2, Reverse<u32>> = PriorityQueue::new();
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
    start: IVec2,
    max_distance: u32,
    navigator: N,
) -> HashSet<IVec2>
where
    N: Fn(IVec2) -> I,
    I: IntoIterator<Item = IVec2>,
{
    find_all_within_distance(start, max_distance, |position| {
        navigator(position).into_iter().map(|target| (target, 1))
    })
}

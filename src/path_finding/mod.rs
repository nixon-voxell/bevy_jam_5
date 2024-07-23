use std::cmp::Reverse;

use bevy::ecs::entity::EntityHashSet;
use bevy::prelude::Entity;
use priority_queue::PriorityQueue;


// find all tiles that are within a certain distance of a given tile
pub fn find_all_within_distance<N, I>(start: Entity, max_distance: u64, mut navigator: N) -> EntityHashSet
where 
    N: FnMut(Entity) -> I,
    I: IntoIterator<Item = (Entity, u64)>,
{
    let mut open_set: PriorityQueue<Entity, Reverse<u64>> = PriorityQueue::new();
    open_set.push(start, Reverse(0));
    let mut visited = EntityHashSet::default();
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

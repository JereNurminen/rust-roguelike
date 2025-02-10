use crate::{
    domain::entity::{types::EntityWithoutId, Entity},
    prefabs,
};

pub fn get_level() -> Vec<EntityWithoutId> {
    let left_wall: Vec<EntityWithoutId> = (-4..4)
        .map(|y| {
            prefabs::dungeon_environment::create_stone_wall(Some(
                crate::domain::world_position::WorldPosition { x: -4, y },
            ))
        })
        .collect();
    let right_wall: Vec<EntityWithoutId> = (-4..5)
        .map(|y| {
            prefabs::dungeon_environment::create_stone_wall(Some(
                crate::domain::world_position::WorldPosition { x: 8, y },
            ))
        })
        .collect();
    let up_wall: Vec<EntityWithoutId> = (-4..8)
        .map(|x| {
            prefabs::dungeon_environment::create_stone_wall(Some(
                crate::domain::world_position::WorldPosition { x, y: 4 },
            ))
        })
        .collect();
    let down_wall: Vec<EntityWithoutId> = (-4..8)
        .map(|x| {
            prefabs::dungeon_environment::create_stone_wall(Some(
                crate::domain::world_position::WorldPosition { x, y: -4 },
            ))
        })
        .collect();
    let middle_wall: Vec<EntityWithoutId> = (-1..5)
        .map(|y| {
            prefabs::dungeon_environment::create_stone_wall(Some(
                crate::domain::world_position::WorldPosition { x: 3, y },
            ))
        })
        .collect();
    [left_wall, right_wall, up_wall, down_wall, middle_wall].concat()
}

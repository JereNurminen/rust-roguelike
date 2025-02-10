// src/application/events.rs
use crate::{
    core::types::Direction,
    domain::{entity::EntityId, world::World, world_position::WorldPosition},
};

#[derive(Debug, Clone)]
pub enum GameEvent {
    Move(EntityId, Direction),
    EndTurn(EntityId),
}

impl GameEvent {
    pub fn apply(self, world: &mut World) {
        match self {
            GameEvent::Move(entity_id, direction) => {
                if let Some(entity) = world.get_entity_mut(entity_id) {
                    if let Some(pos) = entity.pos() {
                        let new_pos = match direction {
                            Direction::North => WorldPosition::new(pos.x, pos.y - 1),
                            Direction::South => WorldPosition::new(pos.x, pos.y + 1),
                            Direction::East => WorldPosition::new(pos.x + 1, pos.y),
                            Direction::West => WorldPosition::new(pos.x - 1, pos.y),
                        };
                        entity.set_pos(Some(new_pos));
                    }
                }
            }
            GameEvent::EndTurn(_) => (), // Turn management handled by game loop
        }
    }
}

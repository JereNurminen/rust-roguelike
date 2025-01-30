// src/application/events.rs

use crate::{
    core::types::Direction,
    domain::{entity::EntityId, world::World, world_position::WorldPosition},
};

// Our extended GameEvent with turn-related events:
#[derive(Debug, Clone)]
pub enum GameEvent {
    Move(EntityId, Direction), // Move entity by (dx, dy)
    EndTurn(EntityId),         // The entity ended its turn
}

/// Process an event directly, possibly spawning follow-up events.
/// This function is purely domain logic: how does each event affect the world?
/// It does NOT handle turn queue logic directly; that is handled in the game loop.
pub fn process_event(world: &mut World, event: &GameEvent) -> Vec<GameEvent> {
    println!("Processing event: {:?}", event);

    match event {
        GameEvent::Move(entity_id, direction) => {
            if let Some(entity) = world.get_entity_mut(*entity_id) {
                if let Some(pos) = entity.pos() {
                    match direction {
                        Direction::North => {
                            entity.set_pos(Some(WorldPosition::new(pos.x, pos.y - 1)))
                        }
                        Direction::South => {
                            entity.set_pos(Some(WorldPosition::new(pos.x, pos.y + 1)))
                        }
                        Direction::East => {
                            entity.set_pos(Some(WorldPosition::new(pos.x + 1, pos.y)))
                        }
                        Direction::West => {
                            entity.set_pos(Some(WorldPosition::new(pos.x - 1, pos.y)))
                        }
                    }
                    println!("Entity {:?} now located at {:?}", entity_id, pos);
                }
            }
            vec![]
        }
        // The turn-related events won't do anything to the World by themselves;
        // so just produce no follow-ups here. The turn logic is in GameLoop.
        GameEvent::EndTurn(_) => vec![],
    }
}

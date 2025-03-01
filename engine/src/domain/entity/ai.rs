use serde::Serialize;
use specta::Type;
use ts_rs::TS;

use super::EntityId;
use crate::{
    application::events::GameEvent,
    core::types::Direction,
    domain::{world::World, world_position::WorldPosition},
};

#[derive(Debug, Clone, Serialize, TS, Type)]
#[ts(export)]
pub struct LastSeen {
    entity: EntityId,
    position: WorldPosition,
    on_turn: usize,
}

#[derive(Debug, Clone, Serialize, TS, Type)]
#[ts(export)]
pub struct Memory {
    last_seen_positions: Vec<LastSeen>,
}

#[derive(Debug, Clone, Serialize, TS, Type)]
#[ts(export)]
pub struct Ai {
    memory: Memory,
}

impl Ai {
    pub fn new() -> Self {
        Ai {
            memory: Memory {
                last_seen_positions: vec![],
            },
        }
    }

    pub fn get_action(&self, entity_id: EntityId, _world: &World) -> GameEvent {
        // TODO actual logic
        let directions: [Direction; 4] = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        let random_direction = directions[rand::random_range(0..directions.len())];
        GameEvent::MoveByDirection(entity_id, random_direction)
    }
}

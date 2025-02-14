// src/application/events.rs
use crate::{
    core::types::Direction,
    domain::{entity::EntityId, world::World, world_position::WorldPosition},
};

#[derive(Debug, Clone, serde::Serialize)]
pub enum StateChange {
    EntityMoved {
        entity_id: EntityId,
        from: Option<WorldPosition>,
        to: Option<WorldPosition>,
    },
    TurnEnded {
        entity_id: EntityId,
    },
    TurnStarted {
        entity_id: EntityId,
    },
}

pub type StateChanges = Vec<StateChange>;

#[derive(Debug, Clone)]
pub enum GameEvent {
    MoveByDirection(EntityId, Direction),
    EndTurn(EntityId),
}

impl GameEvent {
    pub fn get_changes(self, world: &World) -> StateChanges {
        let mut changes = Vec::new();
        
        match self {
            GameEvent::MoveByDirection(entity_id, direction) => {
                let entity = world.get_entity(entity_id);
                let old_pos = entity.and_then(|e| e.pos());

                let new_pos = match (old_pos, direction) {
                    (Some(pos), Direction::North) => Some(WorldPosition::new(pos.x, pos.y - 1)),
                    (Some(pos), Direction::South) => Some(WorldPosition::new(pos.x, pos.y + 1)),
                    (Some(pos), Direction::East) => Some(WorldPosition::new(pos.x + 1, pos.y)),
                    (Some(pos), Direction::West) => Some(WorldPosition::new(pos.x - 1, pos.y)),
                    (None, _) => None,
                };

                if let Some(new_pos) = new_pos {
                    if let Some(entity) = world.get_entity(entity_id) {
                        let entities_in_pos = world.get_entities_by_pos(&new_pos);
                        if entity.can_move_to(&new_pos, entities_in_pos) {
                            changes.push(StateChange::EntityMoved {
                                entity_id,
                                from: old_pos,
                                to: Some(new_pos),
                            });
                        }
                    }
                }
            }
            GameEvent::EndTurn(entity_id) => {
                changes.push(StateChange::TurnEnded { entity_id });
            }
        }
        
        changes
    }
}

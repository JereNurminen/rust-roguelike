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
    SkipTurn,
}

impl GameEvent {}

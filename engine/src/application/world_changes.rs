use serde::Serialize;
use specta::Type;
use ts_rs::TS;

use crate::domain::{entity::EntityId, world_position::WorldPosition};

#[derive(Debug, Clone, Serialize, TS, Type)]
#[ts(export)]
pub struct WorldChange {
    pub entity_id: EntityId,
    pub change: ChangeType,
}

#[derive(Debug, Clone, Serialize, TS, Type)]
#[ts(export)]
pub enum ChangeType {
    Move(WorldPosition, WorldPosition),
}

impl WorldChange {
    pub fn new(entity_id: EntityId, change: ChangeType) -> Self {
        WorldChange { entity_id, change }
    }
}

use std::collections::{HashMap, HashSet};

use crate::core::types::TurnNumber;
use crate::world::{entity::EntityId, WorldPosition};

pub struct Memory {
    entity_memories: HashMap<EntityId, EntityMemory>,
    known_positions: HashSet<WorldPosition>,
    goals: Vec<Goal>,
}

struct EntityMemory {
    last_seen_pos: WorldPosition,
    last_seen_time: TurnNumber,
    feeling: Option<Feeling>,
}

enum FeelingType {
    Aggression,
    Fear,
    Curiosity,
}

struct Feeling {
    kind: FeelingType,
    target: EntityId,
    until: TurnNumber,
}

enum Goal {
    Attack { target: EntityId },
    Investigate { position: WorldPosition },
    FleeFromPosition { from: WorldPosition },
    FleeFromEntity { from: EntityId },
    Patrol { waypoints: Vec<WorldPosition> },
    Rest { until: TurnNumber },
}

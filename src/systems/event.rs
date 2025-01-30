use crate::{core::types::Direction, world::entity::EntityId};

#[derive(Debug, Clone)]
pub enum Event {
    Move {
        entity: EntityId,
        direction: Direction,
    },
    Attack {
        attacker: EntityId,
        target: EntityId,
    },
    Pickup {
        entity: EntityId,
        item: EntityId,
    },
    Drop {
        entity: EntityId,
        item: EntityId,
    },
    Open {
        entity: EntityId,
        door: EntityId,
    },
    Close {
        entity: EntityId,
        door: EntityId,
    },
    Wait {
        entity: EntityId,
    },
    Rest {
        entity: EntityId,
    },
}

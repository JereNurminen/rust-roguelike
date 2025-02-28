use crate::domain::{
    entity::{
        ai::Ai, types::EntityWithoutId, CoreAttributes, Entity, EntityId, EntityKind, Exhaustion,
        SpeciesKind, Stats, Status,
    },
    world_position::WorldPosition,
};

pub fn create_goblin(pos: Option<WorldPosition>) -> EntityWithoutId {
    EntityWithoutId {
        kind: EntityKind::Npc {
            species: SpeciesKind::Goblin,
        },
        pos,
        status: Status {
            health: 2,
            stamina: 2,
            mana: 0,
            exhaustion: Exhaustion::Rested,
        },
        stats: CoreAttributes {
            strength: 4,
            speed: 5,
            durability: 3,
            fortitude: 2,
            magic: 0,
        },
        visible: true,
        discovered: false,
        ai: Some(Ai::new()),
    }
}

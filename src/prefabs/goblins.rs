use crate::domain::{
    entity::{
        CoreAttributes, Entity, EntityId, EntityKind, Exhaustion, SpeciesKind, Stats, Status,
    },
    world_position::WorldPosition,
};

pub fn create_goblin(id: EntityId, pos: Option<WorldPosition>) -> Entity {
    Entity {
        id,
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
    }
}

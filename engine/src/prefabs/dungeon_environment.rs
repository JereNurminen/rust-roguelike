use crate::domain::{
    entity::{
        types::EntityWithoutId, CoreAttributes, Entity, EntityId, EntityKind, Exhaustion, Material,
        MaterialKind, SpeciesKind, Status,
    },
    world_position::WorldPosition,
};

fn get_stone_material() -> Material {
    Material {
        kind: MaterialKind::Stone,
        blocks_vision: true,
        blocks_movement: true,
    }
}

pub fn create_stone_wall(pos: Option<WorldPosition>) -> EntityWithoutId {
    EntityWithoutId {
        kind: EntityKind::Wall {
            material: get_stone_material(),
        },
        pos,
        status: Status {
            health: 5,
            stamina: 0,
            mana: 0,
            exhaustion: Exhaustion::Rested,
        },
        stats: CoreAttributes {
            strength: 0,
            speed: 0,
            durability: 10,
            fortitude: 0,
            magic: 0,
        },
        visible: true,
        discovered: false,
        ai: None,
    }
}

pub fn create_stone_floor(pos: Option<WorldPosition>) -> EntityWithoutId {
    EntityWithoutId {
        kind: EntityKind::Floor {
            material: get_stone_material(),
        },
        pos,
        status: Status {
            health: 5,
            stamina: 0,
            mana: 0,
            exhaustion: Exhaustion::Rested,
        },
        stats: CoreAttributes {
            strength: 0,
            speed: 0,
            durability: 10,
            fortitude: 0,
            magic: 0,
        },
        visible: true,
        discovered: false,
        ai: None,
    }
}

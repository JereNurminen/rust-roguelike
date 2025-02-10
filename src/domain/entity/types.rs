use crate::core::types::Direction;
use super::super::world_position::WorldPosition;
use super::attributes::{CoreAttributes, Status};

pub type EntityId = usize;

pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
}

#[derive(PartialEq)]
pub enum SpeciesKind {
    Human,
    Goblin,
}

#[derive(PartialEq)]
pub enum MaterialKind {
    Stone,
    Flesh,
}

#[derive(PartialEq)]
pub struct Material {
    pub kind: MaterialKind,
    pub blocks_vision: bool,
    pub blocks_movement: bool,
}

#[derive(PartialEq)]
pub enum EntityKind {
    Player,
    Npc { species: SpeciesKind },
    Item { kind: super::equipment::ItemKind },
    Wall { material: Material },
    Floor { material: Material },
}

pub struct Entity {
    pub id: EntityId,
    pub kind: EntityKind,
    pub pos: Option<WorldPosition>,
    pub stats: CoreAttributes,
    pub status: Status,
    pub visible: bool,
    pub discovered: bool,
}

impl Entity {
    pub fn new(
        id: EntityId,
        kind: EntityKind,
        pos: Option<WorldPosition>,
        stats: CoreAttributes,
        status: Status,
    ) -> Self {
        Self {
            id,
            kind,
            pos,
            visible: false,
            discovered: false,
            stats,
            status,
        }
    }

    pub fn pos(&self) -> Option<WorldPosition> {
        self.pos
    }

    pub fn set_pos(&mut self, pos: Option<WorldPosition>) {
        self.pos = pos.clone();
    }

    pub fn get_pos_in_direction(&self, dir: Direction) -> Option<WorldPosition> {
        match (self.pos, dir) {
            (Some(pos), Direction::North) => Some(WorldPosition {
                x: pos.x,
                y: pos.y - 1,
            }),
            (Some(pos), Direction::South) => Some(WorldPosition {
                x: pos.x,
                y: pos.y + 1,
            }),
            (Some(pos), Direction::East) => Some(WorldPosition {
                x: pos.x + 1,
                y: pos.y,
            }),
            (Some(pos), Direction::West) => Some(WorldPosition {
                x: pos.x - 1,
                y: pos.y,
            }),
            (None, _) => None,
        }
    }

    pub fn kind(&self) -> &EntityKind {
        &self.kind
    }
}

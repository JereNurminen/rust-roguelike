use super::super::world_position::WorldPosition;
use super::attributes::{CoreAttributes, Status};
use crate::core::types::Direction;

pub type EntityId = usize;

pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
}

#[derive(PartialEq, Clone)]
pub enum SpeciesKind {
    Human,
    Goblin,
}

#[derive(PartialEq, Clone)]
pub enum MaterialKind {
    Stone,
    Flesh,
}

#[derive(PartialEq, Clone)]
pub struct Material {
    pub kind: MaterialKind,
    pub blocks_vision: bool,
    pub blocks_movement: bool,
}

impl MaterialKind {
    pub fn get_material(&self) -> Material {
        match self {
            MaterialKind::Stone => Material {
                kind: MaterialKind::Stone,
                blocks_vision: true,
                blocks_movement: true,
            },
            MaterialKind::Flesh => Material {
                kind: MaterialKind::Flesh,
                blocks_vision: false,
                blocks_movement: false,
            },
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum EntityKind {
    Player,
    Npc { species: SpeciesKind },
    Item { kind: super::equipment::ItemKind },
    Wall { material: Material },
    Floor { material: Material },
}

#[derive(Clone)]
pub struct Entity {
    pub id: EntityId,
    pub kind: EntityKind,
    pub pos: Option<WorldPosition>,
    pub stats: CoreAttributes,
    pub status: Status,
    pub visible: bool,
    pub discovered: bool,
}

#[derive(Clone)]
pub struct EntityWithoutId {
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

    pub fn can_move_to(&self, pos: &WorldPosition, entities_in_pos: Vec<&Entity>) -> bool {
        entities_in_pos.iter().all(|e| match *e.kind() {
            EntityKind::Wall { .. } => false,
            _ => true,
        })
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

impl EntityWithoutId {
    pub fn new(
        kind: EntityKind,
        pos: Option<WorldPosition>,
        stats: CoreAttributes,
        status: Status,
    ) -> Self {
        Self {
            kind,
            pos,
            visible: false,
            discovered: false,
            stats,
            status,
        }
    }

    pub fn with_id(self, id: EntityId) -> Entity {
        Entity {
            id,
            kind: self.kind,
            pos: self.pos,
            stats: self.stats,
            status: self.status,
            visible: self.visible,
            discovered: self.discovered,
        }
    }
}

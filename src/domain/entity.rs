use crate::core::types::{DieRoll, Direction};

use super::world_position::WorldPosition;

pub type EntityId = usize;

pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
}

pub struct CoreAttributes {
    strenght: u64,
    speed: u64,
    durability: u64,
    fortitude: u64,
    magic: u64,
}

impl CoreAttributes {
    pub fn new(strenght: u64, speed: u64, durability: u64, fortitude: u64, magic: u64) -> Self {
        Self {
            strenght,
            speed,
            durability,
            fortitude,
            magic,
        }
    }
}

impl Default for CoreAttributes {
    fn default() -> Self {
        Self {
            strenght: 10,
            speed: 10,
            durability: 10,
            fortitude: 10,
            magic: 10,
        }
    }
}

pub struct Stats {
    sight_radius: u64,
    hearing_threshold: u64,
}

pub enum Exhaustion {
    WellRested,
    Rested,
    Normal,
    Tired,
    Exhausted,
}

pub struct Status {
    pub health: u64,
    pub stamina: u64,
    pub mana: u64,
    pub exhaustion: Exhaustion,
}

pub enum CreatureRaceKind {
    Human,
    Goblin,
}

pub struct StatVariance {
    low: u64,
    high: u64,
}

pub struct TemplateStat(u64, StatVariance);

pub struct CreatureTemplate {
    kind: CreatureRaceKind,
    max_health: TemplateStat,
    max_stamina: TemplateStat,
    max_mana: TemplateStat,
    stats: CoreAttributes,
}

pub struct Armor {}

pub enum HandsEquipment {
    TwoHanded(Option<EntityId>),
    OneHanded {
        left: Option<EntityId>,
        right: Option<EntityId>,
    },
}

pub struct Equipment {
    armor: Armor,
    hands: HandsEquipment,
}

pub struct Inventory {
    items: Vec<EntityId>,
}

pub struct Entity {
    pub id: EntityId,
    kind: EntityKind,
    pos: Option<WorldPosition>,
    stats: CoreAttributes,
    pub status: Status,
    visible: bool,
    discovered: bool,
    //memory: Option<Memory>,
}

#[derive(PartialEq)]
pub enum SpeciesKind {
    Human,
    Goblin,
}

#[derive(PartialEq)]
pub struct WallMaterial {
    pub blocks_vision: bool,
}

#[derive(PartialEq)]
pub struct FloorMaterial {}

#[derive(PartialEq)]
pub enum EntityKind {
    Player,
    Npc { species: SpeciesKind },
    Item { kind: ItemKind },
    Wall { material: WallMaterial },
    Floor { material: FloorMaterial },
}

#[derive(PartialEq)]
pub enum DamageType {
    Slice,
    Pierce,
    Blunt,
    Fire,
}

#[derive(PartialEq)]
pub struct Damage {
    damage_type: DamageType,
    damage: DieRoll,
}

#[derive(PartialEq)]
pub enum ItemKind {
    Weapon { damage: Vec<Damage> },
    Armor { defense: u64 },
    Potion { effect: PotionEffect },
}

#[derive(PartialEq)]
pub enum PotionEffect {
    Heal(DieRoll),
    Poison(DieRoll),
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
            //memory: None,
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
